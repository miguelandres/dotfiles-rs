// Copyright (c) 2021-2022 Miguel Barreto and others
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{
  collections::HashMap,
  convert::TryFrom,
  path::{Path, PathBuf},
};

use dotfiles_core::{
  error::{process_until_first_err, DotfilesError},
  path::convert_path_to_absolute,
  yaml_util::{fold_hash_until_first_err, map_yaml_array, read_yaml_file},
  Setting, Settings,
};
use getset::Getters;
use strict_yaml_rust::StrictYaml;

use crate::known_directive::{KnownAction, KnownDirective};

/// A context represents an environment in which defaults can be overriden, it can be thought of as
/// the context of an individual configuration file to apply.
///
/// Notice that contexts *can* be built on top of one another, so that defaults can be overriden
/// multiple times, and thus have some sort of configuration inheritance.
#[derive(Getters)]
pub struct Context {
  /// The default overrides for the current context.
  #[getset(get = "pub")]
  defaults: HashMap<String, Settings>,
  /// The list of actions parsed from this file.
  #[getset(get = "pub")]
  actions: Vec<KnownAction<'static>>,
  /// The absolute path to the file to which this context corresponds.
  #[getset(get = "pub")]
  file: PathBuf,
}

impl TryFrom<&str> for Context {
  type Error = DotfilesError;
  fn try_from(file_name: &str) -> Result<Self, Self::Error> {
    log::debug!("creating context for {:?}", file_name);
    let absolute_file = convert_path_to_absolute(&PathBuf::from(file_name), None)?;
    log::debug!("Absolute file name: {:?}", absolute_file.to_str());

    Ok(Self {
      defaults: Default::default(),
      actions: Default::default(),
      file: absolute_file,
    })
  }
}

impl TryFrom<&Path> for Context {
  type Error = DotfilesError;
  fn try_from(file_name: &Path) -> Result<Self, Self::Error> {
    log::debug!("creating context for {:?}", file_name.to_str().unwrap());

    Ok(Self {
      defaults: Default::default(),
      actions: Default::default(),
      file: convert_path_to_absolute(file_name, None)?,
    })
  }
}

impl Context {
  pub fn subcontext(&self, file: &Path) -> Result<Context, DotfilesError> {
    Ok(Context {
      defaults: self.defaults.clone(),
      actions: Default::default(),
      file: convert_path_to_absolute(file, self.file.parent())?,
    })
  }
  pub fn get_default(&self, dir: &str, setting: &str) -> Option<&Setting> {
    self
      .defaults
      .get(dir)
      .and_then(|settings| settings.get(setting))
  }

  pub fn parse_file(&mut self) -> Result<(), DotfilesError> {
    {
      let yaml = read_yaml_file(&self.file)?;
      if let Some(hash) = yaml.first().and_then(|yaml_first| yaml_first.as_hash()) {
        if let Some(yaml_defaults) = hash.get(&StrictYaml::String("defaults".into())) {
          self.defaults = Self::parse_defaults(yaml_defaults)?;
        }
        if let Some(yaml_steps) = hash.get(&StrictYaml::String("steps".into())) {
          let mut local_defaults = self.defaults.clone();
          self.actions = Self::parse_actions(&mut local_defaults, yaml_steps, &self)?;
        } else {
          log::warn!(
            "File {} does not contain any steps to parse",
            self.file.to_str().unwrap()
          );
        }

        fold_hash_until_first_err(
          yaml.first().unwrap(),
          Ok(()),
          |key, _| {
            if key == "defaults" || key == "steps" {
              Ok(())
            } else {
              Err(DotfilesError::from(
                format!("Found a  {} section which I don't know how to process", key),
                dotfiles_core::error::ErrorType::InconsistentConfigurationError,
              ))
            }
          },
          |_, _| Ok(()),
        )
      } else {
        Err(DotfilesError::from_wrong_yaml(
          "StrictYaml file root is expected to be a hash that contains defaults and steps"
            .to_owned(),
          StrictYaml::BadValue,
          StrictYaml::Hash(Default::default()),
        ))
      }
    }
    .map_err(|mut err| {
      err.add_message_prefix(format!("Parsing {}", self.file.to_str().unwrap()));
      err
    })
  }

  /// parses a list of defaults  from the passed StrictYaml configuration.
  ///
  /// It may fail for several reasons:
  ///
  /// * [ErrorType::InconsistentConfigurationError]
  ///   - In case the configuration mentions a directive that doesn't exist
  /// * [ErrorType::YamlParseError]
  ///   - A directive is mentioned more than once.
  ///   - Some other Yaml syntax error.
  /// * [ErrorType::UnexpectedYamlTypeError]:
  ///   - The StrictYaml passed to this function is not a Hash.
  ///   - The hash contains keys that are not Strings.
  ///   - The StrictYaml contains values that are not Hashes of Strings to settings
  ///   - The StrictYaml type for a particular setting does not match its expected data type.
  fn parse_defaults(yaml: &StrictYaml) -> Result<HashMap<String, Settings>, DotfilesError> {
    fold_hash_until_first_err(
      yaml,
      Ok(HashMap::default()),
      |key, yaml_val| Self::parse_directive_defaults_for_yaml(&key, yaml_val),
      |mut defaults, (dir_name, settings)| {
        defaults.insert(dir_name, settings);
        Ok(defaults)
      },
    )
  }

  fn parse_directive_defaults_for_yaml(
    directive_name: &str,
    defaults: &StrictYaml,
  ) -> Result<(String, Settings), DotfilesError> {
    let directive = KnownDirective::try_from(directive_name)?;
    directive.parse_context_defaults(defaults)
  }

  /// parses a list of actions from the passed StrictYaml configuration.
  ///
  /// It may fail for several reasons:
  ///
  /// * [ErrorType::InconsistentConfigurationError]
  ///   - In case the configuration mentions a directive that doesn't exist
  /// * [ErrorType::YamlParseError]
  ///   - A directive is mentioned more than once.
  ///   - Some other Yaml syntax error.
  /// * [ErrorType::UnexpectedYamlTypeError]:
  ///   - The StrictYaml passed to this function is not a Hash.
  ///   - The hash contains keys that are not Strings.
  ///   - The StrictYaml contains values that are not Hashes of Strings to settings
  ///   - The StrictYaml type for a particular setting does not match its expected data type.
  fn parse_actions(
    defaults: &mut HashMap<String, Settings>,
    steps_yaml: &StrictYaml,
    context: &Context,
  ) -> Result<Vec<KnownAction<'static>>, DotfilesError> {
    let all_actions: Vec<KnownAction> = map_yaml_array(steps_yaml, |step| {
      fold_hash_until_first_err(
        step,
        Ok(Vec::<KnownAction>::new()),
        |dir_name, steps_yaml: &StrictYaml| {
          let directive = KnownDirective::try_from(dir_name.as_str())?;
          let context_settings = defaults.entry(dir_name).or_default();

          KnownDirective::parse_action_list(directive, context_settings, steps_yaml, context)
        },
        |mut existing_actions, mut new_actions| {
          existing_actions.append(&mut new_actions);
          Ok(existing_actions)
        },
      )
    })?
    .into_iter()
    .flatten()
    .collect();
    Ok(all_actions)
  }

  /// Runs the actions in this context and consumes the context.
  pub fn run_actions<'a>(context: Context) -> Result<(), DotfilesError> {
    process_until_first_err(context.actions.into_iter(), |action| action.execute())
  }
}

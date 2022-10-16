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

use std::{collections::HashMap, path::PathBuf};

use dotfiles_core::{
  error::{DotfilesError, ErrorType},
  yaml_util::{process_value_from_yaml_hash, process_yaml_hash_until_first_err, read_yaml_file},
  Settings,
};
use yaml_rust::Yaml;

use crate::chosen_directive::ChosenDirective;

/// A context represents an environment in which defaults can be overriden, it can be thought of as
/// the context of an individual configuration file to apply.
///
/// Notice that contexts *can* be built on top of one another, so that defaults can be overriden
/// multiple times, and thus have some sort of configuration inheritance.
pub struct Context {
  /// The default overrides for the current context.
  defaults: HashMap<String, Settings>,
  /// The file name to which this context corresponds.
  file_name: String,
}

impl<'a> From<String> for Context {
  fn from(file_name: String) -> Self {
    Self {
      defaults: Default::default(),
      file_name,
    }
  }
}

impl<'a> Context {
  pub fn parse_file(&mut self) -> Result<(), DotfilesError> {
    {
      let path = PathBuf::from(self.file_name.clone());
      let yaml = read_yaml_file(&path)?;
      if let Some(yaml_hash) = yaml.first() {
        process_value_from_yaml_hash("defaults", yaml_hash, |defaults_yaml| {
          self.add_defaults(defaults_yaml)
        })
      } else {
        Err(DotfilesError::from_wrong_yaml(
          "Yaml file root is expected to be a hash that contains defaults and steps".to_owned(),
          Yaml::Null,
          Yaml::Hash(Default::default()),
        ))
      }
    }
    .map_err(|mut err| {
      err.add_message_prefix(format!("Parsing {}", self.file_name));
      err
    })
  }

  /// Adds defaults to the context configuration from the passed Yaml configuration.
  ///
  /// It may fail for several reasons:
  ///
  /// * [ErrorType::InconsistentConfigurationError]
  ///   - In case the configuration mentions a directive that doesn't exist
  ///   - A directiveis mentioned more than once.
  /// * [ErrorType::UnexpectedYamlTypeError]:
  ///   - The Yaml passed to this function is not a Hash.
  ///   - The hash contains keys that are not Strings.
  ///   - The Yaml contains values that are not Hashes of Strings to settings
  ///   - The Yaml type for a particular setting does not match its expected data type.
  fn add_defaults(&'a mut self, yaml: &Yaml) -> Result<(), DotfilesError> {
    process_yaml_hash_until_first_err(yaml, |key, yaml_val| {
      let (directive_name, defaults) = self.parse_directive_defaults_for_yaml(&key, yaml_val)?;
      self
        .defaults
        .try_insert(directive_name.clone(), defaults)
        .map_or(
          Err(DotfilesError::from(
            format!(
              "Defaults contain multiple configurations for the same directive {}",
              &directive_name
            ),
            ErrorType::InconsistentConfigurationError,
          )),
          |_| Ok(()),
        )
    })
  }

  fn parse_directive_defaults_for_yaml(
    &'a mut self,
    directive_name: &str,
    defaults: &Yaml,
  ) -> Result<(String, Settings), DotfilesError> {
    let directive = ChosenDirective::from(directive_name);
    directive.parse_context_defaults(defaults)
  }
}

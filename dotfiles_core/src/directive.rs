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

//! This module contains the base trait for all [Directive] and all
//! necessary conveniences to allow for user-configuration of directive
//! defaults.

extern crate yaml_rust;

use crate::{
  action::Action,
  error::{fold_until_first_err, DotfilesError, ErrorType},
  settings::{parse_setting, Settings},
  Setting,
};
use getset::Getters;
use std::collections::HashMap;
use yaml_rust::Yaml;

/// A struct that contains the default settings for a Directive and the
/// name it takes in configuration sources. The name must be unique.
///
/// These default settings can be configured by the user as well.
#[derive(Getters, Debug, Clone)]
pub struct DirectiveData {
  /// Unique name of this directive.
  ///
  /// This name will be used in configuration sources to instantiate actions
  /// of this directive
  #[getset(get = "pub")]
  name: String,
  /// Default settings for this directive.
  ///
  /// Any setting that is not in the defaults for a directive but is part of
  /// the corresponding Action struct is considered to be mandatory.
  ///
  /// Since all configurable settings have a default, this can also be used to infer the data
  /// types.
  #[getset(get = "pub")]
  defaults: Settings,
}
impl DirectiveData {
  /// Constructs a new directive from a name and a set of default settings.
  pub fn from(name: String, defaults: Settings) -> DirectiveData {
    DirectiveData {
      name: name,
      defaults,
    }
  }
}

/// A trait for all directives, it is shared between [crate::action::ActionParser] and [Directive]
pub trait HasDirectiveData<'a> {
  /// Returns the directive data for this object
  fn directive_data(&'a self) -> &'a DirectiveData;
}

/// A parser for action steps, each directive represents a type of Action.
pub trait Directive<'a>: HasDirectiveData<'a> {
  /// Returns the name of the directive.
  fn name(&'a self) -> &'a str {
    self.directive_data().name()
  }

  /// Returns the default settings as configured.
  fn defaults(&'a self) -> &'a Settings {
    self.directive_data().defaults()
  }
  /// Builds a list of actions for this directive from a Yaml configuration
  /// object and a set of default settings.
  ///
  /// Returns an Error containing a human readable string in case there
  /// was an issue building the actions.
  fn build_action_list(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Vec<Box<dyn 'a + Action<'a>>>, DotfilesError>;

  /// Parses an individual setting named `name` using the type stored in
  /// `DirectiveData.setting_types`.
  fn parse_setting(&'a self, name: &str, yaml: &Yaml) -> Result<Setting, DotfilesError> {
    if let Some(setting_type) = self.directive_data().defaults().get(name) {
      parse_setting(setting_type, yaml)
    } else {
      Err(DotfilesError::from(
        format!(
          "Directive `{}` could not parse settings, unknown setting: {}",
          self.directive_data().name(),
          name,
        ),
        ErrorType::InconsistentConfigurationError,
      ))
    }
  }

  /// Parses all settings for this directive from Yaml, checking the types correspond to what's
  /// stored in `DirectiveData.setting_types`
  fn parse_settings(&'a self, yaml_settings: &Yaml) -> Result<Settings, DotfilesError> {
    if let Yaml::Hash(hash) = yaml_settings {
      fold_until_first_err(
        hash.into_iter(),
        Ok(Settings::new()),
        |(k, v)| {
          if let Yaml::String(setting_name) = k {
            Ok((
              setting_name.to_owned(),
              self.parse_setting(setting_name, v)?,
            ))
          } else {
            Err(DotfilesError::from(
              format!(
                "Directive `{}` could not parse setting, expected a setting name but got: {:?}",
                self.directive_data().name(),
                k,
              ),
              ErrorType::UnexpectedYamlTypeError,
            ))
          }
        },
        |mut map: Settings, (name, value)| {
          map.insert(name, value);
          Ok(map)
        },
      )
    } else {
      Err(DotfilesError::from(
        format!(
          "Directive `{}` could not parse settings, expected a hash of settings, got: {:?}",
          self.directive_data().name(),
          yaml_settings,
        ),
        ErrorType::UnexpectedYamlTypeError,
      ))
    }
  }
}

/// A struct that contains the currently registered directives.
pub struct DirectiveSet<'a> {
  /// Set of directives.
  ///
  /// This is a hashmap of directive name to the actual directive object, used during parsing.
  directives: HashMap<String, Box<dyn Directive<'a> + 'a>>,
}

impl<'a> Default for DirectiveSet<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> DirectiveSet<'a> {
  /// Create a new directive set
  pub fn new() -> DirectiveSet<'a> {
    DirectiveSet {
      directives: HashMap::new(),
    }
  }

  /// Get a directive named `name`.
  pub fn get(&self, name: &str) -> Option<&Box<dyn Directive<'a> + 'a>> {
    self.directives.get(name)
  }

  /// Checks whether this directive set contains a particular directive
  pub fn has(&self, name: &str) -> bool {
    self.directives.contains_key(name)
  }

  /// Add a new directive
  ///
  /// This fails with an error if another directive with the same name already exists.
  pub fn add(&mut self, name: &str, dir: Box<dyn Directive<'a> + 'a>) -> Result<(), DotfilesError> {
    self
      .directives
      .try_insert(String::from(name), dir)
      .map_or_else(
        |_err| {
          Err(DotfilesError::from(format!(
              "Cannot add a {} directive since there is another directive with the same name already",
              name
            ),
            ErrorType::CoreError))
        },
        |_box| Ok(()),
      )
  }
}

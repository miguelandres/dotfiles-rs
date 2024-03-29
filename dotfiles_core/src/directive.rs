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

extern crate strict_yaml_rust;

use crate::{
  error::{DotfilesError, ErrorType},
  settings::{parse_setting, Settings},
  yaml_util::{fold_hash_until_first_err, get_setting_from_context},
  Setting,
};
use getset::Getters;
use strict_yaml_rust::StrictYaml;

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
    DirectiveData { name, defaults }
  }

  // Parses an individual setting named `name`'s value from a yaml containing the value, according
  // to the type set in
  /// `DirectiveData.setting_types`.
  pub fn parse_setting_value(
    &self,
    name: &str,
    yaml: &StrictYaml,
  ) -> Result<Setting, DotfilesError> {
    if let Some(setting_type) = self.defaults().get(name) {
      parse_setting(setting_type, yaml)
    } else {
      Err(DotfilesError::from(
        format!(
          "Directive `{}` could not parse settings, unknown setting: {}",
          self.name(),
          name,
        ),
        ErrorType::InconsistentConfigurationError,
      ))
    }
  }

  /// Parses all settings for this directive from StrictYaml, checking the types correspond to
  /// what's stored in `DirectiveData.setting_types`
  pub fn parse_context_defaults(
    &self,
    yaml_settings: &StrictYaml,
  ) -> Result<Settings, DotfilesError> {
    fold_hash_until_first_err(
      yaml_settings,
      Ok(Settings::new()),
      |name, value_yaml| {
        self
          .parse_setting_value(&name, value_yaml)
          .map(|value| (name, value))
      },
      |mut settings, (name, val)| {
        settings.try_insert(name.clone(), val).map_err(|_| {
          DotfilesError::from(
            format!(
              "Directive {} configuration contains duplicated setting {}",
              self.name(),
              name
            ),
            ErrorType::InconsistentConfigurationError,
          )
        })?;
        Ok(settings)
      },
    )
  }
}

/// A trait for all directives, it is shared between [crate::action::ActionParser] and [Directive]
pub trait HasDirectiveData<'a> {
  /// Returns the directive data for this object
  fn directive_data(&'a self) -> &'a DirectiveData;

  /// Returns the name of the directive.
  fn name(&'a self) -> &'a str {
    self.directive_data().name()
  }

  /// Returns the default settings as configured.
  fn defaults(&'a self) -> &'a Settings {
    self.directive_data().defaults()
  }
}

/// A parser for action steps, each directive represents a type of Action.
pub trait Directive<'a>: HasDirectiveData<'a> {
  /// Parse a particular setting with its correct type from yaml, fall back to context settings or
  /// directive defaults if not found in yaml. Returns error if there is any kind of parsing or
  /// typing error
  fn get_setting_from_yaml_hash_or_from_context(
    &'a self,
    name: &str,
    yaml: &StrictYaml,
    context_settings: &Settings,
  ) -> Result<Setting, DotfilesError> {
    self
      .get_setting_from_yaml_hash(name, yaml)
      .or_else(|_| get_setting_from_context(name, context_settings, self.defaults()))
  }

  /// Parses an individual setting named `name` from a yaml hash using the type stored in
  /// `DirectiveData.setting_types`.
  fn get_setting_from_yaml_hash(
    &'a self,
    name: &str,
    yaml: &StrictYaml,
  ) -> Result<Setting, DotfilesError> {
    if let Some(setting_type) = self.directive_data().defaults().get(name) {
      crate::yaml_util::get_setting_from_yaml_hash(name, setting_type, yaml)
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
}

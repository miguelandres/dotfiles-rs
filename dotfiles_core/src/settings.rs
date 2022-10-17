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

//! This module contains the definition of a setting and code to parse them.

extern crate strict_yaml_rust;

use std::collections::HashMap;

use strict_yaml_rust::StrictYaml;

use crate::{
  error::DotfilesError,
  yaml_util::{parse_as_boolean, parse_as_integer, parse_as_string},
};

/// The Settings object is a hashmap of option names to a default value
pub type Settings = HashMap<String, Setting>;

/// Represents a value for a setting
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Setting {
  /// A boolean value for a setting
  Boolean(bool),
  /// A string value for a setting
  String(String),
  /// An Integer value for a setting
  Integer(i64),
}

/// Returns a Settings object from an array as a bit of syntactic sugar
pub fn initialize_settings_object(settings: &[(String, Setting)]) -> Settings {
  let settings_object: Settings = settings
    .iter()
    .map(|(name, setting)| (name.clone(), setting.clone()))
    .collect();
  settings_object
}

/// Parse a setting from StrictYaml given a particular setting type.
pub fn parse_setting(setting_type: &Setting, yaml: &StrictYaml) -> Result<Setting, DotfilesError> {
  match setting_type {
    Setting::String(_) => Ok(Setting::String(parse_as_string(yaml)?)),
    Setting::Boolean(_) => Ok(Setting::Boolean(parse_as_boolean(yaml)?)),
    Setting::Integer(_) => Ok(Setting::Integer(parse_as_integer(yaml)?)),
  }
}

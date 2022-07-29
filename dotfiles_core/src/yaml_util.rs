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

//! Module that defines helper functions to process YAML configuration sources.
extern crate yaml_rust;

use crate::{
  directive::{Setting, Settings},
  error::{DotfilesError, ErrorType},
};
use yaml_rust::Yaml;

/// Gets a Boolean value from YAML or defaults.
///
/// First it tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed as a boolean.
/// - Didn't find a setting matching this name anywhere
pub fn get_boolean_setting_from_yaml_or_defaults(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<bool, DotfilesError> {
  match yaml {
    Yaml::Hash(hash) => match hash.get(&Yaml::String(String::from(name))) {
      Some(Yaml::Boolean(b)) => Ok(b.clone()),
      Some(other_yaml) => Err(DotfilesError::from(
        format!(
          "{:} exists but it is not parseable as boolean: {:}",
          name,
          other_yaml.as_str().unwrap_or("<Empty Yaml>")
        ),
        ErrorType::UnexpectedYamlTypeError,
      )),
      None => get_boolean_setting(name, context_settings, directive_defaults),
    },
    _ => get_boolean_setting(name, context_settings, directive_defaults),
  }
}

/// Gets a boolean value for the setting named `name`.
///
/// First it tries to find a value for the setting in the `context_settings`
/// argument, if it doesn't contain any it falls back to `directive-defaults`.
///
/// Returns an error if no such setting was found in either setting collection.
pub fn get_boolean_setting(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<bool, DotfilesError> {
  if let Setting::Boolean(b) = get_setting(name, context_settings, directive_defaults)? {
    Ok(b)
  } else {
    Err(DotfilesError::from(
      format!("Setting {} was found but is not boolean", name),
      ErrorType::CoreError,
    ))
  }
}

/// Gets a String value for the setting named `name`.
///
/// First it tries to find a value for the setting in the `context_settings`
/// argument, if it doesn't contain any it falls back to `directive-defaults`.
///
/// Returns an error if no such setting was found in either setting collection.
pub fn get_string_setting(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<String, DotfilesError> {
  if let Setting::String(s) = get_setting(name, context_settings, directive_defaults)? {
    Ok(s.clone())
  } else {
    Err(DotfilesError::from(
      format!("Setting {} was found but is not a string", name),
      ErrorType::CoreError,
    ))
  }
}

/// Gets a Int value for the setting named `name`.
///
/// First it tries to find a value for the setting in the `context_settings`
/// argument, if it doesn't contain any it falls back to `directive-defaults`.
///
/// Returns an error if no such setting was found in either setting collection.
pub fn get_int_setting(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<i64, DotfilesError> {
  if let Setting::Integer(x) = get_setting(name, context_settings, directive_defaults)? {
    Ok(x)
  } else {
    Err(DotfilesError::from(
      format!("Setting {} was found but is not an integer", name),
      ErrorType::CoreError,
    ))
  }
}

/// Gets a String value for the setting named `name`.
///
/// First it tries to find a value for the setting in the `context_settings`
/// argument, if it doesn't contain any it falls back to `directive-defaults`
///
/// Returns an error if no such setting was found in either setting collection.
pub fn get_setting(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<Setting, DotfilesError> {
  if let Some(setting) = context_settings.get(name) {
    Ok(setting.clone())
  } else if let Some(setting) = directive_defaults.get(name) {
    Ok(setting.clone())
  } else {
    Err(DotfilesError::from(
      format!("Setting {} couldn't be found in context or defaults", name),
      ErrorType::CoreError,
    ))
  }
}

/// Gets a Integer value from YAML or defaults.
///
/// First it tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed.
/// - Didn't find a setting matching this name anywhere
pub fn get_integer_setting_from_yaml_or_defaults(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<i64, DotfilesError> {
  match yaml {
    Yaml::Hash(hash) => match hash.get(&Yaml::String(String::from(name))) {
      Some(Yaml::Integer(i)) => Ok(i.clone()),
      Some(other_yaml) => Err(DotfilesError::from(
        format!(
          "Setting {:} exists but it cannot be parsed as integer: {:}",
          name,
          other_yaml.as_str().unwrap_or("<Empty Yaml>")
        ),
        ErrorType::UnexpectedYamlTypeError,
      )),
      None => get_int_setting(name, context_settings, directive_defaults),
    },
    _ => get_int_setting(name, context_settings, directive_defaults),
  }
}

/// Gets a String value from YAML or defaults.
///
/// First itb tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed.
/// - Didn't find a setting matching this name anywhere
pub fn get_string_setting_from_yaml_or_defaults(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<String, DotfilesError> {
  match yaml {
    Yaml::Hash(hash) => match hash.get(&Yaml::String(String::from(name))) {
      Some(Yaml::String(s)) => Ok(s.clone()),
      Some(other_yaml) => Err(DotfilesError::from(
        format!(
          "Setting {:} exists but it cannot be parsed: {:?}",
          name, other_yaml
        ),
        ErrorType::YamlParseError,
      )),
      None => get_string_setting(name, context_settings, directive_defaults),
    },
    _ => get_string_setting(name, context_settings, directive_defaults),
  }
}

/// Gets the content of this YAML node or the value for a specific key in it.
///
/// If the Yaml node passed is a String node then it returns its contents,
/// if the Yaml node is a Hash then it returns the string content of the
/// value corresponding to the optional Key.
///
/// # Errors
/// - `yaml` is neither a String nor a Hash
/// - `yaml` is a hash but it does not contain a value for `key`
/// - `yaml` is a hash but the value for `key` is not a String.
pub fn get_string_content_or_keyed_value(
  yaml: &Yaml,
  key: Option<&str>,
) -> Result<String, DotfilesError> {
  match (yaml, key) {
    (Yaml::String(s), _) => Ok(s.clone()),
    (Yaml::Hash(hash), Some(key)) => {
      match hash
        .get(&Yaml::String(String::from(key)))
        .ok_or(DotfilesError::from(
          format!("Yaml hash does not contain key {}: {:?}", key, hash),
          ErrorType::IncompleteConfigurationError {
            missing_field: key.into(),
          },
        ))? {
        Yaml::String(s) => Ok(s.clone()),
        yaml => Err(DotfilesError::from(
          format!(
            "Yaml Hash contains key {} but its value is not a string: {:?}",
            key, yaml
          ),
          ErrorType::UnexpectedYamlTypeError,
        )),
      }
    }
    (yaml, _) => Err(DotfilesError::from(
      format!("Yaml value is not a string or hash: {:?}", yaml),
      ErrorType::UnexpectedYamlTypeError,
    )),
  }
}

/// Gets a native `Vec<String>` from a Yaml::Array. It errors out if the passed yaml is not an array or if not all the items in the array are plain Yaml Strings
pub fn get_string_array(yaml: &Yaml, array_name: &str) -> Result<Vec<String>, DotfilesError> {
  match yaml {
    Yaml::Array(arr) => {
      let mut vec = Vec::<String>::with_capacity(arr.len());
      for item in arr {
        match item {
          Yaml::String(str) => vec.push(str.to_owned()),
          _ => {
            return Err(DotfilesError::from(
              format!("Not all members of the {} array are Strings", array_name),
              ErrorType::UnexpectedYamlTypeError,
            ))
          }
        }
      }
      Ok(vec)
    }
    _ => Err(DotfilesError::from(
      format!("Passed Yaml for {} is not an array", array_name),
      ErrorType::UnexpectedYamlTypeError,
    )),
  }
}

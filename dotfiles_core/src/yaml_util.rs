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

use std::path::Path;

use crate::{
  error::{fold_until_first_err, DotfilesError, ErrorType},
  settings::{parse_setting, Setting, Settings},
};
use yaml_rust::{Yaml, YamlLoader};

/// Executes the `process_function` on each of the items in the `yaml_hash`. The yaml hash is
/// assumed to be string keyed. It stops execution if any of the process functions returns an Error,
/// and returns said error.
pub fn process_yaml_hash_until_first_err<F>(
  yaml_hash: &Yaml,
  mut process_function: F,
) -> Result<(), DotfilesError>
where
  F: FnMut(String, &Yaml) -> Result<(), DotfilesError>,
{
  if let Yaml::Hash(hash) = yaml_hash {
    hash
      .into_iter()
      .map(|(key, value)| {
        parse_as_string(key)
          .map(|key_str| (key_str, value))
          .and_then(|(key, val)| process_function(key, val))
      })
      .collect()
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "Expected a yaml hash, got something else".to_owned(),
      yaml_hash.to_owned(),
      Yaml::Hash(Default::default()),
    ))
  }
}

/// Gets the value for a specified key in a yaml hash and does something with it.
///
/// Returns the result of the process function being applied to the value in question.
///
/// # Errors
///
/// * Will return an error that happens during the process_function application.
/// * Will return a [ErrorType::UnexpectedYamlTypeError] if the value is not a hash or if the hash
///   doesn't have string-based keys.
/// * Will return a [ErrorType::IncompleteConfigurationError] if the key is not found in the hash.
pub fn process_value_from_yaml_hash<T, F>(
  key: &str,
  yaml_hash: &Yaml,
  mut process: F,
) -> Result<T, DotfilesError>
where
  F: FnMut(&Yaml) -> Result<T, DotfilesError>,
{
  if let Yaml::Hash(inner_hash) = yaml_hash {
    match inner_hash.get(&Yaml::String(key.into())) {
      Some(yaml) => process(yaml),
      None => Err(DotfilesError::from(
        format!("Hash does not contain key {}", key.to_owned()),
        ErrorType::IncompleteConfigurationError {
          missing_field: key.into(),
        },
      )),
    }
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "process_value_from_yaml_hash expects a hash, but got something else".into(),
      yaml_hash.to_owned(),
      Yaml::Hash(Default::default()),
    ))
  }
}

/// Calls a processing function on all elements of an array, will fail if any of the elements fail
/// to process.
///
/// # Errors
///
/// * Any error that happens in the processing function.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml passed is not an array
pub fn map_yaml_array<T, F>(yaml_array: &Yaml, mut process: F) -> Result<Vec<T>, DotfilesError>
where
  F: FnMut(&Yaml) -> Result<T, DotfilesError>,
{
  if let Yaml::Array(inner_vec) = yaml_array {
    inner_vec.into_iter().map(|item| process(item)).collect()
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "map_yaml_array expects a yaml array, but got something else".into(),
      yaml_array.to_owned(),
      Yaml::Array(vec![]),
    ))
  }
}

/// Gets a specific setting from a yaml hash
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type does not match the `setting_type`.
/// * [ErrorType::IncompleteConfigurationError] if the hash does not contain the requested key
pub fn get_setting_from_yaml_hash(
  name: &str,
  setting_type: &Setting,
  yaml: &Yaml,
) -> Result<Setting, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| {
    parse_setting(setting_type, value_for_name)
  })
}
/// Gets a specific boolean setting from a yaml hash
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type  is not boolean.
/// * [ErrorType::IncompleteConfigurationError] if the hash does not contain the requested key
pub fn get_boolean_from_yaml_hash(name: &str, yaml: &Yaml) -> Result<bool, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| {
    parse_as_boolean(value_for_name)
  })
}

/// Gets a specific integer setting from a yaml hash
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type  is not integer.
/// * [ErrorType::IncompleteConfigurationError] if the hash does not contain the requested key
pub fn get_integer_from_yaml_hash(name: &str, yaml: &Yaml) -> Result<i64, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| {
    parse_as_integer(value_for_name)
  })
}

/// Gets a specific string setting from a yaml hash
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type  is not string.
/// * [ErrorType::IncompleteConfigurationError] if the hash does not contain the requested key
pub fn get_string_from_yaml_hash(name: &str, yaml: &Yaml) -> Result<String, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| parse_as_string(value_for_name))
}

/// Gets a specific string array setting from a yaml hash
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type  is not array, or some of its values
///   are not string.
/// * [ErrorType::IncompleteConfigurationError] if the hash does not contain the requested key
pub fn get_string_array_from_yaml_hash(
  name: &str,
  yaml: &Yaml,
) -> Result<Vec<String>, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| {
    parse_as_string_array(value_for_name)
  })
}

/// Gets a specific string array setting from a yaml hash, but if it is not found it returns an
/// empty array.
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml is not a hash.
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml does not have string keys.
/// * [ErrorType::UnexpectedYamlTypeError] if the value's type  is not array, or some of its values
///   are not string.
pub fn get_optional_string_array_from_yaml_hash(
  name: &str,
  yaml: &Yaml,
) -> Result<Vec<String>, DotfilesError> {
  get_string_array_from_yaml_hash(name, yaml).or_else(|err| {
    if err.is_missing_config(name) {
      Ok(vec![])
    } else {
      Err(err)
    }
  })
}

/// Gets a boolean value for the setting named `name`.
///
/// First it tries to find a value for the setting in the `context_settings`
/// argument, if it doesn't contain any it falls back to `directive-defaults`.
///
/// Returns an error if no such setting was found in either setting collection.
pub fn get_boolean_setting_from_context(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<bool, DotfilesError> {
  if let Setting::Boolean(b) = get_setting_from_context(name, context_settings, directive_defaults)?
  {
    Ok(b)
  } else {
    Err(DotfilesError::from(
      format!(
        "Setting {} was found in directive defaults but is not boolean",
        name
      ),
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
  if let Setting::String(s) = get_setting_from_context(name, context_settings, directive_defaults)?
  {
    Ok(s)
  } else {
    Err(DotfilesError::from(
      format!(
        "Setting {} was found in directive defaults but is not a string",
        name
      ),
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
pub fn get_integer_setting(
  name: &str,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<i64, DotfilesError> {
  if let Setting::Integer(x) = get_setting_from_context(name, context_settings, directive_defaults)?
  {
    Ok(x)
  } else {
    Err(DotfilesError::from(
      format!(
        "Setting {} was found in directive defaults but is not an integer",
        name
      ),
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
pub fn get_setting_from_context(
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

/// Gets a Boolean value from YAML or context.
///
/// First it tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed as a boolean.
/// - Didn't find a setting matching this name anywhere
pub fn get_boolean_setting_from_yaml_or_context(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<bool, DotfilesError> {
  get_boolean_from_yaml_hash(name, yaml)
    .or_else(|_| get_boolean_setting_from_context(name, context_settings, directive_defaults))
}

/// Gets a Integer value from YAML or context.
///
/// First it tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed.
/// - Didn't find a setting matching this name anywhere
pub fn get_integer_setting_from_yaml_or_context(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<i64, DotfilesError> {
  get_integer_from_yaml_hash(name, yaml)
    .or_else(|_| get_integer_setting(name, context_settings, directive_defaults))
}

/// Gets a String value from YAML or context.
///
/// First itb tries to find a value in `yaml`, if it can't find one then it
/// falls back to `context_settings` or finally `default_settings`.
///
/// # Errors
/// - Found a setting in YAML but that couldn't be parsed.
/// - Didn't find a setting matching this name anywhere
pub fn get_string_setting_from_yaml_or_context(
  name: &str,
  yaml: &Yaml,
  context_settings: &Settings,
  directive_defaults: &Settings,
) -> Result<String, DotfilesError> {
  process_value_from_yaml_hash(name, yaml, |value_for_name| parse_as_string(value_for_name))
    .or_else(|_| get_string_setting(name, context_settings, directive_defaults))
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
  parse_as_string(yaml).or_else(|err| {
    if let Some(key_str) = key {
      get_string_from_yaml_hash(key_str, yaml)
    } else {
      Err(err)
    }
  })
}

/// Gets a native `Vec<String>` from a Yaml::Array. It errors out if the passed yaml is not an array
/// or if not all the items in the array are plain Yaml Strings
pub fn parse_as_string_array(yaml: &Yaml) -> Result<Vec<String>, DotfilesError> {
  map_yaml_array(yaml, |item| parse_as_string(item))
}

/// Parse a yaml element as string, will convert booleans and integers to string if necessary.
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if yaml is neither string, boolean or integer and thus
///   cannot be parsed as string losslessly.
pub fn parse_as_string(yaml_to_parse: &Yaml) -> Result<String, DotfilesError> {
  match yaml_to_parse {
    Yaml::String(s) => Ok(s.to_owned()),
    Yaml::Boolean(b) => Ok(b.to_string()),
    Yaml::Integer(i) => Ok(i.to_string()),
    _ => Err(DotfilesError::from_wrong_yaml(
      "Expected Yaml String and got something else".into(),
      yaml_to_parse.clone(),
      Yaml::String("".into()),
    )),
  }
}

/// Parse a yaml element as boolean.
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if yaml is not of type Boolean
pub fn parse_as_boolean(yaml: &Yaml) -> Result<bool, DotfilesError> {
  if let Yaml::Boolean(b) = yaml {
    Ok(b.to_owned())
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "Expected Yaml boolean and got something else".into(),
      yaml.clone(),
      Yaml::Boolean(false),
    ))
  }
}
/// Parse a yaml element as Integer.
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if yaml is not of type Integer
pub fn parse_as_integer(yaml: &Yaml) -> Result<i64, DotfilesError> {
  if let Yaml::Integer(i) = yaml {
    Ok(i.to_owned())
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "Expected Yaml String and got something else".into(),
      yaml.clone(),
      Yaml::Integer(0),
    ))
  }
}

/// Parse a yaml element as an array.
///
/// # Errors
/// * [ErrorType::UnexpectedYamlTypeError] if yaml is not of type array
pub fn parse_as_array(yaml: &Yaml) -> Result<Vec<Yaml>, DotfilesError> {
  if let Yaml::Array(v) = yaml {
    Ok(v.to_owned())
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "Expected Yaml Array and got something else".into(),
      yaml.clone(),
      Yaml::Integer(0),
    ))
  }
}

/// Reads a Yaml File. Returns Error in case of a syntax error.
pub fn read_yaml_file(file: &Path) -> Result<Vec<Yaml>, DotfilesError> {
  let contents = std::fs::read_to_string(file).map_err(|err| DotfilesError::from_io_error(err))?;
  YamlLoader::load_from_str(&contents).map_err(|err| {
    DotfilesError::from(
      format!("yaml syntax error in file `{:?}`", file.as_os_str()),
      ErrorType::YamlParseError { scan_error: err },
    )
  })
}

/// Process each element of the hash with the `process_function` and then fold them all using into a
/// single result using `fold_function`, for an initial value of `init`. Returns the first error
/// that happens in either processing or folding.
///
/// # Errors
///
/// * [ErrorType::UnexpectedYamlTypeError] if the yaml passed in is not a hash
/// * [ErrorType::UnexpectedYamlTypeError] if the hash contains keys that cannot be parsed as
///   strings.
/// * Any errors from the fold_function or process function.
pub fn fold_hash_until_first_err<T, P, Processed, F>(
  yaml: &Yaml,
  init: Result<T, DotfilesError>,
  mut process_function: P,
  fold_function: F,
) -> Result<T, DotfilesError>
where
  P: FnMut(String, &Yaml) -> Result<Processed, DotfilesError>,
  F: FnMut(T, Processed) -> Result<T, DotfilesError>,
{
  if let Yaml::Hash(hash) = yaml {
    fold_until_first_err(
      hash.into_iter(),
      init,
      |(yaml_key, yaml_value)| process_function(parse_as_string(yaml_key)?, yaml_value),
      fold_function,
    )
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "Expected Yaml Hash, got wrong type".to_owned(),
      yaml.to_owned(),
      Yaml::Hash(Default::default()),
    ))
  }
}

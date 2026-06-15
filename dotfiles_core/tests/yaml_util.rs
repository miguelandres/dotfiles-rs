#![cfg(test)]

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

extern crate strict_yaml_rust;

use dotfiles_core::{
  error::DotfilesError, settings::initialize_settings_object, yaml_util::*, Setting, Settings,
};
use strict_yaml_rust::{StrictYaml, StrictYamlLoader};

const YAML_STRING: &str = "
- create:
    - /home/user/somedir";

const YAML_HASH: &str = "
- create:
    - dir: /home/user/somedir";
const YAML_HASH_WITHOUT_EXPECTED_KEY: &str = "
- create:
    - what: /home/user/somedir";

#[test]
fn gets_bare_string_successfully() -> Result<(), DotfilesError> {
  let docs = StrictYamlLoader::load_from_str(YAML_STRING).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, None)?
  );
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, Some("dir"))?
  );
  Ok(())
}
#[test]
fn gets_string_in_hash_successfully() -> Result<(), DotfilesError> {
  let docs = StrictYamlLoader::load_from_str(YAML_HASH).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, Some("dir"))?
  );
  Ok(())
}
#[test]
#[should_panic]
fn fails_to_get_string_in_hash_when_no_key_was_provided() {
  let docs = StrictYamlLoader::load_from_str(YAML_HASH).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  get_string_content_or_keyed_value(list, None).unwrap();
}

#[test]
#[should_panic]
fn fails_to_get_string_in_hash_when_without_correct_key() {
  let docs = StrictYamlLoader::load_from_str(YAML_HASH_WITHOUT_EXPECTED_KEY).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  get_string_content_or_keyed_value(list, None).unwrap();
}

const BOOLEAN_SETTING: &str = "boolean";
const STRING_SETTING: &str = "string";
const INT_SETTING: &str = "int";

fn default_settings() -> Settings {
  initialize_settings_object(&[
    (BOOLEAN_SETTING.to_owned(), Setting::Boolean(false)),
    (STRING_SETTING.to_owned(), Setting::String(String::new())),
    (INT_SETTING.to_owned(), Setting::Integer(0)),
  ])
}

#[test]
fn directive_fails_unknown_setting() {
  let mut map: strict_yaml_rust::strict_yaml::Hash = Default::default();
  map.insert(
    StrictYaml::from_str("unknown"),
    StrictYaml::from_str("value"),
  );
  let yaml = StrictYaml::Hash(map);
  assert!(
    parse_context_defaults("parse_test", &default_settings(), &yaml)
      .unwrap_err()
      .is_inconsistent_config()
  );
}

#[test]
fn directive_fails_parsing_setting_with_wrong_type() {
  let mut map: strict_yaml_rust::strict_yaml::Hash = Default::default();
  map.insert(
    StrictYaml::from_str(BOOLEAN_SETTING),
    StrictYaml::from_str("some"),
  );
  let yaml = StrictYaml::Hash(map);
  assert!(
    parse_context_defaults("parse_test", &default_settings(), &yaml)
      .unwrap_err()
      .is_wrong_yaml()
  );
}

#[test]
fn directive_fails_parsing_context_defaults_when_not_hash() {
  let yaml = StrictYaml::String("some".into());
  assert!(
    parse_context_defaults("parse_test", &default_settings(), &yaml)
      .unwrap_err()
      .is_wrong_yaml()
  );
}

#[test]
fn directive_fails_parsing_context_defaults_hash_not_string_keyed() {
  let mut map: strict_yaml_rust::strict_yaml::Hash = Default::default();
  map.insert(StrictYaml::BadValue, StrictYaml::from_str("1"));
  let yaml = StrictYaml::Hash(map);
  assert!(
    parse_context_defaults("parse_test", &default_settings(), &yaml)
      .unwrap_err()
      .is_wrong_yaml()
  );
}

#[test]
fn directive_succeeds_parsing_context_defaults() {
  let mut map: strict_yaml_rust::strict_yaml::Hash = Default::default();
  map.insert(
    StrictYaml::from_str(STRING_SETTING),
    StrictYaml::from_str(STRING_SETTING),
  );
  map.insert(
    StrictYaml::from_str(BOOLEAN_SETTING),
    StrictYaml::from_str("true"),
  );
  map.insert(StrictYaml::from_str(INT_SETTING), StrictYaml::from_str("1"));
  let yaml = StrictYaml::Hash(map);
  let settings = parse_context_defaults("parse_test", &default_settings(), &yaml).unwrap();

  assert_eq!(
    &Setting::String(STRING_SETTING.into()),
    settings.get(STRING_SETTING).unwrap()
  );
  assert_eq!(
    &Setting::Boolean(true),
    settings.get(BOOLEAN_SETTING).unwrap()
  );

  assert_eq!(&Setting::Integer(1), settings.get(INT_SETTING).unwrap());
}

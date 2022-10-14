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

use std::{marker::PhantomData, mem::discriminant};

use dotfiles_core::{
  directive::{DirectiveData, HasDirectiveData},
  error::ErrorType,
  settings::initialize_settings_object,
  Directive, Setting,
};
use yaml_rust::Yaml;

const DIRECTIVE_NAME: &str = "parse_test";
const BOOLEAN_SETTING: &str = "boolean";
const STRING_SETTING: &str = "string";
const INT_SETTING: &str = "int";

#[test]
fn directive_fails_unknown_setting() {
  let directive = ParseDefaultsTestDirective::new();
  let yaml = Yaml::String("some".into());
  assert_eq!(
    discriminant(&ErrorType::InconsistentConfigurationError),
    discriminant(
      directive
        .parse_setting_value("unknown", &yaml)
        .unwrap_err()
        .error_type()
    )
  );
}

#[test]
fn directive_fails_parsing_setting_with_wrong_type() {
  let directive = ParseDefaultsTestDirective::new();
  let yaml = Yaml::String("some".into());
  assert!(directive
    .parse_setting_value(BOOLEAN_SETTING, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

#[test]
fn directive_fails_parsing_context_defaults_when_not_hash() {
  let directive = ParseDefaultsTestDirective::new();
  let yaml = Yaml::String("some".into());
  assert!(directive
    .parse_context_defaults(&yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

#[test]
fn directive_fails_parsing_context_defaults_hash_not_string_keyed() {
  let directive = ParseDefaultsTestDirective::new();
  let mut map: yaml_rust::yaml::Hash = Default::default();
  map.insert(Yaml::Integer(1), Yaml::from_str("1"));
  let yaml = Yaml::Hash(map);
  assert!(directive
    .parse_context_defaults(&yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

#[test]
fn directive_succeeds_parsing_context_defaults() {
  let directive = ParseDefaultsTestDirective::new();
  let mut map: yaml_rust::yaml::Hash = Default::default();
  map.insert(
    Yaml::from_str(STRING_SETTING),
    Yaml::from_str(STRING_SETTING),
  );
  map.insert(Yaml::from_str(BOOLEAN_SETTING), Yaml::from_str("true"));
  map.insert(Yaml::from_str(INT_SETTING), Yaml::from_str("1"));
  let yaml = Yaml::Hash(map);
  let settings = directive.parse_context_defaults(&yaml).unwrap();

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

struct ParseDefaultsTestDirective<'a> {
  directive_data: DirectiveData,
  phantom_data: PhantomData<&'a DirectiveData>,
}

impl<'a> ParseDefaultsTestDirective<'a> {
  fn new() -> Self {
    ParseDefaultsTestDirective {
      directive_data: DirectiveData::from(
        DIRECTIVE_NAME.into(),
        initialize_settings_object(&[
          (BOOLEAN_SETTING.to_owned(), Setting::Boolean(false)),
          (STRING_SETTING.to_owned(), Setting::String(String::new())),
          (INT_SETTING.to_owned(), Setting::Integer(0)),
        ]),
      ),
      phantom_data: PhantomData,
    }
  }
}
impl<'a> HasDirectiveData<'a> for ParseDefaultsTestDirective<'a> {
  fn directive_data(&'a self) -> &'a DirectiveData {
    &self.directive_data
  }
}

impl<'a> Directive<'a> for ParseDefaultsTestDirective<'a> {
  fn build_action_list(
    &'a self,
    _settings: &dotfiles_core::Settings,
    _yaml: &yaml_rust::Yaml,
  ) -> Result<Vec<Box<dyn 'a + dotfiles_core::Action<'a>>>, dotfiles_core::error::DotfilesError> {
    todo!()
  }
}

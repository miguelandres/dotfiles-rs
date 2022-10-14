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

use dotfiles_core::{settings::parse_setting, Setting};
use yaml_rust::Yaml;

#[test]
fn parse_boolean_setting_succeeds() {
  let setting_type = Setting::Boolean(false);
  let yaml = Yaml::Boolean(true);
  assert_eq!(
    Setting::Boolean(true),
    parse_setting(&setting_type, &yaml).unwrap()
  );
}
#[test]
fn parse_boolean_setting_fails_with_wrong_type() {
  let setting_type = Setting::Boolean(false);
  let yaml = Yaml::String("true".to_owned());
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
  let yaml = Yaml::Integer(14);
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

#[test]
fn parse_string_setting_succeeds() {
  let setting_type = Setting::String(String::new());
  let text = "hola";
  let yaml = Yaml::String(text.to_owned());
  assert_eq!(
    Setting::String(text.to_owned()),
    parse_setting(&setting_type, &yaml).unwrap()
  );
}
#[test]
fn parse_string_setting_fails_with_wrong_type() {
  let setting_type = Setting::String(String::new());
  let yaml = Yaml::Boolean(false);
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
  let yaml = Yaml::Integer(0);
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

#[test]
fn parse_int_setting_succeeds() {
  let setting_type = Setting::Integer(0);
  let number = 14;
  let yaml = Yaml::Integer(number);
  assert_eq!(
    Setting::Integer(number),
    parse_setting(&setting_type, &yaml).unwrap()
  );
}
#[test]
fn parse_int_setting_fails_with_wrong_type() {
  let setting_type = Setting::Integer(0);
  let yaml = Yaml::Boolean(false);
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
  let yaml = Yaml::String("".to_owned());
  assert!(parse_setting(&setting_type, &yaml)
    .unwrap_err()
    .is_wrong_yaml());
}

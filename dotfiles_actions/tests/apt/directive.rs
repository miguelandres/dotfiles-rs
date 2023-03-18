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

#![cfg(test)]
use std::path::PathBuf;

use crate::utils::read_test_yaml;

use dotfiles_actions::apt::directive::AptDirective;
use dotfiles_core::{action::ActionParser, error::DotfilesError, settings::Settings};

#[test]
fn apt_directive_parsed() -> Result<(), DotfilesError> {
  let default_settings = Settings::new();
  let yaml = read_test_yaml("directive/apt/plain_functional.yaml")
    .unwrap()
    .pop()
    .unwrap();
  let directive = AptDirective::default();
  let action = directive.parse_action(&default_settings, &yaml, &PathBuf::from("/home/user"))?;
  assert_eq!(
    action.packages().to_owned(),
    Vec::from([
      "fzf",
      "zsh",
    ])
  );
  Ok(())
}

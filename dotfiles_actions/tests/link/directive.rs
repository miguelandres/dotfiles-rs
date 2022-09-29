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
use crate::utils::read_test_yaml;
use crate::utils::setup_fs;

use dotfiles_actions::link::directive::LinkDirective;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::settings::Settings;
use dotfiles_core::Action;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;

#[test]
fn link_directive_parsed_from_plain_link() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  fs.create_file("/home/user/the_file", String::from("aaa").as_bytes())
    .unwrap();
  let directive = LinkDirective::new(fs);
  let default_settings = Settings::new();
  let yaml = read_test_yaml("directive/link/plain_link.yaml")
    .unwrap()
    .pop()
    .unwrap();

  let action = directive.parse_shortened_action(&default_settings, &yaml)?;
  assert_eq!(action.path(), "a_link");
  assert_eq!(action.target(), "the_file");

  action.execute()?;
  assert!(directive.fs().is_file("a_link"));
  assert!(directive.fs().is_file("/home/user/a_link"));
  Ok(())
}

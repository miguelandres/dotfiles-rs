use std::{path::PathBuf, str::FromStr};

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
use dotfiles_core::{error::DotfilesError, Setting};
use dotfiles_processor::context::Context;

#[test]
fn context_fails_to_parse_nonexistent_file() -> Result<(), DotfilesError> {
  let mut ctx = Context::from(get_test_file("i/dont/exist.yaml"));
  assert!(ctx.parse_file().unwrap_err().is_fs_error());
  Ok(())
}

#[test]
fn context_fails_to_parse_file_with_no_root_hash() -> Result<(), DotfilesError> {
  let mut ctx = Context::from(get_test_file("context/errors/no_root_hash.yaml"));
  assert!(ctx.parse_file().unwrap_err().is_wrong_yaml());
  Ok(())
}

#[test]
fn context_fails_multiple_defaults_same_directive() -> Result<(), DotfilesError> {
  let mut ctx = Context::from(get_test_file(
    "context/errors/multiple_defaults_same_directive.yaml",
  ));
  assert!(ctx.parse_file().unwrap_err().is_yaml_parse_error());
  Ok(())
}

#[test]
fn context_fails_multiple_defaults_same_key() -> Result<(), DotfilesError> {
  let mut ctx = Context::from(get_test_file(
    "context/errors/multiple_defaults_same_key.yaml",
  ));
  assert!(ctx.parse_file().unwrap_err().is_yaml_parse_error());
  Ok(())
}

#[test]
fn context_parses_correct_defaults() -> Result<(), DotfilesError> {
  let mut ctx = Context::from(get_test_file("context/correct_defaults.yaml"));
  ctx.parse_file()?;
  assert_eq!(
    Setting::Boolean(true),
    ctx.get_default("create", "force").unwrap().clone()
  );
  assert_eq!(
    Setting::Boolean(true),
    ctx.get_default("link", "relink").unwrap().clone()
  );
  assert_eq!(
    Setting::Boolean(false),
    ctx.get_default("link", "force").unwrap().clone()
  );
  Ok(())
}

fn get_test_file(file_name: &str) -> String {
  let base_dir = env!("CARGO_MANIFEST_DIR");
  let mut file: PathBuf = PathBuf::from_str(base_dir).unwrap();
  file.push("resources/tests");
  file.push(file_name);
  file.to_str().unwrap().to_string()
}

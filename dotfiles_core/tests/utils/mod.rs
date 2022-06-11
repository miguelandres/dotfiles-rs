#![cfg(test)]
// Copyright (c) 2021-2021 Miguel Barreto and others
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

use dotfiles_core::action::Action;
use std::path::PathBuf;
use std::str::FromStr;
use yaml_rust::ScanError;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn check_action_fail<'a, A: Action<'a>>(
  action: &A,
  error_message: String,
) -> Result<(), String> {
  if let Ok(()) = action.execute() {
    Err(error_message)
  } else {
    Ok(())
  }
}

/// Reads a yaml file from test resources.
pub fn read_test_yaml(test_file_path: &str) -> Result<Vec<Yaml>, ScanError> {
  let base_dir = env!("CARGO_MANIFEST_DIR");
  let mut file: PathBuf = PathBuf::from_str(base_dir).unwrap();
  file.push("resources/tests");
  file.push(test_file_path);
  let contents = std::fs::read_to_string(&file).unwrap();
  YamlLoader::load_from_str(&contents)
}
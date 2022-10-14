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

use dotfiles_core::action::Action;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::error::ErrorType;
use dotfiles_core::yaml_util::read_yaml_file;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use yaml_rust::Yaml;

pub fn check_action_fail<'a, A: Action<'a>>(
  action: &A,
  error_message: String,
) -> Result<(), DotfilesError> {
  if let Ok(()) = action.execute() {
    Err(DotfilesError::from(
      error_message,
      ErrorType::TestingErrorActionSucceedsWhenItShouldFail,
    ))
  } else {
    Ok(())
  }
}

/// Reads a yaml file from test resources.
pub fn read_test_yaml(test_file_path: &str) -> Result<Vec<Yaml>, DotfilesError> {
  let base_dir = env!("CARGO_MANIFEST_DIR");
  let mut file: PathBuf = PathBuf::from_str(base_dir).unwrap();
  file.push("resources/tests");
  file.push(test_file_path);
  read_yaml_file(file.as_ref())
}

fn setup_fs_internal(fs: &FakeFileSystem) -> io::Result<()> {
  fs.create_dir_all("/home/user/")?;
  fs.create_dir("/system")?;
  fs.set_readonly("/system", true)?;
  fs.set_current_dir("/home/user/")?;
  Ok(())
}

/// Setups a test FakeFileSystem
pub fn setup_fs(fs: &FakeFileSystem) -> Result<(), DotfilesError> {
  if let Err(io_error) = setup_fs_internal(fs) {
    return Err(DotfilesError::from_io_error(io_error));
  }
  Ok(())
}

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

use std::env;

use dotfiles_actions::create::action::FakeCreateAction;
use dotfiles_core::{
  action::{Action, ConditionalAction},
  error::DotfilesError,
};
use filesystem::{FakeFileSystem, FileSystem};

use crate::utils::{check_action_fail, setup_fs};

#[test]
fn skip_in_ci_is_respected() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  env::set_var("DOTFILES_TESTING_ENV_VAR", "true");
  env::set_var("TESTING_ONLY_FAKE_CI", "true");
  let action = FakeCreateAction::new(&fs, true, String::from("/home/user/target"), true);
  action.check_conditions_and_execute()?;
  assert!(!fs.is_dir("/home/user/target"));

  env::remove_var("TESTING_ONLY_FAKE_CI");
  action.check_conditions_and_execute()?;
  assert!(fs.is_dir("/home/user/target"));
  env::remove_var("DOTFILES_TESTING_ENV_VAR");
  Ok(())
}

#[test]
fn create_dir_fails_on_nonexistent_path() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs).expect("Failure setting up FakeFileSystem");
  let action = FakeCreateAction::new(
    &fs,
    false,
    String::from("/home/user/nonexistent_path/target"),
    false,
  );
  check_action_fail(
    &action,
    format!(
      "Could create directory in nonexistent path, {}",
      action.directory()
    ),
  )
}

#[test]
fn create_dir_succeeds() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = FakeCreateAction::new(&fs, false, String::from("/home/user/target"), false);
  action.execute()
}

#[test]
fn create_dir_fails_on_readonly_dir() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = FakeCreateAction::new(&fs, false, String::from("/system/target"), false);
  check_action_fail(
    &action,
    format!(
      "Could create directory in readonly path {}",
      action.directory()
    ),
  )
}

#[test]
fn force_create_dir_succeeds_on_nonexistent_path() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = FakeCreateAction::new(
    &fs,
    false,
    String::from("/home/user/nonexistent_path/target"),
    true,
  );
  action.execute()
}

#[test]
fn force_create_dir_succeeds() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = FakeCreateAction::new(&fs, false, String::from("/home/user/target"), true);
  action.execute()
}

#[test]
fn force_create_dir_fails_on_readonly_dir() -> Result<(), DotfilesError> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = FakeCreateAction::new(&fs, false, String::from("/system/target"), true);
  check_action_fail(
    &action,
    format!(
      "Could create directory in readonly path {}",
      action.directory()
    ),
  )
}

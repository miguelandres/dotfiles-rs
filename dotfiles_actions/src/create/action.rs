// Copyright (c) 2021-2026 Miguel Barreto and others
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

//! This module contains the [CreateAction] that creates a new directory
//! when executed

extern crate strict_yaml_rust;

use derivative::Derivative;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::path::convert_path_to_absolute;
use dotfiles_core::path::process_home_dir_in_path;
use dotfiles_core::settings::{initialize_settings_object, Setting, Settings};
use dotfiles_core::yaml_util;
use dotfiles_core::{action::SKIP_IN_CI_SETTING, Action};
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::OsFileSystem;

use getset::Getters;
use log::info;

use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use strict_yaml_rust::StrictYaml;

/// Constant for the name of the [`create_parent_dirs`](CreateAction::create_parent_dirs) Setting
/// which forces to create all parent directories if necessary.
pub const CREATE_PARENT_DIRS_SETTING: &str = "create_parent_dirs";
/// Constant for the name of the [`directory`](CreateAction::directory) argument that contains the
/// name of the directory to create
pub const DIR_SETTING: &str = "dir";

/// Default settings for the Create action.
pub fn default_settings() -> Settings {
  initialize_settings_object(&[
    (
      CREATE_PARENT_DIRS_SETTING.to_owned(),
      Setting::Boolean(false),
    ),
    (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
  ])
}

/// [CreateAction] creates a new [directory](CreateAction::directory) when executed
#[derive(Derivative, Getters)]
#[derivative(Debug, PartialEq)]
pub struct CreateAction<F: FileSystem> {
  /// Skips this action if it is running in a CI environment.
  skip_in_ci: bool,
  /// FileSystem to use to create the directory.
  ///
  /// Having a filesystem instance here allows us to use fakes/mocks to use
  /// in unit tests.
  #[derivative(Debug = "ignore", PartialEq = "ignore")]
  fs: F,
  /// Directory to create. Can be absolute or relative.
  #[get = "pub"]
  directory: String,
  /// Force creation of the directory and all its parents if they do not
  /// exist already.
  ///
  /// Setting [`create_parent_dirs`](CreateAction::create_parent_dirs) to `true` is equivalent to
  /// using the `-p` flag in `mkdir`.
  #[get = "pub"]
  create_parent_dirs: bool,
  /// Current directory that will be used to determine relative file locations if necessary. It
  /// must match the parent directory of the configuration file that declared this action.
  #[get = "pub"]
  current_dir: PathBuf,
}

/// A native create action that works on the real filesystem.
pub type NativeCreateAction = CreateAction<OsFileSystem>;
/// A Fake create action that works on a fake test filesystem.
pub type FakeCreateAction = CreateAction<FakeFileSystem>;

impl<F: FileSystem> CreateAction<F> {
  /// Constructs a new instance of CreateAction
  pub fn new(
    fs: F,
    skip_in_ci: bool,
    directory: String,
    create_parent_dirs: bool,
    current_dir: PathBuf,
  ) -> Result<Self, DotfilesError> {
    let action = CreateAction {
      skip_in_ci,
      fs,
      directory,
      create_parent_dirs,
      current_dir,
    };
    log::trace!("Creating new {:?}", action);
    Ok(action)
  }
}

impl<F: FileSystem> Action for CreateAction<F> {
  /// Creates the [`directory`](CreateAction::directory).
  ///
  /// # Errors
  /// - The parent directory does not exist and
  ///   [`create_parent_dirs`](CreateAction::create_parent_dirs) is false.
  /// - There is already a directory, file or symlink with the same name.
  /// - Permission denied.
  fn execute(&self) -> Result<(), DotfilesError> {
    fn create_dir<F: FileSystem>(
      fs: &'_ F,
      directory: &str,
      create_parent_dirs: bool,
      current_dir: &Path,
    ) -> Result<(), DotfilesError> {
      let path = PathBuf::from(directory.to_owned());
      let path = process_home_dir_in_path(&path);
      let path = convert_path_to_absolute(&path, Some(current_dir))?;
      if create_parent_dirs {
        fs.create_dir_all(path)
      } else {
        fs.create_dir(path)
      }
      .or_else(|io_error| {
        if let ErrorKind::AlreadyExists = io_error.kind() {
          Ok(())
        } else {
          Err(DotfilesError::from_io_error(io_error))
        }
      })
    }
    create_dir(
      &self.fs,
      &self.directory,
      self.create_parent_dirs,
      &self.current_dir,
    )
    .map(|_| {
      info!("Created directory {}", &self.directory);
    })
  }

  fn skip_in_ci(&self) -> bool {
    self.skip_in_ci
  }
}

/// Static parsing function to build a CreateAction from YAML and settings context
pub fn parse_action<F: FileSystem>(
  fs: F,
  settings: &Settings,
  yaml: &StrictYaml,
  current_dir: &Path,
) -> Result<CreateAction<F>, DotfilesError> {
  let defaults = default_settings();
  CreateAction::new(
    fs,
    yaml_util::get_boolean_setting_from_yaml_or_context(
      SKIP_IN_CI_SETTING,
      yaml,
      settings,
      &defaults,
    )?,
    yaml_util::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
    yaml_util::get_boolean_setting_from_yaml_or_context(
      CREATE_PARENT_DIRS_SETTING,
      yaml,
      settings,
      &defaults,
    )?,
    current_dir.to_owned(),
  )
}

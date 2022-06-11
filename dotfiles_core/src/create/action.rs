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

//! This module contains the [CreateAction] that creates a new directory
//! when executed

extern crate yaml_rust;

use crate::action::Action;
use filesystem::FileSystem;
use log::error;
use log::info;
use std::io;

/// [CreateAction] creates a new [directory](CreateAction::directory) when executed
pub struct CreateAction<'a, F: FileSystem> {
  /// FileSystem to use to create the directory.
  ///
  /// Having a filesystem instance here allows us to use fakes/mocks to use
  /// in unit tests.
  fs: &'a F,
  /// Directory to create. Can be absolute or relative.
  pub directory: String,
  /// Force creation of the directory and all its parents if they do not
  /// exist already.
  ///
  /// Setting [`force`](CreateAction::force) to `true` is equivalent to using
  /// the `-p` flag in `mkdir`.
  pub force: bool,
}

impl<F: FileSystem> CreateAction<'_, F> {
  /// Constructs a new instance of CreateAction
  pub fn new(fs: &'_ F, directory: String, force: bool) -> CreateAction<'_, F> {
    CreateAction {
      fs,
      directory,
      force,
    }
  }
  /// Returns the directory to create.
  pub fn directory(&self) -> &str {
    &self.directory
  }
  /// Returns true if the action will create parent directories if necessary.
  ///
  /// [`force`](CreateAction::force) being `true` is equivalent to using the
  /// `-p` flag in `mkdir`
  pub fn force(&self) -> bool {
    self.force
  }
}

impl<F: FileSystem> Action<'_> for CreateAction<'_, F> {
  /// Creates the [`directory`](CreateAction::directory).
  ///
  /// # Errors
  /// - The parent directory does not exist and
  ///   [`force`](CreateAction::force) is false.
  /// - There is already a directory, file or symlink with the same name.
  /// - Permission denied.
  fn execute(&self) -> Result<(), String> {
    fn create_dir<F: FileSystem>(fs: &'_ F, directory: &str, force: bool) -> io::Result<()> {
      if force {
        Ok(fs.create_dir_all(&directory)?)
      } else {
        Ok(fs.create_dir(&directory)?)
      }
    }
    match create_dir(self.fs, &self.directory, self.force) {
      Ok(()) => {
        info!("Created directory {}", &self.directory);
        Ok(())
      }
      Err(s) => {
        error!("Couldn't create directory {}: {}", &self.directory, s);
        Err(s.to_string())
      }
    }
  }
}

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

#![cfg(unix)]
//! This module contains the [LinkAction] that creates a new symlink
//! when executed

use crate::link::directive::*;
use derivative::Derivative;
use dotfiles_core::action::Action;
use dotfiles_core::action::SKIP_IN_CI_SETTING;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::error::ErrorType;
use dotfiles_core::path::process_home_dir_in_path;
use dotfiles_core::settings::Settings;
use dotfiles_core::yaml_util::get_boolean_setting_from_context;
use dotfiles_core_macros::ConditionalAction;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use filesystem::UnixFileSystem;
use getset::CopyGetters;
use getset::Getters;
use log::error;
use log::info;
use std::format;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

/// [LinkAction] creates a new symlink `path` that points to `target`.
///
/// It is equivalent to running `ln -s <target> <path>`
#[derive(Derivative, Getters, CopyGetters, ConditionalAction)]
#[derivative(Debug, PartialEq)]
pub struct LinkAction<'a, F: FileSystem + UnixFileSystem> {
  /// Skips this action if it is running in a CI environment.
  skip_in_ci: bool,
  /// FileSystem to use to create the directory.
  ///
  /// Having a filesystem instance here allows us to use fakes/mocks to use
  /// in unit tests.
  #[derivative(Debug = "ignore", PartialEq = "ignore")]
  fs: &'a F,
  /// Path of the new symlink
  #[getset(get = "pub")]
  path: String,
  /// Path that the symlink points to.
  #[getset(get = "pub")]
  target: String,
  /// Force to re-create the symlink if it exists already
  #[getset(get_copy = "pub")]
  relink: bool,
  /// Force to replace an existing file or directory when executed.
  #[getset(get_copy = "pub")]
  force: bool,
  /// If a relative link is found, convert it to absolute.
  #[getset(get_copy = "pub")]
  convert_to_absolute: bool,
  /// Create all parent directories if they do not exist already
  #[getset(get_copy = "pub")]
  create_parent_dirs: bool,
  /// Succeed even if `target` doesn't point to an existing file or dir.
  #[getset(get_copy = "pub")]
  ignore_missing_target: bool,
  /// Allow relative linking.
  #[getset(get_copy = "pub")]
  relative: bool,
  /// If the target is another symlink, resolve the ultimate concrete file
  /// or directory that it points to and make it the target
  #[getset(get_copy = "pub")]
  resolve_symlink_target: bool,
}

/// A native create action that works on the real filesystem.
pub type NativeLinkAction<'a> = LinkAction<'a, OsFileSystem>;
/// A Fake create action that works on a fake test filesystem.
pub type FakeLinkAction<'a> = LinkAction<'a, FakeFileSystem>;

impl<'a, F: FileSystem + UnixFileSystem> LinkAction<'a, F> {
  /// Constructs a new [LinkAction]
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    fs: &'a F,
    path: String,
    target: String,
    context_settings: &'_ Settings,
    defaults: &'_ Settings,
  ) -> Self {
    let relink =
      get_boolean_setting_from_context(RELINK_SETTING, context_settings, defaults).unwrap();
    let force =
      get_boolean_setting_from_context(FORCE_SETTING, context_settings, defaults).unwrap();
    let create_parent_dirs =
      get_boolean_setting_from_context(CREATE_PARENT_DIRS_SETTING, context_settings, defaults)
        .unwrap();
    let ignore_missing_target =
      get_boolean_setting_from_context(IGNORE_MISSING_TARGET_SETTING, context_settings, defaults)
        .unwrap();
    let relative =
      get_boolean_setting_from_context(RELATIVE_SETTING, context_settings, defaults).unwrap();
    let resolve_symlink_target =
      get_boolean_setting_from_context(RESOLVE_SYMLINK_TARGET_SETTING, context_settings, defaults)
        .unwrap();
    let skip_in_ci =
      get_boolean_setting_from_context(SKIP_IN_CI_SETTING, context_settings, defaults).unwrap();
    let convert_to_absolute =
      get_boolean_setting_from_context(CONVERT_TO_ABSOLUTE_SETTING, context_settings, defaults)
        .unwrap();
    let action = LinkAction {
      skip_in_ci,
      fs,
      path,
      target,
      relink,
      force,
      convert_to_absolute,
      create_parent_dirs,
      ignore_missing_target,
      relative,
      resolve_symlink_target,
    };
    log::trace!("Creating new {:?}", action);
    action
  }
}

impl<F: FileSystem + UnixFileSystem> Action<'_> for LinkAction<'_, F> {
  fn execute(&self) -> Result<(), DotfilesError> {
    fn create_symlink<F: FileSystem + UnixFileSystem>(
      fs: &'_ F,
      action: &'_ LinkAction<F>,
      mut target: PathBuf,
    ) -> io::Result<()> {
      let path: PathBuf = PathBuf::from(action.path());
      let path = process_home_dir_in_path(&path);

      let target_exists = fs.is_dir(&target) || fs.is_file(&target);
      let path_exists = fs.is_dir(&path) || fs.is_file(&path);
      let path_is_symlink = fs.get_symlink_src(&path).is_ok();
      let target_is_symlink = fs.get_symlink_src(&target).is_ok();
      if target_is_symlink && action.resolve_symlink_target() {
        fn resolve_symlink_target<F: FileSystem + UnixFileSystem, P: AsRef<Path>>(
          fs: &'_ F,
          link_path: P,
        ) -> io::Result<PathBuf> {
          match fs.get_symlink_src(link_path.as_ref()) {
            Ok(link_target) => resolve_symlink_target(fs, link_target),
            Err(e) if [ErrorKind::IsADirectory, ErrorKind::InvalidInput].contains(&e.kind()) => {
              Ok(PathBuf::from(link_path.as_ref()))
            }
            Err(e) => Err(e),
          }
        }
        target = resolve_symlink_target(fs, &target)?
      }
      if target_exists || action.ignore_missing_target() {
        if !fs.is_dir(path.parent().unwrap()) && action.create_parent_dirs() {
          fs.create_dir_all(path.parent().unwrap())?
        }
        match (path_exists, action.force(), fs.is_dir(&path), path_is_symlink, action.relink()) {
                        (true, true, true, _, _ ) =>fs.remove_dir_all(&path)?, // path exists, force, is_dir
                        (true, true, false, _, _ ) =>fs.remove_file(&path)?, // path exists, force, is_file
                        (true, false, _, true, true ) =>fs.remove_file(&path)?, // path exists, no force, is_symlink, relink
                        (true, false, _, true, false) =>
                            // path exists, no force, is symlink, no relink
                            return Err(io::Error::new(
                                ErrorKind::AlreadyExists,
                                format!("{:?} already exists. Use `force` to delete a file/directory or `relink` to recreate a symlink", &path))),
                        _ => ()
                    }
        fs.symlink(&target, &path)
      } else {
        Err(io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Couldn't find target file {:?} to link to, use `ignore_missing_target` to ignore",
            target
          ),
        ))
      }
    }
    let target = PathBuf::from(self.target());
    let mut target = process_home_dir_in_path(&target);
    if target.is_relative() && self.convert_to_absolute() {
      target = self.fs.current_dir().map_or_else(
        |err| Err(DotfilesError::from_io_error(err)),
        |mut path| {
          path.push(&target);
          Ok(path)
        },
      )?;
    }
    if target.is_relative() && !self.relative() {
      return Err(DotfilesError::from(
        format!(
          "{} is a relative link target but the action is not set to allow relative targets",
          self.target()
        ),
        ErrorType::InconsistentConfigurationError,
      ));
    };
    match create_symlink(self.fs, self, target) {
      Ok(()) => {
        info!("Created symlink {} -> {}", &self.path, &self.target);
        Ok(())
      }
      Err(err) => {
        error!(
          "Couldn't create symlink {} -> {}: {}",
          &self.path, &self.target, err
        );
        Err(DotfilesError::from(
          err.to_string(),
          ErrorType::FileSystemError { fs_error: err },
        ))
      }
    }
  }
}

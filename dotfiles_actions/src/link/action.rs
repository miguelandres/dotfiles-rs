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

//! This module contains the [LinkAction] that creates a new symlink
//! when executed

use derivative::Derivative;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::error::ErrorType;
use dotfiles_core::path::convert_path_to_absolute;
use dotfiles_core::path::process_home_dir_in_path;
use dotfiles_core::settings::{initialize_settings_object, Setting, Settings};
use dotfiles_core::yaml_util::{self, get_boolean_setting_from_context};
use dotfiles_core::{action::SKIP_IN_CI_SETTING, Action};
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
use strict_yaml_rust::StrictYaml;

/// Path setting (path of the symlink)
pub const PATH_SETTING: &str = "path";
/// Target setting (path to the file the symlink points to)
pub const TARGET_SETTING: &str = "target";
/// Force setting, replaces any other file or directory
pub const FORCE_SETTING: &str = "force";
/// Relink setting, if true the action relinks an existing symlink
/// (applies if force is false)
pub const RELINK_SETTING: &str = "relink";
/// Create parent dirs if they don't exist
pub const CREATE_PARENT_DIRS_SETTING: &str = "create_parent_dirs";
/// Create the symlink even if the target file does not exist
pub const IGNORE_MISSING_TARGET_SETTING: &str = "ignore_missing_target";
/// Resolves the target if it is a symlink and uses the final target file as the target.
pub const RESOLVE_SYMLINK_TARGET_SETTING: &str = "resolve_symlink_target";

/// Initialize the defaults for the LinkAction.
pub fn default_settings() -> Settings {
  initialize_settings_object(&[
    (FORCE_SETTING.to_owned(), Setting::Boolean(false)),
    (RELINK_SETTING.to_owned(), Setting::Boolean(false)),
    (
      CREATE_PARENT_DIRS_SETTING.to_owned(),
      Setting::Boolean(false),
    ),
    (
      IGNORE_MISSING_TARGET_SETTING.to_owned(),
      Setting::Boolean(false),
    ),
    (
      RESOLVE_SYMLINK_TARGET_SETTING.to_owned(),
      Setting::Boolean(false),
    ),
    (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
  ])
}

/// [LinkAction] creates a new symlink `path` that points to `target`.
///
/// It is equivalent to running `ln -s <target> <path>`
#[derive(Derivative, Getters, CopyGetters)]
#[derivative(Debug, PartialEq)]
pub struct LinkAction<F: FileSystem + UnixFileSystem> {
  /// Skips this action if it is running in a CI environment.
  skip_in_ci: bool,
  /// FileSystem to use to create the directory.
  ///
  /// Having a filesystem instance here allows us to use fakes/mocks to use
  /// in unit tests.
  #[derivative(Debug = "ignore", PartialEq = "ignore")]
  fs: F,
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
  /// Create all parent directories if they do not exist already
  #[getset(get_copy = "pub")]
  create_parent_dirs: bool,
  /// Succeed even if `target` doesn't point to an existing file or dir.
  #[getset(get_copy = "pub")]
  ignore_missing_target: bool,
  /// If the target is another symlink, resolve the ultimate concrete file
  /// or directory that it points to and make it the target
  #[getset(get_copy = "pub")]
  resolve_symlink_target: bool,
  /// Current directory that will be used to determine relative file locations if necessary. It
  /// must match the parent directory of the configuration file that declared this action.
  current_dir: PathBuf,
}

/// A native create action that works on the real filesystem.
pub type NativeLinkAction = LinkAction<OsFileSystem>;
/// A Fake create action that works on a fake test filesystem.
pub type FakeLinkAction = LinkAction<FakeFileSystem>;

impl<F: FileSystem + UnixFileSystem> LinkAction<F> {
  /// Constructs a new [LinkAction]
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    fs: F,
    path: String,
    target: String,
    context_settings: &'_ Settings,
    defaults: &'_ Settings,
    current_dir: PathBuf,
  ) -> Result<Self, DotfilesError> {
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
    let resolve_symlink_target =
      get_boolean_setting_from_context(RESOLVE_SYMLINK_TARGET_SETTING, context_settings, defaults)
        .unwrap();
    let skip_in_ci =
      get_boolean_setting_from_context(SKIP_IN_CI_SETTING, context_settings, defaults).unwrap();
    let action = Self {
      skip_in_ci,
      fs,
      path,
      target,
      relink,
      force,
      create_parent_dirs,
      ignore_missing_target,
      resolve_symlink_target,
      current_dir,
    };
    log::trace!("Creating new {:?}", action);
    Ok(action)
  }
}

impl<F: FileSystem + UnixFileSystem> Action for LinkAction<F> {
  fn execute(&self) -> Result<(), DotfilesError> {
    fn create_symlink<F: FileSystem + UnixFileSystem>(
      fs: &'_ F,
      action: &'_ LinkAction<F>,
      path: PathBuf,
      mut target: PathBuf,
    ) -> io::Result<()> {
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
            "Couldn't find target file {target:?} to link to, use `ignore_missing_target` to ignore",
          ),
        ))
      }
    }
    let path: PathBuf = PathBuf::from(self.path());
    let mut path = process_home_dir_in_path(&path);
    path = convert_path_to_absolute(&path, Some(&self.current_dir))?;
    let target = PathBuf::from(self.target());
    let mut target = process_home_dir_in_path(&target);
    target = convert_path_to_absolute(&target, Some(&self.current_dir))?;
    match create_symlink(&self.fs, self, path, target) {
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

  fn skip_in_ci(&self) -> bool {
    self.skip_in_ci
  }
}

/// Static parsing function to build a LinkAction from YAML and settings context
pub fn parse_action<F: FileSystem + UnixFileSystem + Clone>(
  fs: F,
  settings: &Settings,
  yaml: &StrictYaml,
  current_directory: &Path,
) -> Result<LinkAction<F>, DotfilesError> {
  parse_shortened_action(fs.clone(), settings, yaml, current_directory)
    .or_else(|_| parse_full_action(fs, settings, yaml, current_directory))
}

fn parse_full_action<F: FileSystem + UnixFileSystem>(
  fs: F,
  context_settings: &Settings,
  yaml: &StrictYaml,
  current_dir: &Path,
) -> Result<LinkAction<F>, DotfilesError> {
  let defaults = default_settings();
  let path = yaml_util::get_string_setting_from_yaml_or_context(
    PATH_SETTING,
    yaml,
    context_settings,
    &defaults,
  )?;
  let target = yaml_util::get_string_setting_from_yaml_or_context(
    TARGET_SETTING,
    yaml,
    context_settings,
    &defaults,
  )?;
  let action_settings: Result<Settings, DotfilesError> = defaults
    .iter()
    .map(|(name, _)| {
      yaml_util::get_setting_from_yaml_hash_or_from_context(name, yaml, context_settings, &defaults)
        .map(|setting| (name.to_owned(), setting))
    })
    .collect();

  LinkAction::<F>::new(
    fs,
    path,
    target,
    &action_settings?,
    &defaults,
    current_dir.to_owned(),
  )
}

fn parse_shortened_action<F: FileSystem + UnixFileSystem>(
  fs: F,
  context_settings: &Settings,
  yaml: &StrictYaml,
  current_dir: &Path,
) -> Result<LinkAction<F>, DotfilesError> {
  let defaults = default_settings();
  if let StrictYaml::Hash(hash) = yaml {
    match hash.len() {
      1 => {
        if let (StrictYaml::String(path), StrictYaml::String(target)) = hash.front().unwrap() {
          LinkAction::<F>::new(
            fs,
            path.clone(),
            target.clone(),
            context_settings,
            &defaults,
            current_dir.to_owned()
          )
        } else {
          Err(DotfilesError::from_wrong_yaml(
                      "StrictYaml passed to configure a short Link action is not a hash of string to string, cant parse".into(),
                      yaml.to_owned(), StrictYaml::Hash(Default::default())))
        }
      }

      x => Err(DotfilesError::from(
        format!(
          "StrictYaml passed to configure a short Link action is a hash with {x} values, must be just 1",),
        ErrorType::InconsistentConfigurationError,
      )),
    }
  } else {
    Err(DotfilesError::from_wrong_yaml(
      "StrictYaml passed to configure a Link action is not a Hash".into(),
      yaml.to_owned(),
      StrictYaml::Hash(Default::default()),
    ))
  }
}

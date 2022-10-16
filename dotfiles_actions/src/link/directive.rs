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

//! This module defines [LinkDirective].

extern crate yaml_rust;

use crate::filesystem::FileSystemDirective;
use crate::link::action::LinkAction;
use dotfiles_core::action::Action;
use dotfiles_core::action::ActionParser;
use dotfiles_core::directive::Directive;
use dotfiles_core::directive::DirectiveData;
use dotfiles_core::directive::HasDirectiveData;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::error::ErrorType;
use dotfiles_core::settings::initialize_settings_object;
use dotfiles_core::settings::Setting;
use dotfiles_core::settings::Settings;
use dotfiles_core::yaml_util::*;
use dotfiles_core_macros::ActionListDirective;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use filesystem::UnixFileSystem;
use std::marker::PhantomData;

use yaml_rust::Yaml;

/// Name of the link directive
pub const DIRECTIVE_NAME: &str = "link";
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
/// TODO: Allow relative symlinks, if false any relative symlinks cause a failure.
pub const RELATIVE_SETTING: &str = "relative";
/// Resolves the target if it is a symlink and uses the final target file as the target.
pub const RESOLVE_SYMLINK_TARGET_SETTING: &str = "resolve_symlink_target";

/// Initialize the defaults for the LinkDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::from(
    DIRECTIVE_NAME.into(),
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
      (RELATIVE_SETTING.to_owned(), Setting::Boolean(false)),
      (
        RESOLVE_SYMLINK_TARGET_SETTING.to_owned(),
        Setting::Boolean(false),
      ),
    ]),
  )
}

/// A directive that can build [LinkAction]s to create directories
/// in the filesystem.
#[derive(ActionListDirective)]
pub struct LinkDirective<'a, F: FileSystem + UnixFileSystem + Default> {
  fs: F,
  data: DirectiveData,
  phantom: PhantomData<&'a F>,
}

/// [LinkDirective] that uses the native [OsFileSystem].
pub type NativeLinkDirective<'a> = LinkDirective<'a, OsFileSystem>;
/// [LinkDirective] that uses the native [FakeFileSystem] for testing.
pub type FakeLinkDirective<'a> = LinkDirective<'a, FakeFileSystem>;

impl<'a, F: FileSystem + UnixFileSystem + Default> Default for LinkDirective<'a, F> {
  fn default() -> Self {
    Self {
      fs: Default::default(),
      data: init_directive_data(),
      phantom: Default::default(),
    }
  }
}

impl<'a, F: FileSystem + UnixFileSystem + Default> FileSystemDirective<'a, F>
  for LinkDirective<'a, F>
{
  fn fs(&self) -> &F {
    &self.fs
  }

  fn mut_fs(&mut self) -> &mut F {
    &mut self.fs
  }
}

impl<'a, F: FileSystem + UnixFileSystem + Default> LinkDirective<'a, F> {
  /// Returns the [FileSystem] instance being used.
  pub fn fs(&self) -> &F {
    &self.fs
  }

  fn parse_full_action(
    &'a self,
    context_settings: &Settings,
    yaml: &Yaml,
  ) -> Result<LinkAction<'a, F>, DotfilesError> {
    let path = get_string_setting_from_yaml_or_context(
      PATH_SETTING,
      yaml,
      context_settings,
      self.data.defaults(),
    )?;
    let target = get_string_setting_from_yaml_or_context(
      TARGET_SETTING,
      yaml,
      context_settings,
      self.data.defaults(),
    )?;
    let action_settings: Result<Settings, DotfilesError> = [
      RELINK_SETTING,
      FORCE_SETTING,
      CREATE_PARENT_DIRS_SETTING,
      IGNORE_MISSING_TARGET_SETTING,
      RELATIVE_SETTING,
      RESOLVE_SYMLINK_TARGET_SETTING,
    ]
    .iter()
    .map(|&name| {
      self
        .get_setting_from_yaml_hash_or_from_context(name, yaml, context_settings)
        .map(|setting| (name.to_owned(), setting))
    })
    .collect();

    Ok(LinkAction::<'a, F>::new(
      &self.fs,
      path,
      target,
      &action_settings?,
      self.data.defaults(),
    ))
  }

  /// Parse a shortened action with only link name to target name
  pub fn parse_shortened_action(
    &'a self,
    context_settings: &Settings,
    yaml: &Yaml,
  ) -> Result<LinkAction<'a, F>, DotfilesError> {
    if let Yaml::Hash(hash) = yaml {
      match hash.len() {
        1 => {
          if let (Yaml::String(path), Yaml::String(target)) = hash.front().unwrap() {
            Ok(LinkAction::<'a, F>::new(
              &self.fs,
              path.clone(),
              target.clone(),
              context_settings,
              self.data.defaults(),
            ))
          } else {
            Err(DotfilesError::from_wrong_yaml(
                        "Yaml passed to configure a short Link action is not a hash of string to string, cant parse".into(),
                        yaml.to_owned(), Yaml::Hash(Default::default())))
          }
        }

        x => Err(DotfilesError::from(
          format!(
            "Yaml passed to configure a short Link action is a hash with {} values, must be just 1",
            x
          ),
          ErrorType::InconsistentConfigurationError,
        )),
      }
    } else {
      Err(DotfilesError::from_wrong_yaml(
        "Yaml passed to configure a Link action is not a Hash".into(),
        yaml.to_owned(),
        Yaml::Hash(Default::default()),
      ))
    }
  }
}

impl<'a, F: FileSystem + UnixFileSystem + Default> ActionParser<'a> for LinkDirective<'a, F> {
  type ActionType = LinkAction<'a, F>;

  fn parse_action(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<LinkAction<'a, F>, DotfilesError> {
    self
      .parse_shortened_action(settings, yaml)
      .or_else(|_| self.parse_full_action(settings, yaml))
  }
}

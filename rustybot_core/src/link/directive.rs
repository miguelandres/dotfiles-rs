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

//! This module defines [LinkDirective].

extern crate yaml_rust;

use crate::directive::initialize_settings_object;
use crate::directive::Directive;
use crate::directive::DirectiveData;
use crate::directive::Setting;
use crate::directive::Settings;
use crate::link::action::LinkAction;
use crate::yaml_util::*;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use filesystem::UnixFileSystem;
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

/// Create a new link directive using the native filesystem
pub fn new_native_link_directive() -> LinkDirective<OsFileSystem> {
  LinkDirective::new(OsFileSystem::new())
}

/// Initialize the defaults for the LinkDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::new(
    DIRECTIVE_NAME,
    initialize_settings_object(&[
      (String::from(FORCE_SETTING), Setting::Boolean(false)),
      (String::from(RELINK_SETTING), Setting::Boolean(false)),
      (
        String::from(CREATE_PARENT_DIRS_SETTING),
        Setting::Boolean(false),
      ),
      (
        String::from(IGNORE_MISSING_TARGET_SETTING),
        Setting::Boolean(false),
      ),
      (String::from(RELATIVE_SETTING), Setting::Boolean(false)),
      (
        String::from(RESOLVE_SYMLINK_TARGET_SETTING),
        Setting::Boolean(false),
      ),
    ]),
  )
}

/// A directive that can build [LinkAction]s to create directories
/// in the filesystem.
pub struct LinkDirective<F: FileSystem> {
  fs: Box<F>,
  data: DirectiveData,
}

impl<F: FileSystem + UnixFileSystem> LinkDirective<F> {
  /// Returns the [FileSystem] instance being used.
  pub fn fs(&self) -> &F {
    self.fs.as_ref()
  }

  /// Create a new [LinkDirective]
  pub fn new(fs: F) -> LinkDirective<F> {
    LinkDirective::<F> {
      fs: Box::new(fs),
      data: init_directive_data(),
    }
  }

  fn parse_full_action(
    &self,
    yaml: &Yaml,
    context_settings: &Settings,
  ) -> Result<LinkAction<'_, F>, String> {
    match yaml {
      Yaml::Hash(_) => {
        let path = get_string_setting_from_yaml_or_defaults(
          PATH_SETTING,
          yaml,
          context_settings,
          self.data.defaults(),
        )
        .unwrap();
        let target = get_string_setting_from_yaml_or_defaults(
          TARGET_SETTING,
          yaml,
          context_settings,
          self.data.defaults(),
        )
        .unwrap();
        let action_settings: Settings = [
          RELINK_SETTING,
          FORCE_SETTING,
          CREATE_PARENT_DIRS_SETTING,
          IGNORE_MISSING_TARGET_SETTING,
          RELATIVE_SETTING,
          RESOLVE_SYMLINK_TARGET_SETTING,
        ]
        .iter()
        .map(|&name| {
          (
            String::from(name),
            Setting::Boolean(
              get_boolean_setting(name, context_settings, self.data.defaults()).unwrap(),
            ),
          )
        })
        .collect();
        Ok(LinkAction::new(
          &self.fs,
          path,
          target,
          &action_settings,
          self.data.defaults(),
        ))
      }
      _ => Err(format!(
        "Yaml passed to configure a Link action is not a Hash, thus cannot be parsed: {:?}",
        yaml
      )),
    }
  }

  fn parse_shortened_action(
    &self,
    yaml: &Yaml,
    context_settings: &Settings,
  ) -> Result<LinkAction<'_, F>, String> {
    match yaml {
      Yaml::Hash(hash) => match hash.len() {
        1 => {
          if let (Yaml::String(path), Yaml::String(target)) = hash.front().unwrap() {
            Ok(LinkAction::new(
              &self.fs,
              path.clone(),
              target.clone(),
              context_settings,
              self.data.defaults(),
            ))
          } else {
            Err(format!(
                        "Yaml passed to configure a short Link action is not a hash of string to string, cant parse: {:?}", yaml
                    ))
          }
        }
        x => Err(format!(
          "Yaml passed to configure a short Link action is a hash with {:} values, must be just 1",
          x
        )),
      },
      _ => Err(format!(
        "Yaml passed to configure a Link action is not a Hash, thus cannot be parsed: {:?}",
        yaml
      )),
    }
  }
}

impl<'a, F: FileSystem + UnixFileSystem> Directive<'a, LinkAction<'a, F>> for LinkDirective<F> {
  fn name(&self) -> &str {
    self.data.name()
  }

  fn defaults(&self) -> &Settings {
    self.data.defaults()
  }

  fn build_action(&'a self, settings: &Settings, yaml: &Yaml) -> Result<LinkAction<'a, F>, String> {
    if let Ok(action) = self.parse_shortened_action(yaml, settings) {
      Ok(action)
    } else {
      self.parse_full_action(yaml, settings)
    }
  }
}

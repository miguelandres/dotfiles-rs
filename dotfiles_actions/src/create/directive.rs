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

//! This module defines [CreateDirective].
extern crate strict_yaml_rust;

use crate::create::action::CreateAction;
use crate::filesystem::FileSystemDirective;
use dotfiles_core::action::ActionParser;
use dotfiles_core::action::SKIP_IN_CI_SETTING;
use dotfiles_core::directive::DirectiveData;
use dotfiles_core::directive::HasDirectiveData;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::settings::initialize_settings_object;
use dotfiles_core::settings::Setting;
use dotfiles_core::yaml_util;
use dotfiles_core_macros::Directive;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use std::marker::PhantomData;

use strict_yaml_rust::StrictYaml;

/// Constant for the name of the `create` directive.
pub const DIRECTIVE_NAME: &str = "create";
/// Constant for the name of the [`force`](CreateAction::force) Setting which forces to create all
/// parent directories if necessary.
pub const FORCE_SETTING: &str = "force";
/// Constant for the name of the [`directory`](CreateAction::directory) argument that contains the
/// name of the directory to create
pub const DIR_SETTING: &str = "dir";

/// Initializes the default configuration for the [CreateDirective]
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::from(
    DIRECTIVE_NAME.into(),
    initialize_settings_object(&[
      (FORCE_SETTING.to_owned(), Setting::Boolean(false)),
      (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
    ]),
  )
}

/// A directive that can build [CreateAction]s to create directories in the filesystem.
#[derive(Directive, Clone)]
pub struct CreateDirective<'a, F: FileSystem + Default> {
  fs: F,
  data: DirectiveData,
  phantom: PhantomData<&'a F>,
}
/// [CreateDirective] that uses the native [OsFileSystem].
pub type NativeCreateDirective<'a> = CreateDirective<'a, OsFileSystem>;
/// [CreateDirective] that uses the native [FakeFileSystem] for testing.
pub type FakeCreateDirective<'a> = CreateDirective<'a, FakeFileSystem>;

impl<'a, F: FileSystem + Default> FileSystemDirective<'a, F> for CreateDirective<'a, F> {
  fn fs(&self) -> &F {
    &self.fs
  }

  fn mut_fs(&mut self) -> &mut F {
    &mut self.fs
  }
}

impl<'a, F: FileSystem + Default> Default for CreateDirective<'a, F> {
  fn default() -> Self {
    Self {
      fs: Default::default(),
      data: init_directive_data(),
      phantom: Default::default(),
    }
  }
}

impl<'a, F: FileSystem + Default> ActionParser<'a> for CreateDirective<'a, F> {
  type ActionType = CreateAction<'a, F>;

  fn parse_action(
    &'a self,
    settings: &std::collections::HashMap<String, Setting>,
    yaml: &StrictYaml,
  ) -> Result<CreateAction<F>, DotfilesError> {
    Ok(CreateAction::<'a, F>::new(
      self.fs(),
      yaml_util::get_boolean_setting_from_yaml_or_context(
        SKIP_IN_CI_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
      yaml_util::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
      yaml_util::get_boolean_setting_from_yaml_or_context(
        FORCE_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
    ))
  }
}

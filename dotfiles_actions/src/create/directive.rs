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
extern crate yaml_rust;

use crate::create::action::CreateAction;
use dotfiles_core::action::Action;
use dotfiles_core::action::ActionParser;
use dotfiles_core::directive::initialize_settings_object;
use dotfiles_core::directive::Directive;
use dotfiles_core::directive::DirectiveData;
use dotfiles_core::directive::Setting;
use dotfiles_core::directive::Settings;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::yaml_util;
use dotfiles_core_macros::ActionListDirective;
use filesystem::FileSystem;
use filesystem::OsFileSystem;

use std::marker::PhantomData;
use std::vec::Vec;
use yaml_rust::Yaml;

/// Constant for the name of the `create` directive.
pub const DIRECTIVE_NAME: &str = "create";
/// Constant for the name of the [`force`](CreateAction::force) Setting
/// which forces to create all parent directories if necessary.
pub const FORCE_SETTING: &str = "force";
/// Constant for the name of the [`directory`](CreateAction::directory) Setting
/// which forces to create all parent directories if necessary.
pub const DIR_SETTING: &str = "dir";

/// Constructs a new [CreateDirective] using the real filesystem
pub fn new_native_create_directive<'a>() -> CreateDirective<'a, OsFileSystem> {
  CreateDirective::<'a, OsFileSystem>::new(OsFileSystem::new())
}

/// Initializes the default configuration for the [CreateDirective]
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::from(
    DIRECTIVE_NAME,
    initialize_settings_object(&[(String::from(FORCE_SETTING), Setting::Boolean(false))]),
  )
}

/// A directive that can build [CreateAction]s to create directories
/// in the filesystem.
#[derive(ActionListDirective)]
pub struct CreateDirective<'a, F: 'a + FileSystem> {
  fs: Box<F>,
  data: DirectiveData,
  phantom: PhantomData<&'a F>,
}

impl<'a, F: 'a + FileSystem> CreateDirective<'a, F> {
  /// Returns the [FileSystem] instance being used.
  pub fn fs(&self) -> &F {
    self.fs.as_ref()
  }

  /// Constructs a new instance of the create directive.
  pub fn new(fs: F) -> Self {
    CreateDirective::<'a, F> {
      fs: Box::new(fs),
      data: init_directive_data(),
      phantom: PhantomData,
    }
  }
}

impl<'a, F: 'a + FileSystem> ActionParser<'a> for CreateDirective<'a, F> {
  type ActionType = CreateAction<'a, F>;

  fn name(&'a self) -> &'static str {
    "create"
  }

  fn parse_action(
    &'a self,
    settings: &std::collections::HashMap<String, Setting>,
    yaml: &Yaml,
  ) -> Result<CreateAction<F>, DotfilesError> {
    Ok(CreateAction::<'a, F>::new(
      self.fs(),
      yaml_util::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
      yaml_util::get_boolean_setting_from_yaml_or_defaults(
        FORCE_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
    ))
  }
}

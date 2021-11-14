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

//! This module defines [CreateDirective].
extern crate yaml_rust;

use crate::action::Action;
use crate::create::action::CreateAction;
use crate::directive::initialize_settings_object;
use crate::directive::Directive;
use crate::directive::DirectiveData;
use crate::directive::Setting;
use crate::directive::Settings;
use crate::yaml_util;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use std::marker::PhantomData;
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
  DirectiveData::new(
    DIRECTIVE_NAME,
    initialize_settings_object(&[(String::from(FORCE_SETTING), Setting::Boolean(false))]),
  )
}

/// A directive that can build [CreateAction]s to create directories
/// in the filesystem.
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
  pub fn new(fs: F) -> CreateDirective<'a, F> {
    CreateDirective::<'a, F> {
      fs: Box::new(fs),
      data: init_directive_data(),
      phantom: PhantomData,
    }
  }
}

impl<'a, F: 'a + FileSystem> Directive<'a> for CreateDirective<'a, F> {
  fn name(&self) -> &str {
    self.data.name()
  }

  fn defaults(&self) -> &Settings {
    self.data.defaults()
  }

  fn build_action(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Box<dyn Action<'a> + 'a>, String> {
    Ok(Box::new(CreateAction::<'a, F>::new(
      self.fs(),
      yaml_util::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
      yaml_util::get_boolean_setting(FORCE_SETTING, settings, self.defaults())?,
    )))
  }
}

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
use dotfiles_core::check_action_list_or_err;
use dotfiles_core::directive::initialize_settings_object;
use dotfiles_core::directive::Directive;
use dotfiles_core::directive::DirectiveData;
use dotfiles_core::directive::Setting;
use dotfiles_core::directive::Settings;
use dotfiles_core::yaml_util;
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
  pub fn new(fs: F) -> Self {
    CreateDirective::<'a, F> {
      fs: Box::new(fs),
      data: init_directive_data(),
      phantom: PhantomData,
    }
  }
  /// Parses a create action from a yaml file
  pub fn parse_create_action(
    &'a self,
    settings: &std::collections::HashMap<String, Setting>,
    yaml: &Yaml,
  ) -> Result<CreateAction<F>, String> {
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

  /// Parses a list of create actions from a yaml file
  pub fn parse_create_action_list(
    &'a self,
    settings: &std::collections::HashMap<String, Setting>,
    yaml: &Yaml,
  ) -> Result<Vec<CreateAction<F>>, String> {
    if let Yaml::Array(arr) = yaml {
      check_action_list_or_err!(
        CreateAction<F>,
        arr
          .iter()
          .map(|yaml_item| self.parse_create_action(settings, yaml_item))
          .collect()
      )
    } else {
      Err("create directive expects an array of create actions, did not find an array.".to_string())
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
  ) -> Result<Vec<Box<(dyn Action<'a> + 'a)>>, std::string::String> {
    self.parse_create_action_list(settings, yaml).map(|vec| {
      let result: Vec<Box<(dyn Action<'a> + 'a)>> = vec
        .into_iter()
        .map(|action| {
          let boxed: Box<(dyn Action<'a> + 'a)> = Box::new(action);
          boxed
        })
        .collect();
      result
    })
  }
}
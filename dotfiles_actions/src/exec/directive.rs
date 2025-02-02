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

//! This module defines [ExecDirective] which represents commands to execute in a shell.

use dotfiles_core::{action::SKIP_IN_CI_SETTING, directive::HasDirectiveData};
use dotfiles_core_macros::Directive;
use std::{collections::HashMap, marker::PhantomData, path::Path};
use strict_yaml_rust::StrictYaml;

use dotfiles_core::{
  action::ActionParser, directive::DirectiveData, error::DotfilesError,
  settings::initialize_settings_object, yaml_util, Setting,
};

use crate::exec::action::ExecAction;

/// Name of the Exec directive
pub const DIRECTIVE_NAME: &str = "exec";
/// Echo the command to run before running it.
pub const ECHO_SETTING: &str = "echo";
/// Command to run
pub const COMMAND_SETTING: &str = "cmd";
/// Optional description for the command to run
pub const DESCRIPTION_SETTING: &str = "description";

/// Initialize the defaults for the BrewDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::from(
    DIRECTIVE_NAME.into(),
    initialize_settings_object(&[
      (ECHO_SETTING.to_owned(), Setting::Boolean(false)),
      (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
    ]),
  )
}

/// A directive that can build [ExecAction]s to run commands
#[derive(Directive, Clone)]
pub struct ExecDirective<'a> {
  data: DirectiveData,
  phantom_data: PhantomData<&'a DirectiveData>,
}

impl<'a> Default for ExecDirective<'a> {
  fn default() -> Self {
    Self {
      data: init_directive_data(),
      phantom_data: PhantomData,
    }
  }
}

impl<'a> ActionParser<'a> for ExecDirective<'a> {
  type ActionType = ExecAction<'a>;
  fn parse_action(
    &'a self,
    settings: &HashMap<String, Setting>,
    yaml: &StrictYaml,
    current_dir: &Path,
  ) -> Result<ExecAction<'a>, DotfilesError> {
    ExecAction::new(
      yaml_util::get_boolean_setting_from_yaml_or_context(
        SKIP_IN_CI_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
      yaml_util::get_string_content_or_keyed_value(yaml, Some(COMMAND_SETTING))?,
      yaml_util::get_string_setting_from_yaml_or_context(
        DESCRIPTION_SETTING,
        yaml,
        settings,
        self.defaults(),
      )
      .ok(),
      yaml_util::get_boolean_setting_from_yaml_or_context(
        ECHO_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
      current_dir,
    )
  }
}

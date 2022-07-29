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

use std::{collections, marker::PhantomData};

use dotfiles_core_macros::ActionListDirective;
use yaml_rust::Yaml;

use dotfiles_core::{
  check_action_list_or_err,
  directive::{initialize_settings_object, DirectiveData},
  error::{DotfilesError, ErrorType},
  yaml_util, Action, Directive, Setting, Settings,
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

/// Create a new brew directive.
pub fn new_exec_directive<'a>() -> ExecDirective<'a> {
  ExecDirective::new()
}

/// Initialize the defaults for the BrewDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::new(
    DIRECTIVE_NAME,
    initialize_settings_object(&[(String::from(ECHO_SETTING), Setting::Boolean(false))]),
  )
}

/// A directive that can build [ExecAction]s to run commands
#[derive(ActionListDirective)]
pub struct ExecDirective<'a> {
  data: DirectiveData,
  phantom_data: PhantomData<&'a DirectiveData>,
}

impl<'a> ExecDirective<'a> {
  /// Creates a new Exec Directives
  pub fn new() -> Self {
    Self {
      data: init_directive_data(),
      phantom_data: PhantomData,
    }
  }

  /// Parses a exec action from a yaml file
  pub fn parse_exec_action(
    &'a self,
    settings: &collections::HashMap<String, Setting>,
    yaml: &Yaml,
  ) -> Result<ExecAction, DotfilesError> {
    Ok(ExecAction::new(
      yaml_util::get_string_content_or_keyed_value(yaml, Some(COMMAND_SETTING))?,
      yaml_util::get_string_setting_from_yaml_or_defaults(
        DESCRIPTION_SETTING,
        yaml,
        settings,
        self.defaults(),
      )
      .map_or(Option::<String>::None, |desc| Some(desc)),
      yaml_util::get_boolean_setting_from_yaml_or_defaults(
        ECHO_SETTING,
        yaml,
        settings,
        self.defaults(),
      )?,
    ))
  }

  /// Parses a list of exec actions from a yaml file
  pub fn parse_action_list(
    &'a self,
    settings: &std::collections::HashMap<String, Setting>,
    yaml: &Yaml,
  ) -> Result<Vec<ExecAction>, DotfilesError> {
    if let Yaml::Array(arr) = yaml {
      check_action_list_or_err!(
        ExecAction,
        arr
          .iter()
          .map(|yaml_item| self.parse_exec_action(settings, yaml_item))
          .collect()
      )
    } else {
      Err(DotfilesError::from(
        "Exec directive expects an array of exec actions, did not find an array.".to_string(),
        ErrorType::UnexpectedYamlTypeError,
      ))
    }
  }
}

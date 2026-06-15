// Copyright (c) 2021-2026 Miguel Barreto and others
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

//! This module contains the [ExecAction] that executes a command in the shell

use std::path::Path;

use dotfiles_core::action::SKIP_IN_CI_SETTING;
use dotfiles_core::error::execution_error;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::settings::{initialize_settings_object, Setting, Settings};
use dotfiles_core::{yaml_util, Action};
use strict_yaml_rust::StrictYaml;
use subprocess::Exec;

/// Echo the command to run before running it.
pub const ECHO_SETTING: &str = "echo";
/// Command to run
pub const COMMAND_SETTING: &str = "cmd";
/// Optional description for the command to run
pub const DESCRIPTION_SETTING: &str = "description";

/// Default settings for the Exec action.
pub fn default_settings() -> Settings {
  initialize_settings_object(&[
    (ECHO_SETTING.to_owned(), Setting::Boolean(false)),
    (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
  ])
}

/// [ExecAction] Installs software using homebrew.
#[derive(Eq, PartialEq, Debug)]
pub struct ExecAction {
  /// Skips this action if it is running in a CI environment.
  skip_in_ci: bool,
  /// Command to run
  command: String,
  /// Description
  description: Option<String>,
  /// Whether to print out the command for clarity.
  echo: bool,
}

impl ExecAction {
  /// Create a new Exec Action that will run from the parent directory of the config file
  pub fn new(
    skip_in_ci: bool,
    command: String,
    description: Option<String>,
    echo: bool,
    current_dir: &Path,
  ) -> Result<Self, DotfilesError> {
    let action = ExecAction {
      skip_in_ci,
      command: format!(
        "cd \"{}\" && {}",
        current_dir.as_os_str().to_str().unwrap(),
        command
      ),
      description,
      echo,
    };
    log::trace!("Creating new {:?}", action);
    Ok(action)
  }
  /// The command to run
  pub fn command(&self) -> &str {
    self.command.as_str()
  }

  /// Whether to print out the command for clarity.
  pub fn echo(&self) -> bool {
    self.echo
  }

  /// Description for the command to run.
  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }
}

impl Action for ExecAction {
  fn execute(&self) -> Result<(), DotfilesError> {
    if let Some(description) = self.description.as_ref() {
      log::info!("{}", description);
    }
    if self.echo {
      log::info!("Running command: {0}", self.command);
    }
    Exec::shell(self.command()).join().map_or_else(
      |err| {
        Err(DotfilesError::from(
          format!(
            "Couldn't run command `{0}`, failed with error {1}",
            self.command(),
            err
          ),
          execution_error(Some(err), None),
        ))
      },
      |status| match status.success() {
        true => Ok(()),
        false => Err(DotfilesError::from(
          format!(
            "Command `{0}` failed with error code {1}",
            self.command(),
            status.code().unwrap()
          ),
          execution_error(None, Some(status)),
        )),
      },
    )
  }

  fn skip_in_ci(&self) -> bool {
    self.skip_in_ci
  }
}

/// Static parsing function to build an ExecAction from YAML and settings context
pub fn parse_action(
  settings: &Settings,
  yaml: &StrictYaml,
  current_dir: &Path,
) -> Result<ExecAction, DotfilesError> {
  let defaults = default_settings();
  ExecAction::new(
    yaml_util::get_boolean_setting_from_yaml_or_context(
      SKIP_IN_CI_SETTING,
      yaml,
      settings,
      &defaults,
    )?,
    yaml_util::get_string_content_or_keyed_value(yaml, Some(COMMAND_SETTING))?,
    yaml_util::get_string_setting_from_yaml_or_context(
      DESCRIPTION_SETTING,
      yaml,
      settings,
      &defaults,
    )
    .ok(),
    yaml_util::get_boolean_setting_from_yaml_or_context(ECHO_SETTING, yaml, settings, &defaults)?,
    current_dir,
  )
}

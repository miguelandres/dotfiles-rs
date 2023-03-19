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

//! Module that contains common code for all commands that install a package

use dotfiles_core::{error::DotfilesError, exec_wrapper::execute_commands};
use log::info;
use std::fmt::Display;
use subprocess::Exec;

/// Trait that represents a command that installs an item using an action (brew install, brew
/// install cask, apt install...)
pub trait InstallCommand<F: Display> {
  /// The base command to run
  fn base_command(&self) -> Exec;
  /// The arguments to pass to the command
  fn args(&self) -> &Vec<String>;
  /// The description of the action to run, i.e. "Installing cask"
  fn action_description(&self) -> &str;
  /// The name of the action to run, i.e. "cask"
  fn action_name(&self) -> &str;

  /// The item actually being installed, for example a homebrew formula.
  fn items(&self) -> &Vec<F>;

  /// a list of items to display
  fn formatted_item_list(&self) -> Vec<String> {
    self.items().iter().map(|it| format!("{}", it)).collect()
  }

  /// Runs the command to execut
  fn execute(&self) -> Result<(), DotfilesError> {
    let item_list: String = self.formatted_item_list().join(", ").into();
    info!("{} {}", self.action_description(), item_list);
    let mut cmd = self.base_command();
    for arg in self.args().iter() {
      cmd = cmd.arg(arg);
    }
    execute_commands(
      vec![cmd],
      format!("Couldn't {} {}", self.action_name(), item_list).as_str(),
      format!(
        "Unexpected error while {} {}",
        self.action_description(),
        &item_list
      )
      .as_str(),
    )
  }
}

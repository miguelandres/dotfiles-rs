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

use std::marker::PhantomData;

use subprocess::Exec;
use subprocess::ExitStatus;

use crate::Action;

/// [ExecAction] Installs software using homebrew.
pub struct ExecAction<'a> {
  /// Command to run
  command: String,
  /// Description
  description: Option<String>,
  /// Whether to print out the command for clarity.
  echo: bool,
  phantom_data: PhantomData<&'a String>,
}

impl<'a> ExecAction<'a> {
  /// Create a new Exec Action
  pub fn new(command: String, description: Option<String>, echo: bool) -> Self {
    ExecAction {
      command,
      description,
      echo,
      phantom_data: PhantomData,
    }
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

impl<'a> Action<'a> for ExecAction<'a> {
  fn execute(&self) -> Result<(), String> {
    if let Some(description) = self.description.as_ref() {
      log::info!("{}", description);
    }
    if self.echo {
      log::info!("Running command: {0}", self.command);
    }
    Exec::shell(self.command()).join().map_or_else(
      |err| {
        Err(format!(
          "Couldn't run command `{0}`, failed with error {1}",
          self.command(),
          err.to_string()
        ))
      },
      |status| match status {
        ExitStatus::Exited(0) => Ok(()),
        ExitStatus::Exited(code) => Err(format!(
          "Command `{0}` failed with error code {1}",
          self.command(),
          code
        )),
        _ => Err(format!(
          "Unexpected error while running command `{0}`",
          self.command()
        )),
      },
    )
  }
}
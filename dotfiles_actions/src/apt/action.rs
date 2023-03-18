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

//! This module contains the [AptAction] that installs
//! packages using apt on debian-based distros

#![cfg(unix)]
use crate::install_command::InstallCommand;
use dotfiles_core::action::Action;
use dotfiles_core::error::DotfilesError;
use dotfiles_core_macros::ConditionalAction;
use getset::Getters;
use std::marker::PhantomData;
use subprocess::Exec;

struct AptCommand {
  item: String,
  args: Vec<String>,
}

impl InstallCommand<String> for AptCommand {
  fn base_command(&self) -> Exec {
    Exec::cmd("sudo")
  }

  fn args(&self) -> &Vec<String> {
    &self.args
  }

  fn action_description(&self) -> &str {
    "apt installing"
  }

  fn item(&self) -> &String {
    &self.item
  }

  fn action_name(&self) -> &str {
    "apt install"
  }
}

impl AptCommand {
  fn install(item: &str) -> AptCommand {
    AptCommand {
      item: item.into(),
      args: vec!["apt".into(), "install".into(), item.into(), "-y".into()],
    }
  }
}

/// [AptAction] Installs software using apt.
#[derive(Eq, PartialEq, Debug, ConditionalAction, Getters)]
pub struct AptAction<'a> {
  /// Skips this action if it is running in a CI environment.
  #[get = "pub"]
  skip_in_ci: bool,
  /// List of packages to install.
  #[get = "pub"]
  packages: Vec<String>,

  phantom_data: PhantomData<&'a String>,
}
impl<'a> AptAction<'a> {
  /// Constructs a new [AptAction]
  pub fn new(skip_in_ci: bool, packages: Vec<String>) -> Self {
    let action = AptAction {
      skip_in_ci,
      packages,
      phantom_data: PhantomData,
    };
    log::trace!("Creating new {:?}", action);
    action
  }
}

impl Action<'_> for AptAction<'_> {
  fn execute(&self) -> Result<(), DotfilesError> {
    for pkg in &self.packages {
      AptCommand::install(pkg).execute()?;
    }
    Ok(())
  }
}

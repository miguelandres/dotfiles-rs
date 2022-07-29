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

//! This module contains the [BrewAction] that installs
//! a brew formula using homebrew

#![cfg(unix)]
use dotfiles_core::action::Action;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::exec_wrapper::execute_command;
use log::info;
use std::marker::PhantomData;
use subprocess::Exec;

struct BrewCommand {
  item: String,
  args: Vec<String>,
  action_name: String,
  action_description: String,
}

impl BrewCommand {
  fn tap(item: &str) -> BrewCommand {
    BrewCommand {
      item: item.into(),
      args: vec!["tap".into(), item.into()],
      action_name: "tap".into(),
      action_description: "tapping".into(),
    }
  }

  fn install_formula(item: &str) -> BrewCommand {
    BrewCommand {
      item: item.into(),
      args: vec!["install".into(), item.into()],
      action_name: "install formula".into(),
      action_description: "installing formula".into(),
    }
  }

  fn install_cask(item: &str) -> BrewCommand {
    BrewCommand {
      item: item.into(),
      args: vec!["install".into(), "--cask".into(), item.into()],
      action_name: "install cask".into(),
      action_description: "installing cask".into(),
    }
  }

  pub fn execute(&self) -> Result<(), DotfilesError> {
    info!("{} {}", self.action_description, self.item);
    let mut cmd = Exec::cmd("brew");
    for arg in self.args.iter() {
      cmd = cmd.arg(arg);
    }
    execute_command(
      cmd.into(),
      format!("Couldn't {} {}", self.action_name, self.item).as_str(),
      format!(
        "Unexpected error while {} {}",
        self.action_description, self.item
      )
      .as_str(),
    )
  }
}

/// [BrewAction] Installs software using homebrew.
#[derive(Debug, PartialEq, Eq)]
pub struct BrewAction<'a> {
  /// Passes `--force` to `brew install --cask` to prevent the install failure
  /// when the app is already installed before the cask install.
  force_casks: bool,
  /// List of repositories to tap into using `brew tap`.
  taps: Vec<String>,
  /// List of brew formulae to `brew install`, usually command line tools.
  formulae: Vec<String>,

  /// List of casks to install. Casks usually are macOS apps with some sort of UI or framework dependencies.
  casks: Vec<String>,
  phantom_data: PhantomData<&'a String>,
}
impl<'a> BrewAction<'a> {
  /// Constructs a new [BrewAction]
  pub fn new(
    force_casks: bool,
    taps: Vec<String>,
    formulae: Vec<String>,
    casks: Vec<String>,
  ) -> Self {
    BrewAction {
      force_casks,
      taps,
      formulae,
      casks,
      phantom_data: PhantomData,
    }
  }

  /// List of casks to install.
  pub fn casks(&self) -> &[String] {
    self.casks.as_ref()
  }

  /// List of formulae to install.
  pub fn formulae(&self) -> &[String] {
    self.formulae.as_ref()
  }

  /// List of taps to tap into.
  pub fn taps(&self) -> &[String] {
    self.taps.as_ref()
  }

  /// Whether to pass `--force` to cask installation.
  pub fn force_casks(&self) -> bool {
    self.force_casks
  }
}

impl Action<'_> for BrewAction<'_> {
  fn execute(&self) -> Result<(), DotfilesError> {
    for tap in &self.taps {
      BrewCommand::tap(tap).execute()?;
    }
    for formula in &self.formulae {
      BrewCommand::install_formula(formula).execute()?;
    }
    for cask in &self.casks {
      BrewCommand::install_cask(cask).execute()?;
    }
    Ok(())
  }
}

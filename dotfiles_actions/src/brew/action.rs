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
use crate::install_command::InstallCommand;
use dotfiles_core::action::Action;
use dotfiles_core::error::DotfilesError;
use dotfiles_core_macros::ConditionalAction;
use getset::Getters;
use log::info;
#[cfg(target_os = "macos")]
use std::fmt::Display;
use std::marker::PhantomData;
use subprocess::Exec;
#[cfg(target_os = "macos")]
#[derive(Getters, Eq, PartialEq, Debug, Clone)]
/// An item to download from the app store
pub struct MacAppStoreItem {
  #[getset(get)]
  /// Numeric ID from the app store
  id: i64,
  #[getset(get)]
  /// Human readable name.
  name: String,
}

#[cfg(target_os = "macos")]
impl From<(i64, String)> for MacAppStoreItem {
  fn from(value: (i64, String)) -> Self {
    MacAppStoreItem {
      id: value.0,
      name: value.1,
    }
  }
}

#[cfg(target_os = "macos")]
impl Display for MacAppStoreItem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "id: {}, name: {}", self.id, self.name)
  }
}

#[cfg(target_os = "macos")]
#[derive(Eq, PartialEq, Debug, Clone)]
/// Command to download something from the mac app store
pub struct MacAppStoreCommand {
  items: Vec<MacAppStoreItem>,
  args: Vec<String>,
}

#[cfg(target_os = "macos")]
impl From<Vec<MacAppStoreItem>> for MacAppStoreCommand {
  fn from(items: Vec<MacAppStoreItem>) -> Self {
    let mut args: Vec<String> = items.iter().map(|it| it.id().to_string()).collect();
    args.insert(0, "install".into());
    MacAppStoreCommand { items, args }
  }
}

#[cfg(target_os = "macos")]
impl InstallCommand<MacAppStoreItem> for MacAppStoreCommand {
  fn base_command(&self) -> Exec {
    Exec::cmd("mas")
  }

  fn args(&self) -> &Vec<String> {
    &self.args
  }

  fn action_description(&self) -> &str {
    "Installing from Mac App Store"
  }

  fn items(&self) -> &Vec<MacAppStoreItem> {
    &self.items
  }

  fn action_name(&self) -> &str {
    "mas"
  }

  fn execute(&self) -> Result<(), DotfilesError> {
    let item_list: String = self
      .items()
      .iter()
      .map(|it| format!("{}", it))
      .collect::<Vec<String>>()
      .join(", ");
    info!("{} {}", self.action_description(), item_list);
    let mut cmd = self.base_command();
    for arg in self.args().iter() {
      cmd = cmd.arg(arg);
    }
    dotfiles_core::exec_wrapper::execute_commands(
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

struct BrewCommand {
  items: Vec<String>,
  args: Vec<String>,
  action_name: String,
  action_description: String,
}

impl InstallCommand<String> for BrewCommand {
  fn base_command(&self) -> Exec {
    Exec::cmd("brew")
  }

  fn args(&self) -> &Vec<String> {
    &self.args
  }

  fn action_description(&self) -> &str {
    &self.action_description
  }

  fn items(&self) -> &Vec<String> {
    &self.items
  }

  fn action_name(&self) -> &str {
    &self.action_name
  }
}
impl BrewCommand {
  fn tap(tap: &str) -> BrewCommand {
    BrewCommand {
      items: vec![tap.into()],
      args: vec!["tap".into(), tap.into()],
      action_name: "tap".into(),
      action_description: "tapping".into(),
    }
  }

  fn install_formulae(items: &Vec<String>) -> BrewCommand {
    let mut args: Vec<String> = items.clone();
    args.insert(0, "install".into());
    BrewCommand {
      items: items.clone(),
      args: args,
      action_name: "install formula".into(),
      action_description: "installing formula".into(),
    }
  }

  fn install_casks(items: &Vec<String>, force: &bool) -> BrewCommand {
    let mut args = vec!["install".into(), "--cask".into()];
    if *force {
      args.push("--force".into())
    }
    {
      let mut items = items.clone();
      args.append(&mut items)
    }
    let args = args;
    let items = items.clone();
    BrewCommand {
      items,
      args,
      action_name: "install cask".into(),
      action_description: "installing cask".into(),
    }
  }
}

/// [BrewAction] Installs software using homebrew.
#[derive(Eq, PartialEq, Debug, ConditionalAction, Getters)]
pub struct BrewAction<'a> {
  /// Skips this action if it is running in a CI environment.
  #[get = "pub"]
  skip_in_ci: bool,
  /// Passes `--force` to `brew install --cask` to prevent the install failure
  /// when the app is already installed before the cask install.
  #[get = "pub"]
  force_casks: bool,
  /// List of repositories to tap into using `brew tap`.
  #[get = "pub"]
  taps: Vec<String>,
  /// List of brew formulae to `brew install`, usually command line tools.
  #[get = "pub"]
  formulae: Vec<String>,

  /// List of casks to install. Casks usually are macOS apps with some sort of UI or framework
  /// dependencies.
  #[get = "pub"]
  casks: Vec<String>,

  #[cfg(target_os = "macos")]
  /// List of Mac OS apps to install from the App Store
  #[get = "pub"]
  mas_apps: Vec<MacAppStoreItem>,
  phantom_data: PhantomData<&'a String>,
}
impl<'a> BrewAction<'a> {
  /// Constructs a new [BrewAction]
  pub fn new(
    skip_in_ci: bool,
    force_casks: bool,
    taps: Vec<String>,
    formulae: Vec<String>,
    casks: Vec<String>,
    #[cfg(target_os = "macos")] mas_apps: Vec<MacAppStoreItem>,
  ) -> Self {
    let action = BrewAction {
      skip_in_ci,
      force_casks,
      taps,
      formulae,
      casks,
      #[cfg(target_os = "macos")]
      mas_apps,
      phantom_data: PhantomData,
    };
    log::trace!("Creating new {:?}", action);
    action
  }
}

impl Action<'_> for BrewAction<'_> {
  fn execute(&self) -> Result<(), DotfilesError> {
    for tap in &self.taps {
      BrewCommand::tap(tap).execute()?;
    }
    BrewCommand::install_formulae(&self.formulae).execute()?;
    BrewCommand::install_casks(&self.casks, self.force_casks()).execute()?;
    Ok(())
  }
}

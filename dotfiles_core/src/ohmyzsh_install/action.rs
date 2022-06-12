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

//! This module contains the [OhMyZshInstallAction] that sets up ohmyzsh

#![cfg(unix)]
use crate::action::Action;
use std::process::Command;
use subprocess::Exec;
use subprocess::ExitStatus;

/// [OhMyZshInstallAction] sets up ohmyzsh.
pub struct OhMyZshInstallAction {
  skip_chsh: bool,
}

impl OhMyZshInstallAction {
  /// Constructs a new [OhMyZshInstallAction]
  pub fn new(skip_chsh: bool) -> Self {
    OhMyZshInstallAction { skip_chsh }
  }
  /// Returns true if it can find a brew command
  pub fn check_zsh_is_installed(&self) -> bool {
    Command::new("which").arg("zsh").status().unwrap().success()
  }
  /// Returns true if the $ZSH environment var is set
  pub fn check_oh_my_zsh_is_installed(&self) -> bool {
    matches!(std::env::var("ZSH"), Ok(_))
  }
}

impl Action<'_> for OhMyZshInstallAction {
  fn execute(&self) -> Result<(), String> {
    if self.check_oh_my_zsh_is_installed() {
      log::info!("oh-my-zsh is already installed, no need to reinstall");
      return Ok(());
    }
    if !self.check_zsh_is_installed() {
      #[cfg(target_os = "linux")]
      let cmd = Exec::shell("sudo apt install zsh");
      #[cfg(target_os = "macos")]
      let cmd = Exec::shell("brew install zsh");
      cmd.join().map_or_else(
        |_err| Err(String::from("Error zsh installation")),
        |status| match status {
          ExitStatus::Exited(0) => Ok(()),
          ExitStatus::Exited(code) => Err(format!(
            "Couldn't run zsh installation, Error status {}",
            code
          )),
          _ => Err(String::from(
            "Unexpected error while running zsh installation",
          )),
        },
      )?;
    }

    if !self.skip_chsh {
      Exec::shell("chsh -s $(which zsh)").join().map_or_else(
        |_err| Err(String::from("Error setting shell to zsh")),
        |status| match status {
          ExitStatus::Exited(0) => Ok(()),
          ExitStatus::Exited(code) => Err(format!(
            "Couldn't run chsh to set the shell to zsh, Error status {}",
            code
          )),
          _ => Err(String::from(
            "Unexpected error while running chsh to set the shell to zsh",
          )),
        },
      )?;
    }
    Exec::shell(
      "sh -c \"$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"",
    )
    .join()
    .map_or_else(
      |_err| Err(String::from("Error installing ohmyzsh")),
      |status| match status {
        ExitStatus::Exited(0) => Ok(()),
        ExitStatus::Exited(code) => Err(format!("Couldn't install ohmyzsh, Error status {}", code)),
        _ => Err(String::from("Unexpected error while installing ohmyzsh")),
      },
    )
  }
}

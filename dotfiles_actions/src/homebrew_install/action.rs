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

//! This module contains the [HomebrewInstallAction] that installs
//! homebrew on a macOS computer

#![cfg(unix)]
use dotfiles_core::action::Action;
use log::info;
use std::process::Command;
use subprocess::Exec;
use subprocess::ExitStatus;

/// [HomebrewInstallAction] installs homebrew.
pub struct HomebrewInstallAction {}

impl Default for HomebrewInstallAction {
  fn default() -> Self {
    Self::new()
  }
}

impl HomebrewInstallAction {
  /// Constructs a new [HomebrewInstallAction]
  pub fn new() -> Self {
    HomebrewInstallAction {}
  }
  /// Returns true if it can find a brew command
  pub fn check_brew_is_installed(&self) -> bool {
    Command::new("which")
      .arg("brew")
      .status()
      .unwrap()
      .success()
  }
}

impl Action<'_> for HomebrewInstallAction {
  fn execute(&self) -> Result<(), String> {
    if !self.check_brew_is_installed() {
      let result = Exec::shell("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
                .join().map_or_else(|_err|
                        Err(String::from("Error running homebrew installer")),
                        |status| match status  {
                            ExitStatus::Exited(0) => Ok(()),
                        ExitStatus::Exited(code) => Err(format!(
                            "Couldn't run homebrew installer, Error status {}",
                            code
                        )),
                        _ => Err(String::from("Unexpected error while running homebrew installer"))
                    }
                );
      #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
      return result;
      #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
      {
        result?;
        (Exec::shell("echo 'eval \"$(/opt/homebrew/bin/brew shellenv)\"' >> ~/.zprofile")
          | Exec::shell("echo 'eval \"$(/opt/homebrew/bin/brew shellenv)\"' >> ~/.bash_profile"))
        .join()
        .map_or_else(
          |_err| {
            Err(String::from(
              "couldn't set .zprofile and .bash_profile to use homebrew",
            ))
          },
          |status| match status {
            ExitStatus::Exited(0) => Ok(()),
            ExitStatus::Exited(code) => Err(format!(
              "couldn't set .zprofile and .bash_profile to use homebrew, status {}",
              code
            )),
            _ => Err(String::from(
              "Unexpected error while setting .zprofile and .bash_profile to use homebrew",
            )),
          },
        )
      }
    } else {
      info!("Homebrew already installed, no need to re-install");
      Ok(())
    }
  }
}
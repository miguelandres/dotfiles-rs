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

//! This module contains the [BrewAction] that installs
//! a brew formula using homebrew

#![cfg(unix)]
use crate::action::Action;
use log::info;
use std::marker::PhantomData;
use subprocess::Exec;
use subprocess::ExitStatus;

/// [BrewAction] Installs software using homebrew.
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
  fn execute(&self) -> Result<(), String> {
    for tap in &self.taps {
      info!("Tapping {}", tap);
      Exec::cmd("brew").arg("tap").arg(tap).join().map_or_else(
        |_err| Err(format!("Error tapping {}", tap)),
        |status| match status {
          ExitStatus::Exited(0) => Ok(()),
          ExitStatus::Exited(code) => Err(format!("Couldn't tap {}, Error status {}", tap, code)),
          _ => Err(format!("Unexpected error while tapping {}", tap)),
        },
      )?
    }
    for formula in &self.formulae {
      info!("Installing {} formula", formula);
      Exec::cmd("brew")
        .arg("install")
        .arg(formula)
        .join()
        .map_or_else(
          |_err| Err(format!("Error installing {}", formula)),
          |status| match status {
            ExitStatus::Exited(0) => Ok(()),
            ExitStatus::Exited(code) => Err(format!(
              "Couldn't install {}, Error status {}",
              formula, code
            )),
            _ => Err(format!("Unexpected error while installing {}", formula)),
          },
        )?
    }
    for cask in &self.casks {
      info!("Installing {} cask", cask);
      let mut cmd = Exec::cmd("brew").arg("install").arg("--cask");
      if self.force_casks {
        cmd = cmd.arg("--force");
      }
      cmd.arg(cask).join().map_or_else(
        |_err| Err(format!("Error installing {} cask", cask)),
        |status| match status {
          ExitStatus::Exited(0) => Ok(()),
          ExitStatus::Exited(code) => Err(format!(
            "Couldn't install {} cask, Error status {}",
            cask, code
          )),
          _ => Err(format!("Unexpected error while installing {} cask", cask)),
        },
      )?
    }
    Ok(())
  }
}

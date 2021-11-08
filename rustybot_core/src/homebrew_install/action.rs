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

//! This module contains the [HomebrewInstallAction] that installs
//! homebrew on a macOS computer

#![cfg(unix)]
use crate::action::Action;
use log::info;
use std::fs::File;
use std::io::copy;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

/// [HomebrewInstallAction] installs homebrew.
pub struct HomebrewInstallAction {}

impl Default for HomebrewInstallAction {
    fn default() -> Self {
        Self::new()
    }
}

impl HomebrewInstallAction {
    /// Constructs a new [HomebrewInstallAction]
    pub fn new() -> HomebrewInstallAction {
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
            let dir = tempfile::Builder::new()
                .prefix("install_homebrew")
                .tempdir()
                .map_err(|_err| {
                    String::from(
                        "Couldn't create temporary dir to download homebrew install script.",
                    )
                })?;
            let filename = dir.path().join("install.sh");
            let mut file = File::create(&filename).map_err(|_err| {
                format!("could not create temp file {}", filename.to_str().unwrap())
            })?;
            let target = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
            let response = reqwest::blocking::get(target)
                .map_err(|_err| "Error downloading Homebrew install script")?;
            let content = response
                .text()
                .map_err(|_err| "Error downloading Homebrew install script")?;
            copy(&mut content.as_bytes(), &mut file)
                .map_err(|_err| "Couldn't write script to file")?;
            file.set_permissions(PermissionsExt::from_mode(0o755))
                .unwrap();
            Command::new("/bin/bash")
                .arg(filename)
                .status()
                .map_or_else(
                    |_err| Err(String::from("Could not run homebrew installation")),
                    |status| match status.success() {
                        true => Ok(()),
                        false => Err(format!(
                            "Homebrew installation failed with status code {}",
                            status
                        )),
                    },
                )
        } else {
            info!("Homebrew already installed, no need to re-install");
            Ok(())
        }
    }
}

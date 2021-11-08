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
use crate::directive::Settings;
use crate::homebrew_install::directive::*;
use crate::yaml_util::get_boolean_setting;
use log::error;
use log::info;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

/// [HomebrewInstallAction] installs homebrew.
pub struct HomebrewInstallAction{

}

impl HomebrewInstallAction {
    /// Constructs a new [HomebrewInstallAction]
    pub fn new() -> HomebrewInstallAction {
        HomebrewInstallAction{}

    }
}

impl Action for HomebrewInstallAction {
    fn execute(&self) -> Result<(), String> {
        fn check_brew_is_installed() -> Result<bool, String> {
            Command::new("which")
                .arg("brew")
                .status()
                .and_then(|status| status == 0)
                .or_else(|err| Err("Could not run `which brew` to check whether brew is installed"))
        }

        if (check_brew_is_installed()?) {
            let dir = tempfile::Builder::new().prefix("install_homebrew").tempdir().or_else(|err| Err("Couldn't create temporary dir to download homebrew install script.")?
            let filename = dir.path().join("install.sh");
            let mut file = File::create(filename)?;
            let target = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
            let response = reqwest::get(target).await?;
            let content =  response.text().await?;
            copy(&mut content.as_bytes(), &mut file)?;
            let mut permissions = PermissionsExt::from_mode(0o755);
            file.set_permissions(permissions).unwrap();
            Command::new("/bin/bash")
                .arg(filename)
                .status()
                .and_then(|status|
                    if (status != 0) {
                      Err(format!("Homebrew installation failed with status code {}", status))
                    } else {
                        Ok(())
                    })
                .or_else(|err| Err("Could not run homebrew installation"))
        }
    }
}

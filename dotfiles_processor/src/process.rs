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

use std::convert::TryFrom;

use dotfiles_core::Action;

use dotfiles_actions::ohmyzsh_install::action::OhMyZshInstallAction;

use dotfiles_actions::homebrew_install::action::HomebrewInstallAction;
use log::info;

use crate::{
  context::Context,
  flags::{Command, FlagData},
};

use dotfiles_core::error::DotfilesError;

pub fn process(flag_data: &FlagData) -> Result<(), DotfilesError> {
  match &flag_data.command {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    Command::InstallHomebrew => {
      info!("Installing homebrew.");
      HomebrewInstallAction::new().execute()
    }
    #[cfg(unix)]
    Command::InstallOhMyZsh { skip_chsh } => {
      info!("Installing Oh My Zsh!");
      OhMyZshInstallAction::new(skip_chsh.to_owned()).execute()
    }
    Command::ApplyConfig { file, dry_run } => {
      let mut ctx = Context::try_from(file.as_str())?;
      ctx.parse_file()?;
      if !dry_run {
        Context::run_actions(ctx)?;
      }
      Ok(())
    }
  }
}

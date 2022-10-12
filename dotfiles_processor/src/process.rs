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

use dotfiles_actions::brew::directive::BrewDirective;
use dotfiles_actions::create::directive::new_native_create_directive;
use dotfiles_actions::exec::directive::ExecDirective;
use dotfiles_actions::link::directive::new_native_link_directive;
use dotfiles_core::directive::DirectiveSet;
use dotfiles_core::Action;

use dotfiles_actions::ohmyzsh_install::action::OhMyZshInstallAction;

use dotfiles_actions::homebrew_install::action::HomebrewInstallAction;

use crate::flags::{Command, FlagData};

use dotfiles_core::error::DotfilesError;

pub fn process(flag_data: &FlagData) -> Result<(), DotfilesError> {
  let mut directive_set: DirectiveSet = Default::default();
  initialize_directive_set(&mut directive_set)?;

  match &flag_data.command {
    Command::InstallHomebrew => HomebrewInstallAction::new().execute()?,
    Command::InstallOhMyZsh { skip_chsh } => {
      OhMyZshInstallAction::new(skip_chsh.to_owned()).execute()?
    }
    Command::ApplyConfig { file: _ } => todo!(),
  };
  Ok(())
}

pub fn initialize_directive_set<'a>(
  directive_set: &'a mut DirectiveSet,
) -> Result<(), DotfilesError> {
  directive_set.add("brew", Box::new(BrewDirective::new()))?;
  directive_set.add("create", Box::new(new_native_create_directive()))?;
  directive_set.add("exec", Box::new(ExecDirective::new()))?;
  directive_set.add("link", Box::new(new_native_link_directive()))?;
  Ok(())
}

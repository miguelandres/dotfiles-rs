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

mod flags;

use dotfiles_actions::brew::directive::BrewDirective;
use dotfiles_actions::create::directive::new_native_create_directive;
use dotfiles_actions::exec::directive::ExecDirective;
use dotfiles_actions::homebrew_install::action::HomebrewInstallAction;
use dotfiles_actions::link::directive::new_native_link_directive;
use dotfiles_actions::ohmyzsh_install::action::OhMyZshInstallAction;
use dotfiles_core::directive::DirectiveSet;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::Action;
use flags::FlagParser;
use simplelog::*;

fn process() -> Result<(), DotfilesError> {
  let flag_parser = FlagParser::new();
  let flags_vec: Vec<String> = std::env::args().collect();
  let flag_data = flag_parser.parse_flags(&flags_vec[1..])?;
  if flag_data.install_homebrew {
    HomebrewInstallAction::new().execute()?;
  }
  if flag_data.install_ohmyzsh {
    OhMyZshInstallAction::new(flag_data.skip_chsh).execute()?;
  }
  let mut directive_set: DirectiveSet = Default::default();
  directive_set.add("brew", Box::new(BrewDirective::new()))?;
  directive_set.add("create", Box::new(new_native_create_directive()))?;
  directive_set.add("exec", Box::new(ExecDirective::new()))?;
  directive_set.add("link", Box::new(new_native_link_directive()))?;

  Ok(())
}

fn main() {
  CombinedLogger::init(vec![TermLogger::new(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )])
  .unwrap();
  match process() {
    Ok(_) => log::info!("Process completed successfully"),
    Err(error) => {
      log::error!("Processing failed: {:?}", error);
      std::process::exit(1);
    }
  }
}

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

mod flags;
use flags::FlagParser;
use log;
use rustybot_core::homebrew_install::action::HomebrewInstallAction;
use rustybot_core::ohmyzsh_install::action::OhMyZshInstallAction;
use rustybot_core::Action;
use simplelog::*;

fn process() -> Result<(), String> {
    let flag_parser = FlagParser::new();
    let flags_vec: Vec<String> = std::env::args().collect();
    let flag_data = flag_parser.parse_flags(&flags_vec[1..])?;
    if flag_data.install_homebrew {
        HomebrewInstallAction::new().execute()?;
    }
    if flag_data.install_ohmyzsh {
        OhMyZshInstallAction::new(flag_data.skip_chsh).execute()?;
    }
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
            log::error!("Processing failed: {}", error);
            std::process::exit(1);
        }
    }
}

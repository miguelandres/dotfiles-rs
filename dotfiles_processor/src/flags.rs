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

use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;

#[derive(Parser)]
#[command(author)]
#[command(name = "dotfiles")]
#[command(bin_name = "dotfiles")]
#[command(
  about = "A simple dotfile manager that can be configured using yaml. See https://github.com/miguelandres/dotfiles-rs"
)]
pub struct FlagData {
  #[command(subcommand)]
  pub command: Command,
  #[arg(value_enum, long, default_value_t = LogLevelFilter::Info)]
  pub log_level_filter: LogLevelFilter,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevelFilter {
  /// A level lower than all log levels.
  Off,
  /// Corresponds to the `Error` log level.
  Error,
  /// Corresponds to the `Warn` log level.
  Warn,
  /// Corresponds to the `Info` log level.
  Info,
  /// Corresponds to the `Debug` log level.
  Debug,
  /// Corresponds to the `Trace` log level.
  Trace,
}

// Exception to this clippy since LogLevelFilter is in a crate I don't own.
#[allow(clippy::from_over_into)]
impl Into<LevelFilter> for LogLevelFilter {
  fn into(self) -> LevelFilter {
    match self {
      LogLevelFilter::Off => LevelFilter::Off,
      LogLevelFilter::Error => LevelFilter::Error,
      LogLevelFilter::Warn => LevelFilter::Warn,
      LogLevelFilter::Info => LevelFilter::Info,
      LogLevelFilter::Debug => LevelFilter::Debug,
      LogLevelFilter::Trace => LevelFilter::Trace,
    }
  }
}

#[derive(Subcommand)]
pub enum Command {
  /// Installs Homebrew on Mac or Linuxbrew on Linux, see http://brew.sh
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  InstallHomebrew,
  /// Installs Oh My Zsh! and sets `zsh` as the shell, see https://ohmyz.sh/
  #[cfg(unix)]
  InstallOhMyZsh {
    /// Skips running `chsh` to set `zsh` as the shell
    #[arg(long = "skip-chsh", default_value_t = false)]
    skip_chsh: bool,
  },
  /// Applies the configuration in the file passed as argument.
  #[command(arg_required_else_help = true)]
  ApplyConfig {
    /// Configuration file that describes what to do.
    file: String,
    /// Only parse the configuration, do not run it.
    #[arg(short = 'n', long = "dry-run", default_value_t = false)]
    dry_run: bool,
  },
}

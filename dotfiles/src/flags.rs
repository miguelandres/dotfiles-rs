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

use itertools::Itertools;
use std::collections::BTreeSet;

pub struct FlagData {
  pub install_homebrew: bool,
  pub install_ohmyzsh: bool,
  pub skip_chsh: bool,
}

pub struct FlagParser {
  homebrew_install_flag_names: BTreeSet<String>,
  ohmyzsh_install_flag_names: BTreeSet<String>,
  skip_chsh_flag_names: BTreeSet<String>,
  all_flag_names: BTreeSet<String>,
}

impl FlagParser {
  pub fn new() -> FlagParser {
    let homebrew_install_flag_names: BTreeSet<String> = [
      "--install_homebrew",
      "--install_brew",
      "--install-homebrew",
      "--install-brew",
    ]
    .iter()
    .copied()
    .map(String::from)
    .collect();
    let skip_chsh_flag_names: BTreeSet<String> = ["--skip-chsh", "--skip_chsh"]
      .iter()
      .copied()
      .map(String::from)
      .collect();

    let ohmyzsh_install_flag_names: BTreeSet<String> =
      ["--install_ohmyzsh", "--install-ohmyzsh", "--ohmyzsh"]
        .iter()
        .copied()
        .map(String::from)
        .collect();

    let all_flag_names: BTreeSet<String> = homebrew_install_flag_names
      .union(&ohmyzsh_install_flag_names)
      .cloned()
      .collect::<BTreeSet<String>>()
      .union(&skip_chsh_flag_names)
      .cloned()
      .collect();

    FlagParser {
      homebrew_install_flag_names,
      ohmyzsh_install_flag_names,
      skip_chsh_flag_names,
      all_flag_names,
    }
  }

  fn check_flags_are_valid(&self, flags: &[String]) -> Result<(), String> {
    let mut invalid_flags: Vec<&str> = Vec::new();
    for flag in flags.iter() {
      if !self.all_flag_names.contains(flag) {
        invalid_flags.push(flag);
      }
    }
    match invalid_flags.len() {
      0 => Ok(()),
      _ => Err(format!(
        "Unrecognized flag(s): {}\n\n  Hint: valid flags are {}",
        invalid_flags.join(", "),
        self.all_flag_names.iter().format(", ")
      )),
    }
  }

  pub fn parse_flags(&self, flags: &[String]) -> Result<FlagData, String> {
    self.check_flags_are_valid(flags)?;
    let mut flag_data = FlagData {
      install_homebrew: false,
      install_ohmyzsh: false,
      skip_chsh: false,
    };
    for flag in flags.iter() {
      if self.homebrew_install_flag_names.contains(flag) {
        flag_data.install_homebrew = true;
      }
      if self.ohmyzsh_install_flag_names.contains(flag) {
        flag_data.install_ohmyzsh = true;
      }
      if self.skip_chsh_flag_names.contains(flag) {
        flag_data.skip_chsh = true;
      }
    }
    Ok(flag_data)
  }
}

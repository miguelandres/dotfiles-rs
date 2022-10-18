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

//! Contains helpful functions to deal with paths in the context of parsing them for dotfiles
//! configs.

#[cfg(unix)]
use home::home_dir;
#[cfg(unix)]
use std::path::Component;
use std::path::{Path, PathBuf};

/// Checks for ~ and replaces it with a home directory if necessary.
pub fn process_home_dir_in_path(value: &Path) -> PathBuf {
  #[cfg(unix)]
  if let Some(Component::Normal(component)) = value.components().next() {
    if component == "~" {
      // Starts with ~/, should be replaced by home directory
      let mut new_path = home_dir().unwrap();
      let rest_of_path: PathBuf = value.components().skip(1).collect();
      new_path.push(rest_of_path);

      return new_path;
    }
  }
  value.to_owned()
}

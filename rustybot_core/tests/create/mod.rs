#![cfg(test)]

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

mod action;
mod directive;

use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use std::io;

fn setup_fs_internal(fs: &FakeFileSystem) -> io::Result<()> {
  fs.create_dir_all("/home/user/")?;
  fs.create_dir("/system")?;
  fs.set_readonly("/system", true)?;
  fs.set_current_dir("/home/user/")?;
  Ok(())
}

pub fn setup_fs(fs: &FakeFileSystem) -> Result<(), String> {
  if let Err(io_error) = setup_fs_internal(fs) {
    return Err(io_error.to_string());
  }
  Ok(())
}

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

use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use rustybot_core::create::action::*;
use rustybot_core::directive::*;
use rustybot_core::new::directive::*;
use std::io;

fn setup_fs_internal(fs: &FakeFileSystem) -> io::Result<()> {
    fs.create_dir_all("/home/user/")?;
    fs.create_dir("/system")?;
    fs.set_readonly("/system", true)?;
    fs.set_current_dir("/home/user/")?;
    Ok(())
}

fn setup_fs(fs: &FakeFileSystem) -> Result<(), String> {
    if let Err(io_error) = setup_fs(fs) {
        return Err(io_error.to_string());
    }
    Ok(())
}

#[test]
fn create_dir_fails_on_nonexistent_path() -> Result<(), String> {
    let err_fmt = "Could create directory in nonexistent path, {}";
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    let action = CreateAction::new(
        &fs,
        String::from("/home/user/nonexistent_path/target"),
        false,
    );
    check_action_fail(action, format!(err_fmt, action.directory()))?
}

#[test]
fn create_dir_succeeds() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs)?;
    let action = CreateAction::new(&fs, String::from("/home/user/target"), false);
    action.execute()?
}

#[test]
fn create_dir_fails_on_readonly_dir() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs)?;
    let action = CreateAction::new(&fs, String::from("/system/target"), false);
    let err_fmt="Could create directory in readonly path {}";
    check_action_fail(action, format!(err_fmt, action.directory()))?
}

#[test]
fn force_create_dir_succeeds_on_nonexistent_path() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs)?;
    let action = CreateAction::new(
        &fs,
        String::from("/home/user/nonexistent_path/target"),
        true,
    );
    action.execute()?
}

#[test]
fn force_create_dir_succeeds() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs)?;
    let action = CreateAction::new(&fs, String::from("/home/user/target"), true);
    action.execute()?
}

#[test]
fn force_create_dir_fails_on_readonly_dir() -> Result<(), io::Error> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs)?;
    let err_fmt="Could create directory in readonly path {}";
    let action = CreateAction::new(&fs, String::from("/system/target"), true);
    check_action_fail(&action,format!(err_fmt, action.directory()))?
}

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

use crate::utils::check_action_fail;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::UnixFileSystem;

use rustybot_core::action::Action;
use rustybot_core::directive::initialize_settings_object;
use rustybot_core::directive::Setting;
use rustybot_core::link::action::LinkAction;

use rustybot_core::directive::Settings;
use rustybot_core::link::directive::*;

use std::io;
use std::path::PathBuf;

fn setup_fs_internal<F: FileSystem + UnixFileSystem>(fs: &F) -> io::Result<()> {
    fs.create_dir_all("/home/user/")?;
    fs.create_dir("/system")?;
    fs.set_readonly("/system", true)?;
    fs.set_current_dir("/home/user/")?;
    Ok(())
}

fn setup_fs<F: FileSystem + UnixFileSystem>(fs: &F) -> Result<(), String> {
    if let Err(io_error) = setup_fs_internal(fs) {
        return Err(io_error.to_string());
    }
    Ok(())
}

#[test]
fn link_fails_on_nonexistent_path() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    let settings = Settings::new();
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/nonexistent_path/path"),
        String::from("/home/user/target"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {} pointing to {}, this shouldn't happen since the path doesn't exist",
            action.path(),
            action.target()
        ),
    )
}

#[test]
fn link_succeeds_on_nonexistent_path_with_create_parent_dirs() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    let settings = initialize_settings_object(&[(
        CREATE_PARENT_DIRS_SETTING.to_string(),
        Setting::Boolean(true),
    )]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/nonexistent_path/path"),
        String::from("/home/user/target"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_fails_on_readonly_dir() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/readonly").unwrap();
    fs.set_readonly("/home/user/readonly", true).unwrap();
    let settings = Settings::new();
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/readonly/some"),
        String::from("/home/user"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {}, this shouldn't happen since it's in a readonly dor",
            action.path(),
        ),
    )
}

#[test]
fn link_fails_on_nonexistent_target() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    let settings = Settings::new();
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {} pointing to {}, this shouldn't happen since the target doesn't exist",
            action.path(),
            action.target()
        ),
    )
}

#[test]
fn link_succeeds_on_nonexistent_target_if_ignoring_missing_target() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    let settings = initialize_settings_object(&[(
        String::from(IGNORE_MISSING_TARGET_SETTING),
        Setting::Boolean(true),
    )]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_fails_on_existing_link() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.symlink("/home/user/target", "/home/user/path").unwrap();
    let settings = Settings::new();
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {} pointing to {}, this shouldn't happen since the {} already exists as a symlink to {}",
            action.path(),
            action.target(),
            action.path(),
            fs.get_symlink_src(action.path()).unwrap().to_str().unwrap()
        ),
    )
}

#[test]
fn link_succeeds_on_existing_link_with_relink() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.symlink("/home/user/target", "/home/user/path").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(RELINK_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_fails_on_existing_file_with_relink() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.create_file("/home/user/path", "").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(RELINK_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {} pointing to {}, this shouldn't happen since the {} already exists as a file",
            action.path(),
            action.target(),
            action.path(),
        ),
    )
}

#[test]
fn link_fails_on_existing_dir_with_relink() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.create_dir("/home/user/path").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(RELINK_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    check_action_fail(
        &action,
        format!(
            "Could create {} pointing to {}, this shouldn't happen since the {} already exists as a dir",
            action.path(),
            action.target(),
            action.path(),
        ),
    )
}

#[test]
fn link_succeeds_on_existing_link_with_force() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.symlink("/home/user/target", "/home/user/path").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(FORCE_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_fails_on_existing_file_with_force() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.create_file("/home/user/path", "").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(FORCE_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_fails_on_existing_dir_with_force() -> Result<(), String> {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.create_dir("/home/user/target2").unwrap();
    fs.create_dir("/home/user/path").unwrap();
    let settings =
        initialize_settings_object(&[(String::from(FORCE_SETTING), Setting::Boolean(true))]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/target2"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute()
}

#[test]
fn link_resolves_symlink_target() {
    let fs = FakeFileSystem::new();
    setup_fs(&fs).expect("Failure setting up FakeFileSystem");
    fs.create_dir("/home/user/target").unwrap();
    fs.symlink("/home/user/target", "/home/user/symlink")
        .unwrap();
    let settings = initialize_settings_object(&[(
        RESOLVE_SYMLINK_TARGET_SETTING.to_string(),
        Setting::Boolean(true),
    )]);
    let action = LinkAction::new(
        &fs,
        String::from("/home/user/path"),
        String::from("/home/user/symlink"),
        &settings,
        init_directive_data().defaults(),
    );
    action.execute().unwrap();
    let source = fs.get_symlink_src("/home/user/path").unwrap();
    assert!(source == PathBuf::from("/home/user/target"));
}

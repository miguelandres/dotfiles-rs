#![cfg(test)]

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

use crate::utils::check_action_fail;
use crate::utils::setup_fs;
use dotfiles_actions::link::action::LinkAction;
use dotfiles_actions::link::directive::init_directive_data;
use dotfiles_actions::link::directive::CREATE_PARENT_DIRS_SETTING;
use dotfiles_actions::link::directive::FORCE_SETTING;
use dotfiles_actions::link::directive::IGNORE_MISSING_TARGET_SETTING;
use dotfiles_actions::link::directive::RELINK_SETTING;
use dotfiles_actions::link::directive::RESOLVE_SYMLINK_TARGET_SETTING;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::settings::initialize_settings_object;
use dotfiles_core::settings::Setting;
use dotfiles_core::Action;
use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use filesystem::UnixFileSystem;

use dotfiles_core::settings::Settings;

use std::path::PathBuf;

#[test]
fn link_fails_on_nonexistent_path() -> Result<(), DotfilesError> {
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
fn link_succeeds_on_nonexistent_path_with_create_parent_dirs() -> Result<(), DotfilesError> {
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
fn link_fails_on_readonly_dir() -> Result<(), DotfilesError> {
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
fn link_fails_on_nonexistent_target() -> Result<(), DotfilesError> {
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
fn link_succeeds_on_nonexistent_target_if_ignoring_missing_target() -> Result<(), DotfilesError> {
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
fn link_fails_on_existing_link() -> Result<(), DotfilesError> {
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
fn link_succeeds_on_existing_link_with_relink() -> Result<(), DotfilesError> {
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
fn link_fails_on_existing_file_with_relink() -> Result<(), DotfilesError> {
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
fn link_fails_on_existing_dir_with_relink() -> Result<(), DotfilesError> {
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
fn link_succeeds_on_existing_link_with_force() -> Result<(), DotfilesError> {
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
fn link_fails_on_existing_file_with_force() -> Result<(), DotfilesError> {
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
fn link_fails_on_existing_dir_with_force() -> Result<(), DotfilesError> {
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

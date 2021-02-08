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

mod utils;
use utils::*;

fn setup_defaults(force: bool) -> Settings {
  let mut settings = Settings::new();
  settings.insert("force", Setting::Boolean(force));
  settings
}

static YAML_NONEXISTENT_PATH: &str = "/home/user/nonexistent_path/target";
static YAML_NONEXISTENT_PATH_FORCE: &str = "dir: /home/user/nonexistent_path/target
force: true";
static YAML_NONEXISTENT_PATH_NO_FORCE: &str = "dir: /home/user/nonexistent_path/target
force: false";

#[test]
fn create_dir_fails_on_nonexistent_path() -> Result<(), io::Error> {
  let err_fmt = "Could create directory in nonexistent path, {}";
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let settings = setup_defaults(false);
  let directive = CreateDirective::new(fs);
  let action = directive.get_action(settings, YAML_NONEXISTENT_PATH);
  check_action_fail(action, format!(err_fmt, action.directory()))?;
  let action = directive.get_action(settings, YAML_NONEXISTENT_PATH_NO_FORCE);
  check_action_fail(action, format!(err_fmt, action.directory()))?;
  Ok(())
}

#[test]
fn create_dir_force_succeeds_on_nonexistent_path() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let settings = setup_defaults(true);
  let directive = CreateDirective::new(fs);
  let action = directive.get_action(settings, YAML_NONEXISTENT_PATH);
  action.execute()?;
  let action = directive.get_action(settings, YAML_NONEXISTENT_PATH_NO_FORCE);
  action.execute()?;
  let action = directive.get_action(settings, YAML_NONEXISTENT_PATH_FORCE);
  action.execute()?
}

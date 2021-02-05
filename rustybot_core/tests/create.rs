#![cfg(test)]

use filesystem::FakeFileSystem;
use filesystem::FileSystem;
use rustybot_core::create::*;
use rustybot_core::directive::*;
use std::io;

trait CreateFake {
  fn new_fake() -> Self;
}
impl CreateFake for CreateDirective<FakeFileSystem> {
  fn new_fake() -> Self {
    CreateDirective::<FakeFileSystem>::create(FakeFileSystem::new())
  }
}

fn setup_fs(fs: &FakeFileSystem) -> io::Result<()> {
  fs.create_dir_all("/home/user/")?;
  fs.create_dir("/system")?;
  fs.set_readonly("/system", true)?;
  fs.set_current_dir("/home/user/")?;
  Ok(())
}

#[test]
fn create_dir_fails_on_nonexistent_path() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/home/user/nonexistent_path/target"),
    false);
  if let Ok(()) = action.execute() {
    panic!(
      "Could create directory in nonexistent path {}",
      action.directory()
    )
  }
  Ok(())
}

#[test]
fn create_dir_succeeds() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/home/user/target"),
    false);
  if let Err(e) = action.execute() {
    panic!(
      "Could not create directory path {}: {:?}",
      action.directory(), e
    )
  }
  Ok(())
}

#[test]
fn create_dir_fails_on_readonly_dir() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/system/target"),
    false);
  if let Ok(()) = action.execute() {
    panic!(
      "Could create directory in readonly path {}",
      action.directory()
    )
  }
  Ok(())
}

#[test]
fn force_create_dir_succeeds_on_nonexistent_path() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/home/user/nonexistent_path/target"),
    true);
  if let Err(e) = action.execute() {
    panic!(
      "Couldn't force create directory in nonexistent path {}: {:?}",
      action.directory(), e
    )
  }
  Ok(())
}

#[test]
fn force_create_dir_succeeds() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/home/user/target"),
    true);
  if let Err(e) = action.execute() {
    panic!(
      "Could not create directory path {}: {:?}",
      action.directory(), e
    )
  }
  Ok(())
}

#[test]
fn force_create_dir_fails_on_readonly_dir() -> Result<(), io::Error> {
  let fs = FakeFileSystem::new();
  setup_fs(&fs)?;
  let action = CreateAction::create(
    &fs,
    String::from("/system/target"),
    true);
  if let Ok(()) = action.execute() {
    panic!(
      "Could create directory in readonly path {}",
      action.directory()
    )
  }
  Ok(())
}

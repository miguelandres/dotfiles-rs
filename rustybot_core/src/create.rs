extern crate yaml_rust;

use crate::directive::DirectiveData;
use crate::directive;
use crate::directive::Action;
use crate::directive::Directive;
use crate::directive::Setting;
use crate::directive::Settings;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use log::error;
use log::info;
use std::io;
use yaml_rust::Yaml;

const DIRECTIVE_NAME: &str = "create";
const FORCE_SETTING: &str = "force";
const DIR_SETTING: &str = "dir";


pub fn init_directive_data() -> DirectiveData {
  let mut settings = Settings::new();
  settings.insert(String::from(FORCE_SETTING), Setting::Boolean(false));
  DirectiveData::new(DIRECTIVE_NAME,settings)
}
  
pub struct CreateDirective<F: FileSystem> {
  fs: Box<F>,
  data: DirectiveData
}

impl <F:FileSystem> CreateDirective<F> {
  pub fn fs(&self) -> &F {
    self.fs.as_ref()
  }
  
  pub fn create(fs: F) -> CreateDirective<F> {
    CreateDirective::<F> {
      fs: Box::new(fs),data: init_directive_data()
    }
  }
}

impl CreateDirective<OsFileSystem> {
  #[allow(dead_code)]
  fn new_native() -> CreateDirective<OsFileSystem> {
    CreateDirective::<OsFileSystem>::create(OsFileSystem::new())
  }
}

impl<F:FileSystem> Directive<'_, CreateAction<'_, F>> for CreateDirective<F> {
  fn name(&self) -> &str {
    self.data.name()
  }

  fn defaults(&self) -> &Settings {
    self.data.defaults()
  }

  fn get_action(
    &self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Box<dyn Action<'_> + '_>,String> {
    Ok(Box::from(CreateAction::<'_> {
      fs: self.fs(),
      force: directive::get_boolean_setting(FORCE_SETTING, &settings, self.defaults())?,
      directory: directive::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
    }))
  }
}

pub struct CreateAction<'a, F: FileSystem> {
  fs: &'a F,
  directory: String,
  force: bool,
}

impl <F:FileSystem> CreateAction<'_, F> {
  pub fn create(fs: &'_ F, directory: String, force: bool) -> CreateAction<'_, F> {
    CreateAction {
      fs,
      directory,force
    }
  }
  pub fn directory(&self) -> &str {
    &self.directory
  }
  pub fn force(&self) -> bool {
    self.force
  }
}

impl <F:FileSystem> Action<'_> for CreateAction<'_, F> {
  fn execute(&self) -> Result<(), String> {
    fn create_dir<F: FileSystem>(fs: &'_ F, directory: &str, force: bool) -> io::Result<()> {
      if force {
        Ok(fs.create_dir_all(&directory)?)
      } else {
        Ok(fs.create_dir(&directory)?)
      }
    }
    match create_dir(self.fs, &self.directory, self.force) {
      Ok(()) => {
        info!("Created directory {}", &self.directory);
        Ok(())
      }
      Err(s) => {
        error!("Couldn't create directory {}: {}", &self.directory, s);
        Err(s.to_string())
      }
    }
  }
}



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

//! This module defines [CreateDirective].
extern crate yaml_rust;

use crate::create::action::CreateAction;
use crate::directive::Directive;
use crate::directive::DirectiveData;
use crate::directive::Setting;
use crate::directive::Settings;
use crate::yaml_util;
use filesystem::FileSystem;
use filesystem::OsFileSystem;
use yaml_rust::Yaml;

/// Constant for the name of the `create` directive.
pub const DIRECTIVE_NAME: &str = "create";
/// Constant for the name of the [`force`](CreateAction::force) Setting
/// which forces to create all parent directories if necessary.
pub const FORCE_SETTING: &str = "force";
/// Constant for the name of the [`directory`](CreateAction::directory) Setting
/// which forces to create all parent directories if necessary.
pub const DIR_SETTING: &str = "dir";

/// Constructs a new [CreateDirective] using the real filesystem
pub fn new_native_create_directive() -> CreateDirective<OsFileSystem> {
    CreateDirective::new(OsFileSystem::new())
}

/// Initializes the default configuration for the [CreateDirective]
pub fn init_directive_data() -> DirectiveData {
    let mut settings = Settings::new();
    settings.insert(String::from(FORCE_SETTING), Setting::Boolean(false));
    DirectiveData::new(DIRECTIVE_NAME, settings)
}

pub struct CreateDirective<F: FileSystem> {
    fs: Box<F>,
    data: DirectiveData,
}

impl<F: FileSystem> CreateDirective<F> {
    pub fn fs(&self) -> &F {
        self.fs.as_ref()
    }

    pub fn new(fs: F) -> CreateDirective<F> {
        CreateDirective::<F> {
            fs: Box::new(fs),
            data: init_directive_data(),
        }
    }
}

impl<'a, F: FileSystem> Directive<'a, CreateAction<'a, F>> for CreateDirective<F> {
    fn name(&self) -> &str {
        self.data.name()
    }

    fn defaults(&self) -> &Settings {
        self.data.defaults()
    }

    fn build_action(
        &self,
        settings: &Settings,
        yaml: &Yaml,
    ) -> Result<CreateAction<'_, F>, String> {
        Ok(CreateAction::<'_, F>::new(
            self.fs(),
            yaml_util::get_string_content_or_keyed_value(yaml, Some(DIR_SETTING))?,
            yaml_util::get_boolean_setting(FORCE_SETTING, settings, self.defaults())?,
        ))
    }
}

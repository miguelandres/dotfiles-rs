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

//! This module contains the base trait for all [Directive] and all
//! necessary conveniences to allow for user-configuration of directive
//! defaults.

extern crate yaml_rust;

use crate::action::Action;
use std::collections::HashMap;
use yaml_rust::Yaml;

/// The Settings object is a hashmap of option names to a default value
pub type Settings = HashMap<String, Setting>;

/// Represents a value for a setting
#[derive(Clone, Debug)]
pub enum Setting {
    /// A boolean value for a setting
    Boolean(bool),
    /// A string value for a setting
    String(String),
}

/// Returns a Settings object from an array as a bit of syntactic sugar
pub fn initialize_settings_object(settings: &[(String, Setting)]) -> Settings {
    let settings_object: Settings = settings
        .iter()
        .map(|(name, setting)| (name.clone(), setting.clone()))
        .collect();
    settings_object
}

/// A struct that contains the default settings for a Directive and the
/// name it takes in configuration sources. The name must be unique.
///
/// These default settings can be configured by the user as well.
pub struct DirectiveData {
    /// Unique name of this directive.
    ///
    /// This name will be used in configuration sources to instantiate actions
    /// of this directive
    name: &'static str,
    /// Default settings for this directive.
    ///
    /// Any setting that is not in the defaults for a directive but is part of
    /// the corresponding Action struct is considered to be mandatory.
    defaults: Settings,
}
impl DirectiveData {
    /// Constructs a new directive from a name and a set of default settings.
    pub fn new(name: &'static str, defaults: Settings) -> DirectiveData {
        DirectiveData { name, defaults }
    }
    /// Returns the name of the directive
    pub fn name(&self) -> &str {
        self.name
    }
    /// Returns the collection of default settings.
    pub fn defaults(&self) -> &Settings {
        &self.defaults
    }
}

/// A parser for action steps, each directive represents a type of Action.
pub trait Directive<'a, A: Action<'a>> {
    /// Returns the name of the directive.
    fn name(&self) -> &str;
    /// Returns the defaults settings as configured.
    fn defaults(&self) -> &Settings;
    /// Builds an action from a Yaml configuration source and a set of
    /// default settings.
    ///
    /// Returns an Error containing a human readable string in case there
    /// was an issue building the action.
    fn build_action(&'a self, settings: &Settings, yaml: &Yaml) -> Result<A, String>;
}

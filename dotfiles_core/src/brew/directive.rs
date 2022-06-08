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

//! This module defines [BrewDirective].

extern crate yaml_rust;

use crate::action::Action;
use crate::brew::action::BrewAction;
use crate::directive::initialize_settings_object;
use crate::directive::Directive;
use crate::directive::DirectiveData;
use crate::directive::Setting;
use crate::directive::Settings;
use crate::yaml_util::*;
use std::marker::PhantomData;
use yaml_rust::Yaml;

/// Name of the link directive
pub const DIRECTIVE_NAME: &str = "fetch";
/// force casks to deal with previously installed apps
pub const FORCE_CASKS_SETTING: &str = "force_casks";

/// Create a new brew directive.
pub fn new_brew_directive<'a>() -> BrewDirective<'a> {
  BrewDirective::new()
}

/// Initialize the defaults for the BrewDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::new(
    DIRECTIVE_NAME,
    initialize_settings_object(&[(String::from(FORCE_CASKS_SETTING), Setting::Boolean(false))]),
  )
}

/// A directive that can build [BrewAction]s to install formulae, casks
pub struct BrewDirective<'a> {
  data: DirectiveData,
  phantom_data: PhantomData<&'a DirectiveData>,
}

/// Default for [BrewDirective]
impl<'a> Default for BrewDirective<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> BrewDirective<'a> {
  /// Create a new [BrewDirective]
  pub fn new() -> BrewDirective<'a> {
    BrewDirective::<'a> {
      data: init_directive_data(),
      phantom_data: PhantomData,
    }
  }

  /// Parse a brew action for the following yaml section.
  pub fn parse_brew_action(
    &'a self,
    context_settings: &Settings,
    yaml: &Yaml,
  ) -> Result<BrewAction<'a>, String> {
    match yaml {
      Yaml::Hash(hash) => {
        let force_casks = get_boolean_setting_from_yaml_or_defaults(
          FORCE_CASKS_SETTING,
          yaml,
          context_settings,
          self.data.defaults(),
        )?;
        let taps = hash
          .get(&Yaml::String(String::from("tap")))
          .map_or(Ok(Vec::new()), |vec| get_string_array(vec, "tap"))?;
        let formulae = hash
          .get(&Yaml::String(String::from("formula")))
          .map_or(Ok(Vec::new()), |vec| get_string_array(vec, "formula"))?;
        let casks = hash
          .get(&Yaml::String(String::from("cask")))
          .map_or(Ok(Vec::new()), |vec| get_string_array(vec, "cask"))?;
        Ok(BrewAction::new(force_casks, taps, formulae, casks))
      }
      _ => Err(format!(
        "Yaml passed to configure a Brew action is not a Hash, thus cannot be parsed: {:?}",
        yaml
      )),
    }
  }
}

impl<'a> Directive<'a> for BrewDirective<'a> {
  fn name(&self) -> &str {
    self.data.name()
  }

  fn defaults(&self) -> &Settings {
    self.data.defaults()
  }

  fn build_action(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Box<dyn Action<'a> + 'a>, String> {
    Ok(Box::new(self.parse_brew_action(settings, yaml)?))
  }
}

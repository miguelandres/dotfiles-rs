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
//! This module defines [BrewDirective].

extern crate strict_yaml_rust;

use crate::brew::action::BrewAction;
use dotfiles_core::action::ActionParser;
use dotfiles_core::action::SKIP_IN_CI_SETTING;
use dotfiles_core::directive::DirectiveData;
use dotfiles_core::error::add_directive_error_prefix;
use dotfiles_core::error::DotfilesError;
use dotfiles_core::settings::initialize_settings_object;
use dotfiles_core::settings::Setting;
use dotfiles_core::settings::Settings;
use dotfiles_core::yaml_util::*;
use dotfiles_core_macros::Directive;

use std::marker::PhantomData;
use std::path::Path;
use strict_yaml_rust::StrictYaml;

/// Name of the Brew directive
pub const DIRECTIVE_NAME: &str = "brew";
/// force casks to deal with previously installed apps
pub const FORCE_CASKS_SETTING: &str = "force_casks";

/// The string that identifies the list of taps to install
pub const TAP_SETTING: &str = "tap";
/// The string that identifies the list of formulae to install
pub const FORMULA_SETTING: &str = "formula";
/// The string that identifies the list of casks to install
pub const CASK_SETTING: &str = "cask";

/// Initialize the defaults for the BrewDirective.
pub fn init_directive_data() -> DirectiveData {
  DirectiveData::from(
    DIRECTIVE_NAME.into(),
    initialize_settings_object(&[
      (FORCE_CASKS_SETTING.to_owned(), Setting::Boolean(false)),
      (SKIP_IN_CI_SETTING.to_owned(), Setting::Boolean(false)),
    ]),
  )
}

/// A directive that can build [BrewAction]s to install formulae, casks
#[derive(Directive, Clone)]
pub struct BrewDirective<'a> {
  data: DirectiveData,
  phantom_data: PhantomData<&'a DirectiveData>,
}

impl<'a> Default for BrewDirective<'a> {
  fn default() -> BrewDirective<'a> {
    BrewDirective::<'a> {
      data: init_directive_data(),
      phantom_data: PhantomData,
    }
  }
}

impl<'a> ActionParser<'a> for BrewDirective<'a> {
  type ActionType = BrewAction<'a>;

  fn parse_action(
    &'a self,
    context_settings: &Settings,
    yaml: &StrictYaml,
    _: &Path,
  ) -> Result<BrewAction<'a>, DotfilesError> {
    let force_casks = get_boolean_setting_from_yaml_or_context(
      FORCE_CASKS_SETTING,
      yaml,
      context_settings,
      self.data.defaults(),
    )?;
    let skip_in_ci = get_boolean_setting_from_yaml_or_context(
      SKIP_IN_CI_SETTING,
      yaml,
      context_settings,
      self.data.defaults(),
    )?;
    let taps = get_optional_string_array_from_yaml_hash(TAP_SETTING, yaml)?;
    let formulae = get_optional_string_array_from_yaml_hash(FORMULA_SETTING, yaml)?;
    let casks = get_optional_string_array_from_yaml_hash(CASK_SETTING, yaml)?;

    Ok(BrewAction::new(
      skip_in_ci,
      force_casks,
      taps,
      formulae,
      casks,
    ))
  }

  /// Parse the list of actions from yaml, in this case it's only one action so
  /// this function only wraps [BrewDirective::parse_action]
  fn parse_action_list(
    &'a self,
    context_settings: &Settings,
    yaml: &StrictYaml,
    current_dir: &Path,
  ) -> Result<Vec<BrewAction<'a>>, DotfilesError> {
    Ok(vec![add_directive_error_prefix(
      self,
      self.parse_action(context_settings, yaml, current_dir),
    )?])
  }
}

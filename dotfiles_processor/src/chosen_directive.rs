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

use std::convert::TryFrom;

use clap::__macro_refs::once_cell::sync::Lazy;
use dotfiles_actions::{
  brew::{action::BrewAction, directive::BrewDirective},
  create::{action::NativeCreateAction, directive::NativeCreateDirective},
  exec::{action::ExecAction, directive::ExecDirective},
  link::{action::NativeLinkAction, directive::NativeLinkDirective},
};
use dotfiles_core::{
  action::ActionParser,
  directive::{DirectiveData, HasDirectiveData},
  error::{DotfilesError, ErrorType},
  Action, Settings,
};

static BREW: Lazy<BrewDirective<'static>> = Lazy::new(Default::default);
static CREATE: Lazy<NativeCreateDirective<'static>> = Lazy::new(Default::default);
static EXEC: Lazy<ExecDirective<'static>> = Lazy::new(Default::default);
static LINK: Lazy<NativeLinkDirective<'static>> = Lazy::new(Default::default);

#[derive(Clone)]
pub enum KnownDirective {
  Brew,
  Create,
  Exec,
  Link,
}

pub enum KnownAction<'a> {
  Brew(BrewAction<'a>),
  Create(NativeCreateAction<'a>),
  Exec(ExecAction<'a>),
  Link(NativeLinkAction<'a>),
}

impl<'a> From<BrewAction<'a>> for KnownAction<'a> {
  fn from(value: BrewAction<'a>) -> Self {
    KnownAction::Brew(value)
  }
}

impl<'a> From<NativeCreateAction<'a>> for KnownAction<'a> {
  fn from(value: NativeCreateAction<'a>) -> Self {
    KnownAction::Create(value)
  }
}

impl<'a> From<ExecAction<'a>> for KnownAction<'a> {
  fn from(value: ExecAction<'a>) -> Self {
    KnownAction::Exec(value)
  }
}

impl<'a> From<NativeLinkAction<'a>> for KnownAction<'a> {
  fn from(value: NativeLinkAction<'a>) -> Self {
    KnownAction::Link(value)
  }
}

impl<'a> KnownAction<'a> {
  pub fn execute(&'a self) -> Result<(), DotfilesError> {
    match self {
      KnownAction::Brew(action) => action.execute(),
      KnownAction::Create(action) => action.execute(),
      KnownAction::Exec(action) => action.execute(),
      KnownAction::Link(action) => action.execute(),
    }
  }
}

impl KnownDirective {
  pub fn data(&self) -> &DirectiveData {
    match self {
      KnownDirective::Brew => BREW.directive_data(),
      KnownDirective::Create => CREATE.directive_data(),
      KnownDirective::Exec => EXEC.directive_data(),
      KnownDirective::Link => LINK.directive_data(),
    }
  }
  pub fn parse_context_defaults(
    &self,
    defaults: &strict_yaml_rust::StrictYaml,
  ) -> Result<(String, Settings), DotfilesError> {
    Ok((
      self.data().name().clone(),
      self.data().parse_context_defaults(defaults)?,
    ))
  }

  pub fn parse_action_list<'a>(
    directive: KnownDirective,
    context_settings: &Settings,
    actions: &strict_yaml_rust::StrictYaml,
  ) -> Result<Vec<KnownAction<'a>>, DotfilesError> {
    match directive {
      KnownDirective::Brew => BREW
        .parse_action_list(context_settings, actions)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Create => CREATE
        .parse_action_list(context_settings, actions)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Exec => EXEC
        .parse_action_list(context_settings, actions)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Link => LINK
        .parse_action_list(context_settings, actions)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
    }
  }
}

impl TryFrom<&str> for KnownDirective {
  type Error = DotfilesError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "brew" => Ok(KnownDirective::Brew),
      "create" => Ok(KnownDirective::Create),
      "exec" => Ok(KnownDirective::Exec),
      "link" => Ok(KnownDirective::Link),
      _ => Err(DotfilesError::from(
        format!("Configuration refers to unknown directive `{}`", value),
        ErrorType::InconsistentConfigurationError,
      )),
    }
  }
}

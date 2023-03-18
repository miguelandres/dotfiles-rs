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
use std::path::PathBuf;

use clap::__macro_refs::once_cell::sync::Lazy;
#[cfg(target_os = "linux")]
use dotfiles_actions::apt::{action::AptAction, directive::AptDirective};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use dotfiles_actions::brew::{action::BrewAction, directive::BrewDirective};
use dotfiles_actions::create::{action::NativeCreateAction, directive::NativeCreateDirective};
use dotfiles_actions::exec::{action::ExecAction, directive::ExecDirective};
#[cfg(unix)]
use dotfiles_actions::link::{action::NativeLinkAction, directive::NativeLinkDirective};
use dotfiles_core::action::ActionParser;
use dotfiles_core::action::ConditionalAction;
use dotfiles_core::directive::{DirectiveData, HasDirectiveData};
use dotfiles_core::error::{DotfilesError, ErrorType};
use dotfiles_core::yaml_util::map_yaml_array;
use dotfiles_core::Settings;
use strict_yaml_rust::StrictYaml;

use crate::context::Context;

#[cfg(target_os = "linux")]
static APT: Lazy<AptDirective<'static>> = Lazy::new(Default::default);
#[cfg(any(target_os = "linux", target_os = "macos"))]
static BREW: Lazy<BrewDirective<'static>> = Lazy::new(Default::default);
static CREATE: Lazy<NativeCreateDirective<'static>> = Lazy::new(Default::default);
static EXEC: Lazy<ExecDirective<'static>> = Lazy::new(Default::default);
#[cfg(unix)]
static LINK: Lazy<NativeLinkDirective<'static>> = Lazy::new(Default::default);
static SUBCONFIG_DIRECTIVE_DATA: Lazy<DirectiveData> = Lazy::new(subconfig_directive_data);

fn subconfig_directive_data() -> DirectiveData {
  DirectiveData::from("subconfig".into(), Default::default())
}

#[derive(Clone)]
pub enum KnownDirective {
  #[cfg(target_os = "linux")]
  Apt,
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  Brew,
  Create,
  Exec,
  #[cfg(unix)]
  Link,
  Subconfig,
}

pub enum KnownAction<'a> {
  #[cfg(target_os = "linux")]
  Apt(AptAction<'a>),
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  Brew(BrewAction<'a>),
  Create(NativeCreateAction<'a>),
  Exec(ExecAction<'a>),
  #[cfg(unix)]
  Link(NativeLinkAction<'a>),
  Subconfig(Context),
}

#[cfg(target_os = "linux")]
impl<'a> From<AptAction<'a>> for KnownAction<'a> {
  fn from(value: AptAction<'a>) -> Self {
    KnownAction::Apt(value)
  }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
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

#[cfg(unix)]
impl<'a> From<NativeLinkAction<'a>> for KnownAction<'a> {
  fn from(value: NativeLinkAction<'a>) -> Self {
    KnownAction::Link(value)
  }
}

impl<'a> From<Context> for KnownAction<'a> {
  fn from(value: Context) -> Self {
    KnownAction::Subconfig(value)
  }
}

impl<'a> KnownAction<'a> {
  pub fn execute(self) -> Result<(), DotfilesError> {
    match self {
      #[cfg(target_os = "linux")]
      KnownAction::Apt(action) => action.check_conditions_and_execute().map_err(|mut err| {
        err.add_message_prefix("Executing apt action".into());
        err
      }),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownAction::Brew(action) => action.check_conditions_and_execute().map_err(|mut err| {
        err.add_message_prefix("Executing brew action".into());
        err
      }),
      KnownAction::Create(action) => action.check_conditions_and_execute().map_err(|mut err| {
        err.add_message_prefix("Executing create action".into());
        err
      }),
      KnownAction::Exec(action) => action.check_conditions_and_execute().map_err(|mut err| {
        err.add_message_prefix("Executing exec action".into());
        err
      }),
      #[cfg(unix)]
      KnownAction::Link(action) => action.check_conditions_and_execute().map_err(|mut err| {
        err.add_message_prefix("Executing link action".into());
        err
      }),
      KnownAction::Subconfig(subcontext) => Context::run_actions(subcontext),
    }
  }
}

impl KnownDirective {
  pub fn data(&self) -> &DirectiveData {
    match self {
      #[cfg(target_os = "linux")]
      KnownDirective::Apt => APT.directive_data(),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownDirective::Brew => BREW.directive_data(),
      KnownDirective::Create => CREATE.directive_data(),
      KnownDirective::Exec => EXEC.directive_data(),
      #[cfg(unix)]
      KnownDirective::Link => LINK.directive_data(),
      KnownDirective::Subconfig => &SUBCONFIG_DIRECTIVE_DATA,
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
    context: &Context,
  ) -> Result<Vec<KnownAction<'a>>, DotfilesError> {
    let file = context.file();
    let current_dir = file.parent().ok_or_else(||
      DotfilesError::from(
        format!(
          "{} doesn't seem to have a parent dir to use as a current directory to parse actions, this makes no sense and should never happen",
          file.to_str().unwrap()) ,
      ErrorType::CoreError))?;
    match directive {
      #[cfg(target_os = "linux")]
      KnownDirective::Apt => APT
        .parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownDirective::Brew => BREW
        .parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Create => CREATE
        .parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Exec => EXEC
        .parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      #[cfg(unix)]
      KnownDirective::Link => LINK
        .parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Subconfig => map_yaml_array(actions, |file_yaml| {
        file_yaml
          .clone()
          .into_string()
          .ok_or(DotfilesError::from_wrong_yaml(
            "Subconfig: Expected a file name".to_owned(),
            actions.clone(),
            StrictYaml::String("".into()),
          ))
          .and_then(|filename| {
            let path = PathBuf::from(&filename);
            let mut subcontext = context.subcontext(&path)?;
            subcontext.parse_file()?;
            Ok(KnownAction::from(subcontext))
          })
      }),
    }
  }
}

impl TryFrom<&str> for KnownDirective {
  type Error = DotfilesError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      #[cfg(target_os = "linux")]
      "apt" => Ok(KnownDirective::Apt),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      "brew" => Ok(KnownDirective::Brew),
      "create" => Ok(KnownDirective::Create),
      "exec" => Ok(KnownDirective::Exec),
      "link" => Ok(KnownDirective::Link),
      "subconfig" => Ok(KnownDirective::Subconfig),
      _ => Err(DotfilesError::from(
        format!("Configuration refers to unknown directive `{value}`"),
        ErrorType::InconsistentConfigurationError,
      )),
    }
  }
}

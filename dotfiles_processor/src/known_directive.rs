// Copyright (c) 2021-2026 Miguel Barreto and others
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

#[cfg(target_os = "linux")]
use dotfiles_actions::apt::action::AptAction;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use dotfiles_actions::brew::action::BrewAction;
use dotfiles_actions::create::action::NativeCreateAction;
use dotfiles_actions::exec::action::ExecAction;
#[cfg(unix)]
use dotfiles_actions::link::action::NativeLinkAction;

use dotfiles_core::error::{DotfilesError, ErrorType};
use dotfiles_core::yaml_util::map_yaml_array;
use dotfiles_core::{Action, Settings};
use strict_yaml_rust::StrictYaml;

use crate::context::Context;

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

pub enum KnownAction {
  #[cfg(target_os = "linux")]
  Apt(AptAction),
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  Brew(BrewAction),
  Create(NativeCreateAction),
  Exec(ExecAction),
  #[cfg(unix)]
  Link(NativeLinkAction),
  Subconfig(Context),
}

#[cfg(target_os = "linux")]
impl From<AptAction> for KnownAction {
  fn from(value: AptAction) -> Self {
    KnownAction::Apt(value)
  }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl From<BrewAction> for KnownAction {
  fn from(value: BrewAction) -> Self {
    KnownAction::Brew(value)
  }
}

impl From<NativeCreateAction> for KnownAction {
  fn from(value: NativeCreateAction) -> Self {
    KnownAction::Create(value)
  }
}

impl From<ExecAction> for KnownAction {
  fn from(value: ExecAction) -> Self {
    KnownAction::Exec(value)
  }
}

#[cfg(unix)]
impl From<NativeLinkAction> for KnownAction {
  fn from(value: NativeLinkAction) -> Self {
    KnownAction::Link(value)
  }
}

impl From<Context> for KnownAction {
  fn from(value: Context) -> Self {
    KnownAction::Subconfig(value)
  }
}

impl KnownAction {
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
  pub fn name(&self) -> &'static str {
    match self {
      #[cfg(target_os = "linux")]
      KnownDirective::Apt => "apt",
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownDirective::Brew => "brew",
      KnownDirective::Create => "create",
      KnownDirective::Exec => "exec",
      #[cfg(unix)]
      KnownDirective::Link => "link",
      KnownDirective::Subconfig => "subconfig",
    }
  }

  pub fn default_settings(&self) -> Settings {
    match self {
      #[cfg(target_os = "linux")]
      KnownDirective::Apt => dotfiles_actions::apt::action::default_settings(),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownDirective::Brew => dotfiles_actions::brew::action::default_settings(),
      KnownDirective::Create => dotfiles_actions::create::action::default_settings(),
      KnownDirective::Exec => dotfiles_actions::exec::action::default_settings(),
      #[cfg(unix)]
      KnownDirective::Link => dotfiles_actions::link::action::default_settings(),
      KnownDirective::Subconfig => Settings::new(),
    }
  }

  pub fn parse_context_defaults(
    &self,
    defaults: &strict_yaml_rust::StrictYaml,
  ) -> Result<(String, Settings), DotfilesError> {
    Ok((
      self.name().to_owned(),
      dotfiles_core::yaml_util::parse_context_defaults(
        self.name(),
        &self.default_settings(),
        defaults,
      )?,
    ))
  }

  pub fn parse_action_list(
    directive: KnownDirective,
    context_settings: &Settings,
    actions: &strict_yaml_rust::StrictYaml,
    context: &Context,
  ) -> Result<Vec<KnownAction>, DotfilesError> {
    let file = context.file();
    let current_dir = file.parent().ok_or_else(||
      DotfilesError::from(
        format!(
          "{} doesn't seem to have a parent dir to use as a current directory to parse actions, this makes no sense and should never happen",
          file.to_str().unwrap()) ,
      ErrorType::CoreError))?;
    match directive {
      #[cfg(target_os = "linux")]
      KnownDirective::Apt => dotfiles_actions::apt::action::parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      KnownDirective::Brew => dotfiles_actions::brew::action::parse_action_list(context_settings, actions, current_dir)
        .map(|list| list.into_iter().map(KnownAction::from).collect()),
      KnownDirective::Create => {
        dotfiles_core::yaml_util::map_yaml_array(actions, |yaml_item| {
          dotfiles_actions::create::action::parse_action::<dotfiles_actions::filesystem::OsFileSystem>(
            dotfiles_actions::filesystem::OsFileSystem::default(),
            context_settings,
            yaml_item,
            current_dir,
          )
        })
        .map(|list| list.into_iter().map(KnownAction::from).collect())
      }
      KnownDirective::Exec => {
        dotfiles_core::yaml_util::map_yaml_array(actions, |yaml_item| {
          dotfiles_actions::exec::action::parse_action(context_settings, yaml_item, current_dir)
        })
        .map(|list| list.into_iter().map(KnownAction::from).collect())
      }
      #[cfg(unix)]
      KnownDirective::Link => {
        dotfiles_core::yaml_util::map_yaml_array(actions, |yaml_item| {
          dotfiles_actions::link::action::parse_action::<dotfiles_actions::filesystem::OsFileSystem>(
            dotfiles_actions::filesystem::OsFileSystem::default(),
            context_settings,
            yaml_item,
            current_dir,
          )
        })
        .map(|list| list.into_iter().map(KnownAction::from).collect())
      }
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
        format!("Configuration refers to unknown action type `{value}`"),
        ErrorType::InconsistentConfigurationError,
      )),
    }
  }
}

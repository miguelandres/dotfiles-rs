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

use dotfiles_actions::{
  brew::directive::BrewDirective, create::directive::NativeCreateDirective,
  exec::directive::ExecDirective, link::directive::NativeLinkDirective,
};
use dotfiles_core::{
  directive::{DirectiveData, HasDirectiveData},
  error::{DotfilesError, ErrorType},
  Settings,
};

pub enum KnownDirective<'a> {
  Brew(BrewDirective<'a>),
  Create(NativeCreateDirective<'a>),
  Exec(ExecDirective<'a>),
  Link(NativeLinkDirective<'a>),
}

pub struct ChosenDirective<'a> {
  name: String,
  chosen: Option<KnownDirective<'a>>,
}

impl<'a> From<&str> for ChosenDirective<'a> {
  fn from(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      chosen: match name {
        "brew" => Some(KnownDirective::Brew(Default::default())),
        "create" => Some(KnownDirective::Create(Default::default())),
        "exec" => Some(KnownDirective::Exec(Default::default())),
        "link" => Some(KnownDirective::Link(Default::default())),
        _ => None,
      },
    }
  }
}

impl<'a> ChosenDirective<'a> {
  pub fn data(&self) -> Option<DirectiveData> {
    if let Some(x) = &self.chosen {
      return Some(match x {
        KnownDirective::Brew(dir) => dir.directive_data().clone(),
        KnownDirective::Create(dir) => dir.directive_data().clone(),
        KnownDirective::Exec(dir) => dir.directive_data().clone(),
        KnownDirective::Link(dir) => dir.directive_data().clone(),
      });
    }
    None
  }

  pub fn parse_context_defaults(
    &self,
    defaults: &strict_yaml_rust::StrictYaml,
  ) -> Result<(String, Settings), DotfilesError> {
    if let Some(dir) = &self.chosen {
      Ok((
        self.name.clone(),
        match dir {
          KnownDirective::Brew(dir) => dir.directive_data().parse_context_defaults(defaults)?,
          KnownDirective::Create(dir) => dir.directive_data().parse_context_defaults(defaults)?,
          KnownDirective::Exec(dir) => dir.directive_data().parse_context_defaults(defaults)?,
          KnownDirective::Link(dir) => dir.directive_data().parse_context_defaults(defaults)?,
        },
      ))
    } else {
      Err(DotfilesError::from(
        format!(
          "Found defaults section for nonexistent Directive {}",
          self.name
        ),
        ErrorType::InconsistentConfigurationError,
      ))
    }
  }
}

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

//! This module contains the base trait for all [Action]s.

use yaml_rust::Yaml;

use crate::{directive::HasDirectiveData, error::DotfilesError, Settings};

/// An action to be run by a the dotfiles runtime.
pub trait Action<'a> {
  /// Executes the action.
  ///
  /// Returns an error String describing the issue, this string can be used
  /// to log or display to the user.
  fn execute(&self) -> Result<(), DotfilesError>;
}

/// Trait to parse a specific action type from Yaml.
pub trait ActionParser<'a>: HasDirectiveData<'a> {
  /// The action type this object parses
  type ActionType: Action<'a>;

  /// The name of the action this object parses
  fn name(&'a self) -> &'a str {
    self.directive_data().name()
  }

  /// Builds a single action of type [ActionParser::ActionType] from Yaml tree object
  /// that represents the action's configuration and a default settings object.
  ///
  /// Returns an Error containing a human readable string in case there
  /// was an issue building the action.
  fn parse_action(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Self::ActionType, DotfilesError>;

  /// Builds a list of actions of type [ActionParser::ActionType] from Yaml tree object
  /// that represents the actions' configurations and a default settings object.
  ///
  /// Returns an Error containing a human readable string in case there
  /// was an issue building the action.
  ///
  /// The default implementation assumes there must be Yaml array whose items each
  /// represent an individual action
  fn parse_action_list(
    &'a self,
    settings: &Settings,
    yaml: &Yaml,
  ) -> Result<Vec<Self::ActionType>, DotfilesError> {
    if let Yaml::Array(arr) = yaml {
      let list: Vec<Result<Self::ActionType, DotfilesError>> = arr
        .iter()
        .map(|yaml_item| self.parse_action(settings, yaml_item))
        .collect();

      let mut list_successes = Vec::<Self::ActionType>::new();
      for res in list.into_iter() {
        match res {
          Err(err) => return Err(err),
          Ok(act) => list_successes.push(act),
        }
      }
      Ok(list_successes)
    } else {
      Err(DotfilesError::from_wrong_yaml(
        format!(
          "An array of {} actions was expected, did not find an array.",
          self.name()
        ),
        yaml.clone(),
        Yaml::Array(vec![]),
      ))
    }
  }
}

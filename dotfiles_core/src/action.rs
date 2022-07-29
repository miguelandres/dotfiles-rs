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

use crate::error::DotfilesError;

/// An action to be run by a the dotfiles runtime.
pub trait Action<'a> {
  /// Executes the action.
  ///
  /// Returns an error String describing the issue, this string can be used
  /// to log or display to the user.
  fn execute(&self) -> Result<(), DotfilesError>;
}

/// Macro to check that all actions in a Vector of results of parsing actions from yaml
/// does not contain a single error. If it contains at least an error it returns the first
/// Error, otherwise it returns Ok with a list of all the actions.
#[macro_export]
macro_rules! check_action_list_or_err {
  ( $action_type:ty, $actions_expr:expr) => {{
    let mut actions_expr_list: Vec<Result<$action_type, dotfiles_core::error::DotfilesError>> =
      $actions_expr;
    let mut list_successes = Vec::<$action_type>::new();
    for res in actions_expr_list.into_iter() {
      match res {
        Err(err) => return Err(err),
        Ok(act) => list_successes.push(act),
      }
    }
    Ok(list_successes)
  }};
}

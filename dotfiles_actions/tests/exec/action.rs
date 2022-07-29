#![cfg(test)]

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

use dotfiles_actions::exec::action::ExecAction;
use dotfiles_core::{action::Action, error::DotfilesError};

use crate::utils::check_action_fail;

#[test]
fn failed_command_returns_error() -> Result<(), DotfilesError> {
  let action = ExecAction::new("exit 1".into(), None, false);
  check_action_fail(
    &action,
    "An error was expected when exit 1 was called but no error was returned".to_string(),
  )
}

#[test]
fn exec_succeeds() -> Result<(), DotfilesError> {
  let action = ExecAction::new("exit 0".into(), None, false);
  action.execute()
}

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

#![cfg(test)]
use crate::utils::read_test_yaml;

use dotfiles_actions::exec::action::ExecAction;
use dotfiles_actions::exec::directive::ExecDirective;
use dotfiles_core::action::ActionParser;
use dotfiles_core::directive::Settings;
use dotfiles_core::error::DotfilesError;

#[test]
fn parse_list_of_execs() -> Result<(), DotfilesError> {
  let directive = ExecDirective::new();
  let default_settings = Settings::new();
  let yaml = read_test_yaml("directive/exec/exec_list.yaml")
    .unwrap()
    .pop()
    .unwrap();

  let actions = directive.parse_action_list(&default_settings, &yaml)?;
  assert_eq!(actions.len(), 3);
  assert_eq!(
    actions[0],
    ExecAction::new(r#"echo "hello world""#.into(), None, false)
  );
  assert_eq!(
    actions[1],
    ExecAction::new(
      r#"sleep 5"#.into(),
      Some(String::from("waste some time")),
      true
    )
  );
  assert_eq!(actions[2], ExecAction::new(r#"ls"#.into(), None, false));
  Ok(())
}

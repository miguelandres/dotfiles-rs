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

//! This module contains the base trait for all [Action]s.

use crate::error::DotfilesError;

/// Skip this whole action in CI environments.
pub const SKIP_IN_CI_SETTING: &str = "skip_in_ci";
/// An action to be run by the dotfiles runtime.
pub trait Action {
  /// Executes the action.
  ///
  /// Returns an error String describing the issue, this string can be used
  /// to log or display to the user.
  fn execute(&self) -> Result<(), DotfilesError>;

  /// Whether to skip this action in Continuous Integration environments.
  ///
  /// See [is_running_in_ci()]
  fn skip_in_ci(&self) -> bool;

  /// Checks that the conditions allow for executing this action, and if so executes it according to
  /// [execute(&self)].
  ///
  /// If conditions don't pass it simply skips and returns `Ok(())`
  ///
  /// At this moment the only condition that is supported is whether the action should be skipped in
  /// CI, see [skip_in_ci(&self)].
  fn check_conditions_and_execute(&self) -> Result<(), DotfilesError> {
    if self.skip_in_ci() && is_running_in_ci() {
      Ok(())
    } else {
      self.execute()
    }
  }
}

/// Whether the execution environment is presumed to be CI
///
/// The presence of any of the following Environment Variables is assumed to
/// mean that this action is running in a CI Environment:
///
/// * `TF_BUILD`
/// * `BUILDKITE`
/// * `BUILD_ID`
/// * `CI`
/// * `CIRCLECI`
/// * `CIRRUS_CI`
/// * `CODEBUILD_BUILD_ID`
/// * `GITLAB_CI`
/// * `GITHUB_ACTIONS`
/// * `HEROKU_TEST_RUN_ID`
/// * `TEAMCITY_VERSION`
/// * `TRAVIS`
pub fn is_running_in_ci() -> bool {
  if std::env::var("DOTFILES_TESTING_ENV_VAR").is_ok() {
    return std::env::var("TESTING_ONLY_FAKE_CI").is_ok();
  }
  let env_vars = vec![
    "TF_BUILD",
    "BUILDKITE",
    "BUILD_ID",
    "CI",
    "CIRCLECI",
    "CIRRUS_CI",
    "CODEBUILD_BUILD_ID",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "HEROKU_TEST_RUN_ID",
    "TEAMCITY_VERSION",
    "TRAVIS",
  ];
  for var in env_vars.iter() {
    if std::env::var(var).is_ok() {
      return true;
    }
  }
  false
}

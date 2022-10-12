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

//! Module for the error handling classes and enums.

use std::fmt::Formatter;

use getset::Getters;
use itertools::fold;
use std::fmt::Display;
use subprocess::ExitStatus;
use subprocess::PopenError;

/// Executes the `process_function` in each of the items in the `iterable`, and then returns
/// `Ok(())`. It stops execution if any of the process functions returns an Error, and returns said
/// error.
pub fn fold_until_first_err<I, F, E>(iterable: I, mut process_function: F) -> Result<(), E>
where
  I: IntoIterator,
  F: FnMut(I::Item) -> Result<(), E>,
{
  fold(iterable, Ok(()), |prev_res, item| match prev_res {
    Ok(()) => process_function(item),
    Err(err) => Err(err),
  })
}

/// A collection of types of errors that may occur while parsing or executing actions
#[derive(Debug)]
pub enum ErrorType {
  /// An error occurred while running a command necessary for executing an action
  ExecutionError {
    /// If the command could not execute for some reason the underlying Popen Error will be saved
    /// here
    popen_error: Option<PopenError>,
    /// If the command attempted to execute but failed for some reason, the underlying ExitStatus
    /// will be saved here.
    exit_status: Option<ExitStatus>,
  },
  /// A filesystem error that was encountered while either reading configuration or
  /// executing a filesystem related action
  FileSystemError {
    /// The underlying filesystem error.
    fs_error: std::io::Error,
  },
  /// The configuration is missing a required field
  IncompleteConfigurationError {
    /// Name of the field missing in the configuration
    missing_field: String,
  },
  /// An error that occurred while parsing the Yaml file
  YamlParseError,
  /// Received an Yaml object of an unexpected type
  UnexpectedYamlTypeError,
  /// A core logic error for Dotfiles-rs
  CoreError,
  /// An error only for testing, the action that should fail actually succeeds!
  TestingErrorActionSucceedsWhenItShouldFail,
}

impl Display for ErrorType {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Creates an [ErrorType::ExecutionError]
pub fn execution_error(
  popen_error: Option<PopenError>,
  exit_status: Option<ExitStatus>,
) -> ErrorType {
  ErrorType::ExecutionError {
    popen_error,
    exit_status,
  }
}

/// Struct that represents an error that happened while parsing or executing actions.
#[derive(Debug, Getters)]
pub struct DotfilesError {
  /// Human-readable error message
  #[getset(get = "pub")]
  message: String,
  /// [Error type](ErrorType)
  #[getset(get = "pub")]
  error_type: ErrorType,
}

impl DotfilesError {
  /// Creates a new Dotfiles error with the given message and error type
  pub fn from(message: String, error_type: ErrorType) -> Self {
    DotfilesError {
      message,
      error_type,
    }
  }
  /// Creates a new Dotfiles error with the given message and error type
  pub fn from_io_error(io_error: std::io::Error) -> Self {
    DotfilesError {
      message: io_error.to_string(),
      error_type: ErrorType::FileSystemError { fs_error: io_error },
    }
  }
}

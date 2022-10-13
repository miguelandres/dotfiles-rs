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

//! Wraps some logic to run external commands and handle errors

use subprocess::{Exec, ExitStatus, PopenError};

use crate::error::{execution_error, process_until_first_err, DotfilesError};

/// Executes the `cmd` and waits for it to finish.
///
/// If the execution returns a [subprocess::PopenError] then it uses the `popen_error_message`
/// for the message in a DotfilesError.
///
/// If the execution finishes but in an error state, then it uses the
/// `error_while_running_message` instead.
pub fn execute_commands(
  cmds: Vec<Exec>,
  popen_error_message: &str,
  error_while_running_message: &str,
) -> Result<(), DotfilesError> {
  process_until_first_err(cmds.into_iter(), |cmd| {
    handle_exec_error(cmd.join(), popen_error_message, error_while_running_message)
  })
}

fn handle_exec_error(
  popen: Result<ExitStatus, PopenError>,
  popen_error_message: &str,
  error_while_running_message: &str,
) -> Result<(), DotfilesError> {
  popen.map_or_else(
    |err| {
      Err(DotfilesError::from(
        popen_error_message.into(),
        execution_error(Some(err), None),
      ))
    },
    |status| match status {
      ExitStatus::Exited(0) => Ok(()),
      _ => Err(DotfilesError::from(
        format!("{}, {:?}", error_while_running_message, status),
        execution_error(None, Some(status)),
      )),
    },
  )
}

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

use std::path::PathBuf;

use dotfiles_core::path::process_home_dir_in_path;

#[test]
fn test_converts_tilde_to_absolute() {
  let path = PathBuf::from("~");
  assert!(path.is_relative());
  assert!(!path.is_absolute());
  let new_path = process_home_dir_in_path(&path);
  assert!(new_path.is_absolute());
  assert!(!new_path.is_relative());
  assert_ne!(&path, &new_path);

  let path = PathBuf::from("~/my_file");
  assert!(path.is_relative());
  assert!(!path.is_absolute());
  let path = process_home_dir_in_path(&path);
  assert!(new_path.is_absolute());
  assert!(!new_path.is_relative());
  assert_ne!(&path, &new_path);
}

#[test]
fn test_does_not_convert_component_that_starts_with_tilde() {
  let path = PathBuf::from("~arg/some");
  assert!(path.is_relative());
  assert!(!path.is_absolute());
  let new_path = process_home_dir_in_path(&path);
  // no change should have happened
  assert!(new_path.is_relative());
  assert!(!new_path.is_absolute());
  assert_eq!(&path, &new_path);
}

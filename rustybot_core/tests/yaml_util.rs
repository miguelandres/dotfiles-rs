#![cfg(test)]

// Copyright (c) 2021-2021 Miguel Barreto and others
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

extern crate yaml_rust;

use rustybot_core::yaml_util::*;
use yaml_rust::YamlLoader;

const YAML_STRING: &str = "
- create:
    - /home/user/somedir";

const YAML_HASH: &str = "
- create:
    - dir: /home/user/somedir";
const YAML_HASH_WITHOUT_EXPECTED_KEY: &str = "
- create:
    - what: /home/user/somedir";

#[test]
fn gets_bare_string_successfully() -> Result<(), String> {
  let docs = YamlLoader::load_from_str(YAML_STRING).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, None)?
  );
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, Some("dir"))?
  );
  Ok(())
}
#[test]
fn gets_string_in_hash_successfully() -> Result<(), String> {
  let docs = YamlLoader::load_from_str(YAML_HASH).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  assert_eq!(
    "/home/user/somedir",
    get_string_content_or_keyed_value(list, Some("dir"))?
  );
  Ok(())
}
#[test]
#[should_panic]
fn fails_to_get_string_in_hash_when_no_key_was_provided() {
  let docs = YamlLoader::load_from_str(YAML_HASH).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  get_string_content_or_keyed_value(list, None).unwrap();
}

#[test]
#[should_panic]
fn fails_to_get_string_in_hash_when_without_correct_key() {
  let docs = YamlLoader::load_from_str(YAML_HASH_WITHOUT_EXPECTED_KEY).unwrap();
  let doc = &docs[0];
  let list = &doc[0]["create"][0];
  get_string_content_or_keyed_value(list, None).unwrap();
}

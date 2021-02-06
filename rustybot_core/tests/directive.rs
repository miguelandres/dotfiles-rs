#![cfg(test)]

extern crate yaml_rust;

use rustybot_core::directive::*;
use yaml_rust::YamlLoader;

mod test_get_string_content_or_keyed_value {
  use super::*;
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
}

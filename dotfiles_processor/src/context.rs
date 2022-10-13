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

use std::collections::HashMap;

use dotfiles_core::{
  directive::DirectiveSet,
  error::{process_until_first_err, DotfilesError, ErrorType},
  Settings,
};
use yaml_rust::Yaml;
/// A context represents an environment in which defaults can be overriden, it can be thought of as
/// the context of an individual configuration file to apply.
///
/// Notice that contexts *can* be built on top of one another, so that defaults can be overriden
/// multiple times, and thus have some sort of configuration inheritance.
pub struct Context<'a> {
  /// A reference to the directive set that is being used, this will be used to get the list of
  /// existing directives and to find out how to parse configuration values.
  directive_set: &'a DirectiveSet<'a>,
  /// The default overrides for the current context.
  defaults: HashMap<String, Settings>,
  /// The file name to which this context corresponds.
  file_name: String,
}

impl<'a> Context<'a> {
  /// Creates a new [`Context`].
  pub fn new(
    directive_set: &'a DirectiveSet<'a>,
    defaults: HashMap<String, Settings>,
    file_name: String,
  ) -> Self {
    Self {
      directive_set,
      defaults,
      file_name,
    }
  }

  /// Adds defaults to the context configuration from the passed Yaml configuration.
  ///
  /// It may fail for several reasons:
  ///
  /// * [ErrorType::InconsistentConfigurationError]
  ///   - In case the configuration mentions a directive that doesn't exist
  ///   - A directiveis mentioned more than once.
  /// * [ErrorType::UnexpectedYamlTypeError]:
  ///   - The Yaml passed to this function is not a Hash.
  ///   - The hash contains keys that are not Strings.
  ///   - The Yaml contains values that are not Hashes of Strings to settings
  ///   - The Yaml type for a particular setting does not match its expected data type.
  fn add_defaults(&mut self, yaml: &Yaml) -> Result<(), DotfilesError> {
    match yaml {
      Yaml::Hash(hash) => process_until_first_err(hash.into_iter(), |(k, v)| {
        let (directive_name, defaults) = self.parse_directive_defaults_for_yaml(k, v)?;
        self
          .defaults
          .try_insert(directive_name.clone(), defaults)
          .map_or(
            Err(DotfilesError::from(
              format!(
                "Defaults contain multiple configurations for the same directive {}",
                &directive_name
              ),
              ErrorType::InconsistentConfigurationError,
            )),
            |_| Ok(()),
          )
      }),
      something_else => Err(DotfilesError::from(
        format!(
          "Defaults should contain a Hash, it instead contains {:?}",
          something_else
        ),
        ErrorType::UnexpectedYamlTypeError,
      )),
    }
  }

  fn parse_directive_defaults_for_yaml(
    &mut self,
    k: &Yaml,
    v: &Yaml,
  ) -> Result<(String, Settings), DotfilesError> {
    if let (Yaml::String(directive_name), defaults) = (k, v) {
      if let Some(directive) = self.directive_set.get(&directive_name) {
        Ok((
          directive_name.to_owned(),
          directive.parse_settings(&defaults)?,
        ))
      } else {
        Err(DotfilesError::from(
          format!(
            "Found defaults section for nonexistent Directive {}",
            &directive_name
          ),
          ErrorType::InconsistentConfigurationError,
        ))
      }
    } else {
      Err(DotfilesError::from(
        format!(
          "Unable to parse defaults section, was expecting String and Hash but got {:?} -> {:?}",
          k, v
        ),
        ErrorType::UnexpectedYamlTypeError,
      ))
    }
  }
}

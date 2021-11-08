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

//! This module defines [HomebrewInstallDirective].
#![cfg(unix)]
extern crate yaml_rust;

use crate::directive::Directive;
use crate::directive::DirectiveData;
use crate::directive::Settings;
use crate::homebrew_install::action::HomebrewInstallAction;
use yaml_rust::Yaml;

/// Name of the link directive
pub const DIRECTIVE_NAME: &str = "homebrew_install";

/// Initialize the defaults for the HomebrewInstallDirective.
pub fn init_directive_data() -> DirectiveData {
    DirectiveData::new(DIRECTIVE_NAME, Settings::new())
}

/// A directive that can build [HomebrewInstallAction] to set up homebrew
pub struct HomebrewInstallDirective {
    data: DirectiveData,
}

impl Default for HomebrewInstallDirective {
    fn default() -> Self {
        Self::new()
    }
}

impl HomebrewInstallDirective {
    /// Create a new [HomebrewInstallDirective]
    pub fn new() -> HomebrewInstallDirective {
        HomebrewInstallDirective {
            data: init_directive_data(),
        }
    }
}

impl Directive<'_, HomebrewInstallAction> for HomebrewInstallDirective {
    fn name(&self) -> &str {
        self.data.name()
    }

    fn defaults(&self) -> &Settings {
        self.data.defaults()
    }

    fn build_action(
        &self,
        _settings: &Settings,
        _yaml: &Yaml,
    ) -> Result<HomebrewInstallAction, String> {
        Ok(HomebrewInstallAction {})
    }
}

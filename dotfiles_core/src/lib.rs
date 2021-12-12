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

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(io_error_more)]
#![feature(map_try_insert)]

//! The core of Dotfiles-rs is basically a set of [directives](
//! directive::Directive) that can build executable [actions](action::Action)
//! from configuration sources (Yaml is the only supported configuration source
//! so far).
//!
//! Directives are responsible for parsing configuration from the configuration
//! source, applying default [settings](directive::Settings) if necessary, and
//! using these to build actions that can be executed.
//!
//! The base traits and classes for these building blocks can be found in the
//! [action] and [directive] modules. The [yaml_util] module is a set of helper
//! functions for Directives to parse configuration.
//!
//! Finally all concrete actions and directives can be found in the following
//! modules:
//!
//! - [create]: Create a new directory
pub mod action;
#[cfg(unix)]
pub mod brew;
pub mod create;
pub mod directive;
#[cfg(unix)]
pub mod homebrew_install;
pub mod link;
#[cfg(unix)]
pub mod ohmyzsh_install;
pub mod yaml_util;

pub use action::Action;
pub use directive::Directive;
pub use directive::Setting;
pub use directive::Settings;

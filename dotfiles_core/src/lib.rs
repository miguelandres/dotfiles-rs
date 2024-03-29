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

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(map_try_insert)]

//! The core of Dotfiles-rs is basically a set of [directives](
//! directive::Directive) that can build executable [actions](action::Action)
//! from configuration sources (StrictYaml is the only supported configuration source
//! so far).
//!
//! Directives are responsible for parsing configuration from the configuration
//! source, applying default [settings] if necessary, and
//! using these to build actions that can be executed.
//!
//! The base traits and classes for these building blocks can be found in the
//! [action] and [directive] modules. The [yaml_util] module is a set of helper
//! functions for Directives to parse configuration.
pub mod action;
pub mod directive;
pub mod error;
pub mod exec_wrapper;
pub mod path;
pub mod settings;
pub mod yaml_util;

pub use action::Action;
pub use directive::Directive;
pub use settings::Setting;
pub use settings::Settings;

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
#![feature(io_error_more)]
#![feature(drain_filter)]

//! This crate contains all concrete [actions](dotfiles_core::action::Action)
//! and [directives](dotfiles_core::directive::Directive). They are contained in
//! each of the modules in this crate:
//!
//! - [brew]: A set of actions and directive that act as wrappers around
//!   [homebrew](http://brew.sh).
//! - [create]: Creates a new directory
//! - [exec]: runs a command in the shell
//! - [homebrew_install]: installs [homebrew](http://brew.sh).
//! - [link]: Creates a symlink to a file or directory
//! - [ohmyzsh_install]: installs ohmyzsh
#[cfg(unix)]
pub mod brew;
pub mod create;
pub mod exec;
#[cfg(unix)]
pub mod homebrew_install;
#[cfg(unix)]
pub mod link;
#[cfg(unix)]
pub mod ohmyzsh_install;

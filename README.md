# dotfiles-rs

[![Build Status](https://github.com/miguelandres/dotfiles-rs/actions/workflows/rust_build_and_test.yml/badge.svg?branch=main)](https://github.com/miguelandres/dotfiles-rs/actions/workflows/rust_build_and_test.yml)
[![Rust Documentation](https://github.com/miguelandres/dotfiles-rs/actions/workflows/rust_doc_generator.yml/badge.svg?branch=main)](https://miguelandres.github.io/dotfiles-rs/)

[Rust Documentation](https://miguelandres.github.io/dotfiles-rs/)

Dotfiles-rs is an automation framework that can be used for dotfiles
configuration. It was inspired by
[`anishathalye/dotbot`](https://github.com/anishathalye/dotbot) and rewritten in
rust in part to improve maintainability and in part for curiosity about the Rust
language.

## Core Concepts

A `dotfiles-rs` configuration is, in practice, a list of steps (actions) that
execute linearly. This configuration is expressed across one or multiple files,
to allow for modularization and reuse of these configurations.

### Directives & Actions

A directive is a type of action that `dotfiles-rs` can execute, currently the
following directives are supported:

* `brew`: uses homebrew to install formulae, casks or Mac App Store apps via
  `mas`
* `create`: creates a new directory
* `exec`: runs a command in zsh
* `link`: creates a symlink to a file or directory
* `subconfig`: calls into another configuration file, this allows modularization
  of configurations

Directives are used to instantiate actions, which make up the steps that
`dotfiles-rs` executes. In order to instantiate you need to specify them in the
`steps` object in a YAML configuration file.

```yaml
steps:
 - create:
  - ~/src
 - exec:
   - 'cd ~/src && git clone git://github.com/miguelandres/dotfiles_v2'
 - create:
  - path: ~/.oh-my-zsh/custom
    create_parent_dirs: true
 - link:
  - ~/.oh-my-zsh/custom/something: ~/src/dotfiles_v2/something
  - ~/.oh-my-zsh/custom/something_else: something_else_in_this_dir
```

Notice that in the example above:

* Directives can be invoked more than once to generate the actions in the right
  order
* Some directives (create, link, exec) have short-hand versions and versions
  where more settings can be specified
* Directives support both absolute paths (or relative to the home dir, `~`) and
  paths relative to the configuration file.

### Defaults

Defaults are settings for each directive that can be shared across multiple
actions of the same directive in the same file or context.

For example, if you always (or almost always) use the same setting for a specific
directive, you can set it up in a `defaults` section and then only specify it in
cases where you need to override that default.

That means that the following file ...

```yaml
steps:
 - create:
  - ~/src
  - path: ~/.vim/plugins
    create_parent_dirs: true
  - path: ~/.oh-my-zsh/custom
    create_parent_dirs: true
```

... would be equivalent to ...

```yaml
defaults:
  create:
   create_parent_dirs: true

steps:
 - create:
  - path: ~/src
    create_parent_dirs: true
  - ~/.vim/plugins
  - ~/.oh-my-zsh/custom
```

### Configurations and contexts

* Each file is a
  [`Context`](https://miguelandres.github.io/dotfiles-rs/dotfiles_processor/context/struct.Context.html).
  A context contains defaults for the entire context, and subcontexts.
* A subcontext is created by invoking a file from another using the `subconfig`
  directive. Defaults are inherited from the parent context (file) and can be
  overriden at any level if necessary.

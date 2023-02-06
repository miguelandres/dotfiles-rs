# `exec` directive

The `exec` executes a command in the `zsh` shell. Each command will run in its
own isolated nested zsh process from the directory where the configuration file
is.

## Shortened version

You can just list commands to run

```yaml
  - exec:
    - git clone git://github.com/miguelandres/dotfiles-rs
    - ls
```

## Options

* `cmd` (**required**): the command to run
* `description` (optional): human-readable description.
* `echo` (default: false): Output the command to run to the console

## Example

```yaml
  - exec:
    - cmd: git clone git://github.com/miguelandres/dotfiles-rs
      description: Cloning repo
      echo: true
    - ls dotfiles-rs
```

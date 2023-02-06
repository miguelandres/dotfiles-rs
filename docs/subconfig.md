# `subconfig` directive

The `subconfig` directive instantiates the listed configuration files in new
contexts for each one.

The new contexts inherit all of the defaults for the current configuration and
any defaults found in the invoked file will override existing ones in the
created subcontext.

## Example

```yaml
# All of these defaults will be inherited in each of the files below, when
# inboked from this file.
defaults:
  create:
    force: true
  exec:
    echo: true
  brew:
    force_casks: true
  link:
    force: true
    relink: true

steps:
  - subconfig:
      # Each of these files can override any of the defaults above if necessary.
      - base.yaml
      - linux/base.yaml
      - zsh/base.yaml
      - zsh/linux.yaml
      - git/personal.yaml
      - tmux/tmux.yaml
      - vim/vim.yaml

```

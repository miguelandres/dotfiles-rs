# `create` directive

The `create` directive creates a new directory specified by the `dir` setting.

## Shortened version

You can just list directories to create and the directive will use the defaults
for the current context.

```yaml
  - create:
    - ~/.zsh
    - ~/.ssh
    - ~/src
```

## Options

* `dir` (**required**): the path (absolute or relative) of the directory to
  create
* `create_parent_dirs` (default: false): creates all the parent dirs that
  contain the directory to create.

## Example

```yaml
  - create:
    - ~/.zsh
    - dir: ~/.ssh/keys
      create_parent_dirs: true
```

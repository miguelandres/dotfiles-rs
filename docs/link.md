# `link` directive

The `link` creates a symlink at the `path` pointing to the `target` file or dir.

Notice that even though the `target` file can be relative to the current
configuration file, the symlink will be created pointing to the absolute path to
the file.

## Shortened version

You can just list directories mappings of symlink paths to destinations and the
directive  will use the defaults for the current context.

```yaml
  - link:
    - ~/.zsh/custom: zsh/custom
    - ~/.ssh: ssh
```

## Options

* `path` (**required**): path of the new symlink. Can be absolute or relative.
* `target` (**required**): path to the target file or directory for the symlink.
  Can be absolute or relative, but the symlink will be created to the absolute
  location of this file/dir.
* `create_parent_dirs` (default: false): create parent dirs of the `path` if
  they don’t exist
* `ignore_missing_target` (default: false): if set to true the action will not
  fail if the target file does not exist
* `force` (default: false): delete any existing file or directory at `path` and
  replace it with the new symlink
* `relink` (default: false): if a symlink already exists at `path`, recreate it
  pointing to the new `target`.
* `resolve_simlink_target` (default: false): if the `target` is also a simlink,
  recursively find the concrete target it points to and use that instead.

## Example

```yaml
  - link:
      - scratch_dir/test_dir/home: ~
      - path: ~/.zsh/custom
        target: zsh/custom
        force: true
```

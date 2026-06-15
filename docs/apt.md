# `apt` action

The `apt` action allows you to install packages using `apt`.

## Options

* `package` (optional): a list of packages to install

## Example

```yaml
  - apt:
      package:
        - fzf
  - apt:
      # This is a separate apt action so it can be skipped in CI.
      package:
        - python3
```

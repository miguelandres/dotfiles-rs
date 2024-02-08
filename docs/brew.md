# `brew` directive

The `brew` directive allows you to install packages using homebrew and `mas`.

**Note:** in order to use `mas` you need to install it as a formula using `brew`.
You can use this brew directive itself to install `mas`

## Options

* `force_casks` (default: false): if true it passes `--force` to all invocations
  `brew install --cask ...`. This is useful when you may have a previous version
  of the same application already installed, as it overwrites whatever binaries
  already exist in your system. This is not the default behavior for `brew` so
  that is why it's not enabled by default in `dotfiles-rs`
* `adopt_casks` (default: false): if true it passes `--adopt` to all invocations
  `brew install --cask ...`. This is useful when you may have a previous version
  of the same application already installed, as it allows homebrew to adopt
  already existing binaries so long as they match the one that was going to be
  installed. This is not the default behavior for `brew` so that is why it's not
  enabled by default in `dotfiles-rs`

> [!CAUTION]
> `force_casks` and `adopt_casks` are mutually exclusive and brew will fail to
> run if you enable both. See the [homebrew documentation](https://docs.brew.sh/Manpage#install-options-formulacask-)

* `tap` (optional): a list of homebrew taps to use
* `formula` (optional): a list of formulae to install
* `cask` (optional): a list of casks to install
* `mas` (optional): a dictionary of Mac App Store apps to install. The keys of
  the dictionary are human-readable names for the apps, the values are app store
  IDs. See the [mas documentation](https://github.com/mas-cli/mas) for more info

## Example

```yaml
  - brew:
      tap:
        # for different versions of temurin
        - homebrew/cask-versions
      formula:
        - vim
      cask:
        - visual-studio-code
  - brew:
      # This is a separate brew directive so it can be skipped in CI.
      # Some of these formulae and casks either take too long to install or just
      # don't install in CI.
      # `mas` actions should never be run in CI since they depend on a
      # Mac App Store login.
      skip_in_ci: true
      cask:
        - basictex
      mas:
        WhatsApp: 1147396723
```

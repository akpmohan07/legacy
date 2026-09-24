---
source: homebrew-cellar
status: built
platform: macOS
scanner: detector/src/scanner/macos/homebrew_cellar.rs
issues: [16]
---

# Source: homebrew-cellar

## Scope
- **In:** formulae in the Cellar that were installed **on request** (the user ran `brew install <formula>`).
- **Out:** formulae pulled in only as dependencies, casks (apps), Homebrew taps that were added but
  not installed from, and any version directory without a readable `INSTALL_RECEIPT.json`.

## Identifier
The formula name (`btop`). For a formula from a tap other than `homebrew/core`, the tap is prefixed
(`matheusml/zsh-ai/zsh-ai`). `name` is always the bare formula name.

## Discovery
Look for `Cellar` under `/opt/homebrew` (Apple Silicon), then `/usr/local` (Intel), and use the first
that exists. Under it, each `<formula>/<version>/` directory with an `INSTALL_RECEIPT.json` is read
for `installed_on_request`, `time`, and `source.tap`. The `brew` command is never run.

## Version and install date
- `version`: the name of the version directory (`1.4.6`), which includes Homebrew's revision suffix
  when there is one (for example `1.4.6_1`).
- `installed_at`: the receipt's `time` in whole seconds. Some receipts have none, so it can be absent.

## Absence check
The default path check, on the recorded version directory. After an upgrade the formula is found again
under the same identifier with a new version, so it is an update, not an uninstall. (Whether the old
version directory lingers until `brew cleanup` was not checked; if it does, two versions are on disk
and the newest wins, see Known gaps.)

## Platform notes
macOS only today. Homebrew on Linux uses a different prefix (`/home/linuxbrew/.linuxbrew`), which is not
handled.

## Known gaps
- The source's own `version` (the Homebrew version) is not read: the probe reports none, so a source
  never gets an `updated` event. Reading it means running `brew --version`, which needs a timeout
  first.
- A formula with two version directories on disk is reported once, the one with the newest
  `installed_at`, then the greater path.

## Guarded by
- test: scanner::macos::homebrew_cellar::tests::keeps_only_requested_formulae_and_prefixes_non_core_taps
- test: scanner::macos::homebrew_cellar::tests::an_unreadable_cellar_is_an_error_not_an_empty_success
- test: domain::plan::tests::duplicate_identifiers_keep_the_newest_install_regardless_of_order

## Verified live
19 formulae found as the baseline on the owner's Mac. `brew install tree` produced one `installed`
event and `brew uninstall tree` one `uninstalled` event, and a repeated scan afterward recorded nothing.

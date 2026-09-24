---
source: application
status: built
platform: macOS
scanner: detector/src/scanner/macos/applications.rs
issues: [16]
---

# Source: application

## Scope
- **In:** every `*.app` directly inside `/Applications` that has a readable `Contents/Info.plist`.
- **Out:** apps in subfolders (`/Applications/Utilities`, vendor folders), `~/Applications`, `/System/Applications`,
  and anything that is not an `.app` bundle. A bundle whose `Info.plist` is missing or unreadable is
  skipped silently.

## Identifier
`CFBundleIdentifier` (for example `com.apple.Safari`). If the plist has none, the identifier is the
literal `unknown` (see Known gaps). `name` is the folder name without `.app`.

## Discovery
List `/Applications`, keep entries ending in `.app`, parse `Contents/Info.plist` with the `plist` crate.
No processes are spawned.

## Version and install date
- `version`: `CFBundleShortVersionString` (the marketing version), or none if absent.
- `installed_at`: **not available** (always none). Bundles carry no reliable install date.

## Absence check
The default: the recorded path is missing ⇒ gone; present ⇒ still there; unreadable ⇒ unknown.
The probe is whether `/Applications` exists.

## Platform notes
macOS only. Compiled only on macOS through `#[cfg(target_os = "macos")]`.

## Known gaps
- Two apps with no bundle identifier both become `unknown`; the planner keeps only one of them.
- Apps installed through Homebrew Cask or the Mac App Store are reported here as ordinary apps, with
  no hint of how they were installed. Which source should own them is undecided.
- The marketing version does not change on build-only updates, so those updates are not seen.
- An app moved out of `/Applications` (or into a subfolder) is no longer found, so it is recorded as
  uninstalled.
- Not investigated: the noise from watching `/Applications` (needed by `automatic-monitoring`).

## Guarded by
- test: scanner::macos::applications::tests::reads_id_and_version_and_skips_broken_bundles_and_non_apps
- test: scanner::macos::applications::tests::an_unreadable_apps_folder_is_an_error_not_an_empty_success
- test: scanner::tests::probe_path_distinguishes_present_from_missing
- test: scanner::tests::absence_is_gone_only_when_the_path_is_confirmed_missing

## Verified live
39 apps found and recorded as the baseline on the owner's Mac.

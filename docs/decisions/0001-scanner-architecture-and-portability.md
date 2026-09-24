---
id: 0001
title: Scanner architecture and portability
status: accepted
date: 2026-09-22
issues: [15]
supersedes: []
related: [0008, 0009]
---

## Context
Legacy must discover tools from several unlike sources (`/Applications` bundles, Homebrew receipts,
later cargo, npm, and others), on macOS now and possibly Windows and Linux later. Every source has
its own raw format, but everything downstream needs one shape.

## Decision
- **Strategy plus Adapter.** A `Scanner` trait is the contract (`source_name`, `probe`, `scan`, and a
  default `confirm_absent`). Each implementation adapts one source's raw data into the single
  `DiscoveredTool` shape. A `ScannerRegistry` holds them as `Box<dyn Scanner>`, and the rest of the
  system talks only to the trait.
- **Adding a source is one new file plus one line in `ScannerRegistry::build()`.** Nothing downstream
  changes.
- **Platform differences are compile-time, not runtime.** OS-specific scanners live under
  `scanner/macos/` behind `#[cfg(target_os = "macos")]`, and are registered inside a `cfg` block.
  A scanner whose source behaves the same on every OS (cargo and npm are the expected cases) would be a
  plain module with no `cfg`. Do not write `if os == "macos"` checks: they ship dead code and can
  run the wrong branch.
- Sources fall into three groups for portability: those with no equivalent elsewhere (Homebrew, the
  App Store), those that need a different implementation per OS (native apps), and those that are
  already cross-platform (cargo, npm).
- Scanners never touch the database (see 0009).

## Consequences
- **Known gap:** `identity.rs` is not gated. It runs `system_profiler` unconditionally, so on another
  OS it would compile and then fail at run time. The detector is macOS-only for v1, so this is deferred.
- Precedent: the Rust standard library's `sys` modules and the `notify` crate (one interface, a backend
  per OS), and osquery's per-source table plugins.

## Alternatives rejected
- **One large `scan()` function that handles every source** (the way `cli-tools` does it): adding a
  source means editing shared code, and there is no per-source probe or absence check.
- **Runtime OS checks:** see above.

## Evidence
- Code: `detector/src/scanner/mod.rs` (`Scanner`, `ScannerRegistry`), `detector/src/scanner/macos/` (`applications.rs`, `homebrew_cellar.rs`)
- test: scanner::macos::applications::tests::reads_id_and_version_and_skips_broken_bundles_and_non_apps
- test: scanner::macos::homebrew_cellar::tests::keeps_only_requested_formulae_and_prefixes_non_core_taps
- test: run::tests::a_failing_source_does_not_stop_the_others_and_records_nothing_for_itself

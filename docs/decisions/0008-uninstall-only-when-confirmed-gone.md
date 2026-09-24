---
id: 0008
title: Uninstall only when confirmed gone
status: accepted
date: 2026-09-23
issues: [16]
supersedes: []
related: [0005, 0007]
---

## Context
A scan that misses a tool looks the same as a tool that was removed. An unreadable folder, a
half-installed app bundle, or a failing scanner would otherwise record a wave of false uninstalls,
and the next good scan would record them all as installs again.

## Decision
A tool that is on record but not found becomes `uninstalled` **only when its scanner reports
`Absence::Gone`**: its stored path was checked and does not exist. `StillThere` and `Unknown` record
nothing; the tool is left as it was and the planner reports it for logging.

- A source probe has three outcomes: available, not present, or failed. A failed probe records nothing.
- Existence checks use `try_exists`, so a permission error becomes "failed" or "unknown", never
  "not there". Plain `exists()` returns false on errors and must not be used for this.
- Scanners return an **error** when they cannot read their source, never an empty success.
- A tool that was already `uninstalled` and is still missing records nothing, so an uninstall is
  never repeated.
- `confirm_absent()` has a default (is the stored path gone?) that a scanner can override.

## Consequences
- A real uninstall can be recorded one scan late if the check is inconclusive.
- Every new scanner must implement `probe()` honestly.
- Item-level skips, such as an app whose `Info.plist` cannot be read, are protected because the path
  still exists (`StillThere`).

## Alternatives rejected
- **Treat "not in this scan" as uninstalled:** mass false events on any partial failure.
- **Require two consecutive misses:** adds pending state and delays every real uninstall.
- **A circuit breaker on large uninstall waves:** deferred; the checks above cover the known cases.

## Evidence
- Code: `detector/src/scanner/mod.rs` (`Probe`, `probe_path`, `absence_by_path`, `confirm_absent`), `detector/src/domain/plan.rs` (`plan_tools`)
- test: scanner::tests::probe_path_distinguishes_present_from_missing
- test: scanner::tests::absence_is_gone_only_when_the_path_is_confirmed_missing
- test: scanner::macos::homebrew_cellar::tests::an_unreadable_cellar_is_an_error_not_an_empty_success
- test: scanner::macos::applications::tests::an_unreadable_apps_folder_is_an_error_not_an_empty_success
- test: domain::plan::tests::a_missing_tool_confirmed_gone_is_uninstalled_with_its_last_version
- test: domain::plan::tests::a_missing_tool_that_is_still_there_or_unknown_records_nothing
- test: domain::plan::tests::an_already_uninstalled_tool_that_stays_missing_is_not_reported_again
- test: store::apply::tests::an_uninstall_is_recorded_once_and_then_stays_quiet
- test: run::tests::a_tool_that_is_still_there_is_left_alone
- test: run::tests::a_broken_probe_is_a_failure_with_nothing_recorded

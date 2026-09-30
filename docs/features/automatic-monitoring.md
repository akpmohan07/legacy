---
feature: automatic-monitoring
status: in progress      # v1 (watch command) written 2026-09-30, awaiting QA
issues: [17]
decisions: [0012, 0010, 0007]
---

# Automatic monitoring

## Purpose
Start a scan by itself when something changes on the machine, whatever caused it (a person, a script,
or an AI agent), so the history stays current without anyone running a scan.

## What is decided
See `../decisions/0012-automatic-trigger-mechanism.md` (status `proposed`): a small resident process
using `notify` with debouncing, each source scanned on its own, a catch-up scan at start, a scheduled
backstop, and one `launchd` plist to start it at login. Scan results and events follow the existing
rules (`change-events`, `scan-run`); only the trigger is new.

## v1: `legacy-detector watch`
1. Scans every source (`triggered_by = startup`).
2. Watches each source's folders; a source with none is scanned only at startup.
3. Loops: wait for a file event, drain the queue, keep the events a source calls relevant, and scan
   each affected source once (`triggered_by = watcher`). Changes during a scan cause one follow-up.

Relevant changes: `application`: a top-level `X.app` or `X.app/Contents/Info.plist`.
`homebrew-cellar`: a formula folder, a version folder, or `INSTALL_RECEIPT.json`.

Not in v1: the scheduled backstop and noticing a source that appears while running (#23), starting at
login (#25). A quiet window was measured as unnecessary (#24).

## Not yet known
How it is started at login and kept alive (#25), and what to do about a source that keeps failing.
Noise and idle cost were measured 2026-09-30 (decision 0012).

## Guarded by
- test: watch::tests::a_burst_under_one_folder_marks_only_that_source_once
- test: watch::tests::a_batch_of_many_events_causes_one_scan_of_the_affected_source
- test: watch::tests::a_rescan_notice_marks_every_watched_source
- test: run::tests::run_only_scans_the_named_sources_and_records_the_trigger
- test: scanner::macos::applications::tests::only_a_bundle_itself_or_its_info_plist_is_a_relevant_change
- test: scanner::macos::homebrew_cellar::tests::only_formula_and_version_folders_and_receipts_are_relevant_changes

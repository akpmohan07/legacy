---
feature: scan-run
status: built
issues: [16]
decisions: [0001, 0007, 0009, 0010, 0011]
---

# Scan run

## Purpose
One invocation of the detector: find which sources exist, scan each one, record what changed, and
report. Today it is started by hand (`cargo run`); other triggers are planned (`automatic-monitoring`).

## Rules
1. Work out once, at the start, whether this is the first run, and make one random `run_id`.
2. Probe every source. For each one, **independently** (one failure never stops the others):
   - Open a `scans` row **before** scanning, so a failed attempt stays visible.
   - **Available:** scan, then record (see `change-events`). The scanner's own check decides whether a
     missing tool is really gone.
   - **Not present:** record that (a source that just left gets its own `uninstalled` event).
   - **Probe or scan failed:** log a warning, record nothing, leave the scan's `finished_at` empty.
3. After all sources, update the installation's heartbeat (`local_identity.last_seen_at`).
4. Print a summary and exit.

**Exit codes:** `0` every source recorded, `1` at least one source failed, `2` a database error stopped
the run.

**Output** (stdout; logs go to stderr, warnings by default, `LEGACY_LOG=debug` for more):
```
application      39 tools   baseline recorded (no events)
homebrew-cellar  19 tools   baseline recorded (no events)
2 sources checked, 0 failed, 0 events
```
Later runs say `no changes`, or counts such as `1 installed, 1 updated`, or `FAILED: …`.

## Invariants
1. A failing source never prevents the other sources from being recorded.
2. A scan's `finished_at` is set only when its source was successfully recorded.
3. `run.rs` contains no SQL and no filesystem access of its own.
4. A database failure during a run stops it (exit 2) rather than continuing.

## Code map
`src/main.rs` (logging, exit code), `src/run.rs` (`run`, `RunReport`), `src/store/mod.rs` (`Store`:
scans, run ids, heartbeat), `src/store/apply.rs` (`record_scan`), `src/scanner/mod.rs` (`discover`).

## Guarded by
- test: run::tests::the_first_run_baselines_every_source_without_events
- test: run::tests::a_later_run_records_installs_updates_and_uninstalls
- test: run::tests::a_failing_source_does_not_stop_the_others_and_records_nothing_for_itself
- test: run::tests::a_broken_probe_is_a_failure_with_nothing_recorded
- test: run::tests::a_source_that_disappears_is_recorded_without_touching_its_tools
- test: run::tests::a_tool_that_is_still_there_is_left_alone
- test: run::tests::the_run_updates_the_installations_heartbeat
- test: run::tests::the_summary_reads_cleanly_for_each_outcome

## Verify by hand
`cargo run`, then `sqlite3 ~/Library/Application\ Support/Legacy/legacy.db "SELECT * FROM scans"`:
one row per source per run, all with a `finished_at` on a healthy run.

## Known limits
Only the `manual` trigger exists. Failures while opening the store panic (exit 101). The exit-2 path
has no test. Two runs at once are safe (nothing is duplicated) but not serialized.

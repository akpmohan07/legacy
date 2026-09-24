---
id: 0011
title: Error policy, exit codes, and logging
status: accepted
date: 2026-09-24
issues: [16]
supersedes: []
related: [0007, 0008]
---

## Context
A run touches several independent sources. One unreadable folder must not hide every other source's
changes, but a failure of the database itself could put the history at risk.

## Decision
- **A source fails** (its probe failed or its scan returned an error): log a warning, record nothing
  for it, leave its scan's `finished_at` empty, and carry on with the other sources.
- **The database fails** (reading or writing during a run): stop the run. `run()` returns `RunError`.
- **Exit codes:** `0` every source was recorded, `1` at least one source failed, `2` a database error
  stopped the run.
- **Logging** uses `tracing`, written to stderr. Warnings and errors show by default; set
  `LEGACY_LOG=debug` for details. Each source's scan runs inside a `source` span, so its log lines
  carry the source name. The human summary goes to stdout.
- Scan errors are not stored in a table; they go to the log and the exit code.

## Consequences
- A scheduler or script can tell a partly failed run from a clean one by the exit code.
- **Known gap:** failures while *opening* the store (data folder, migrations, first identity) still
  panic, which exits with code 101, not 2. Only errors during a run return 2.
- The exit-code-2 path has no test yet.
- The monitor will need a policy for a source that keeps failing. Nothing is designed for that.

## Alternatives rejected
- **Abort on the first failed source:** one broken source would hide every other source's changes.
- **Turn errors into an empty success:** false uninstalls (0008).
- **A table of scan errors:** deferred; the log and exit code are enough for now.
- **`log` plus `env_logger`:** a sound choice, but each call must repeat the source name by hand.
  `tracing` was chosen for its automatic per-source context. Adoption was checked: 42,931 crates depend
  on `tracing`, against 30,776 for `log`.
- **Default `info` verbosity:** noisy for a tool most people run occasionally.

## Evidence
- Code: `detector/src/run.rs` (`run`, `RunError`, `RunReport::exit_code`), `detector/src/main.rs` (`init_logging`)
- test: run::tests::a_failing_source_does_not_stop_the_others_and_records_nothing_for_itself
- test: run::tests::a_broken_probe_is_a_failure_with_nothing_recorded
- test: run::tests::the_summary_reads_cleanly_for_each_outcome

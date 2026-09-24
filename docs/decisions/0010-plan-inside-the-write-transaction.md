---
id: 0010
title: Plan inside the write transaction
status: accepted
date: 2026-09-24
issues: [16, 17]
supersedes: []
related: [0007, 0009]
---

## Context
The first version of the writer read what was on record and decided the plan **before** taking the
write lock. Two overlapping runs (a manual scan plus, later, the monitor) could both plan from the
same old state and then both write: a duplicate `updated` event, or a unique-key failure for a tool
both saw as new. SQLite's default is also no busy timeout, so a second writer fails at once with
"database is locked".

## Decision
- `record_scan` reads what is on record, decides the plan, and writes it **inside one immediate
  transaction**, which takes the write lock up front. SQLite admits one writer at a time, so a second
  overlapping run waits, then plans from what the first run left and finds nothing new to record.
- Every connection sets `PRAGMA busy_timeout = 5000`, so a waiting writer waits up to five seconds
  instead of failing.
- The "first run" flag is still evaluated **once at the start of a run** and passed in. Evaluating it
  per source would make the second source of a first run look like a late arrival.
- **Deferred to the monitor (issue 17):** per-source lock files, coalescing several requests into one
  follow-up scan, and WAL mode.

## Consequences
- The write lock is held while planning, including the "is it really gone?" checks. They are short.
- Two overlapping scans of one source may repeat some scanning work, but they cannot duplicate events
  or corrupt anything.
- The database is still on SQLite's default journal mode.

## Alternatives rejected
- **Plan outside the transaction:** the behavior above.
- **Lock files now:** only needed once something can overlap automatically.
- **WAL now:** not needed yet.
- **An in-process mutex:** does not cover a second process.

## Evidence
- Code: `detector/src/store/apply.rs` (`record_scan`, `write_plans`), `detector/src/store/mod.rs` (`configure`)
- Checked by breaking it: with no busy timeout, the race test fails with `database is locked`; with planning done before the lock, the race test fails.
- test: store::apply::tests::two_connections_racing_with_the_same_old_view_record_each_change_exactly_once
- test: store::apply::tests::two_runs_that_saw_the_same_old_state_record_a_version_change_once
- test: store::apply::tests::two_runs_that_both_see_a_new_tool_insert_it_once_without_an_error
- test: store::apply::tests::two_runs_that_both_see_a_tool_missing_record_the_uninstall_once
- test: store::apply::tests::two_first_runs_started_together_do_not_double_baseline_or_invent_events
- test: store::apply::tests::a_failing_write_rolls_everything_back

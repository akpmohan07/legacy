---
id: 0007
title: Scans per source, and freshness
status: accepted
date: 2026-09-24
issues: [16, 17]
supersedes: []
related: [0005, 0006, 0008]
---

## Context
Unchanged tools record nothing (0005), so events alone cannot tell "nothing changed" from "Legacy
was not looking". Automatic triggers (issue 17) will often check a single source, not all of them.

## Decision
There is one `scans` row per scan of **one source**, with `run_id`, `source_id`, `triggered_by`,
`started_at`, and `finished_at`.

- The row is created **before** scanning, so a failed attempt stays visible.
- `finished_at` is set only by a successful `record_scan`, in the same transaction. A failed or
  crashed scan leaves it empty. A source confirmed absent counts as a success.
- So a source's **freshness** ("last checked") is its latest scan with a `finished_at`. It is derived,
  never stored.
- `run_id` is a random hex string shared by every scan of one run. It groups a run without a
  `runs` table.
- `triggered_by` is a contract in Rust (`TriggeredBy`), not a database `CHECK`. Only `manual` exists
  until the other triggers are built.
- Each event points to the scan that noticed it (`change_events.scan_id`); the event's source is
  reached through that scan.
- The whole installation has a heartbeat: `local_identity.last_seen_at`, updated once per run.

## Consequences
- Coverage questions ("was Homebrew checked in March?") are small queries over `scans`.
- Adding a trigger means adding a `TriggeredBy` value, not changing the schema.
- Scan errors are not stored in a table; they go to the log and the exit code.

## Alternatives rejected
- **One scan row per run:** a trigger that checks one source would leave per-source coverage ambiguous.
- **A `runs` table:** the `run_id` column is enough.
- **`last_seen_at` on `tools` and `sources`:** derivable, and it would rewrite every row on every scan.
- **`source_id` on `change_events`:** derivable through the scan; two places for one fact.
- **A database `CHECK` on `triggered_by`:** the set will grow, and SQLite cannot change a `CHECK` in place.

## Evidence
- Code: `detector/src/store/mod.rs` (`begin_scan`, `new_run_id`, `touch_identity`), `detector/src/store/apply.rs` (`write_plans`), `detector/src/domain/mod.rs` (`TriggeredBy`)
- Migration: `2026-09-23-232014_add_scans_change_events_and_lifecycle`
- test: store::apply::tests::a_failed_scan_leaves_no_trace_and_the_next_scan_is_still_the_baseline
- test: store::apply::tests::a_second_identical_scan_records_nothing
- test: store::apply::tests::run_ids_are_distinct_32_character_hex
- test: run::tests::a_failing_source_does_not_stop_the_others_and_records_nothing_for_itself
- test: run::tests::the_run_updates_the_installations_heartbeat
- test: domain::tests::triggered_by_keeps_unknown_values_instead_of_failing

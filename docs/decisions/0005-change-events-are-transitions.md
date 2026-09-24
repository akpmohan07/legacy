---
id: 0005
title: Change events are transitions
status: accepted
date: 2026-09-23
issues: [16]
supersedes: []
related: [0006, 0007, 0008]
---

## Context
Legacy exists to show how a toolset evolved over years. Storing a snapshot of every tool on every
scan would mostly repeat identical rows: roughly 44,000 rows for 40 tools scanned daily for three
years, against about 150–200 real changes.

## Decision
History is an append-only `change_events` table, written **only when a tool or a source changes
state**. Nothing is written for something unchanged.

- Three event types: `installed`, `updated`, `uninstalled`.
- A tool that disappears and returns keeps its row; its status flips back and another `installed`
  event is recorded. There is no separate `reinstalled` type, because a return is derivable from
  history (an `installed` event for a tool that already has earlier events).
- `changes` holds JSON `{"field":[old,new]}`, with `null` for a missing side. The keys mirror the
  tool's stored attributes. Only `version` is tracked for now (`TrackedField`). The database rejects
  invalid JSON; the shape is typed in Rust (`Changes`).
- Sources have the same life cycle and the same three event types. A source event has
  `tool_id` NULL; its source is reached through its scan (see 0007).
- Current state lives on the row: `tools.status` and `sources.status` are `installed` or
  `uninstalled`, the outcome of the latest event. `updated` leaves the status `installed`.

## Consequences
- Rebuilding the state at a past date means replaying events. The `checkpoints` table from the
  original plan is not built and not needed yet.
- Only version changes are events. Path or name changes are refreshed silently.
- A renamed formula or a changed bundle identifier shows up as an uninstall plus an install.

## Alternatives rejected
- **A snapshot row per tool per scan** (`scan_results`): about 200 times more rows, nearly all noise.
- **`previous_version` / `new_version` columns:** cannot grow beyond version without a migration.
- **A `reinstalled` event type:** derivable, and ambiguous with `brew reinstall` (same version, no change).
- **Separate source event types** (for example `source_appeared`): reuse the tool vocabulary instead.
- **`last_seen_at` on tools and sources, and `source_id` on events:** both are derivable from `scans`
  (0007), and would store one fact in two places.

## Evidence
- Code: `detector/src/domain/mod.rs` (`EventType`, `Status`, `Changes`), `detector/src/domain/plan.rs`, `detector/src/store/apply.rs`
- Migrations: `2026-09-23-232014_add_scans_change_events_and_lifecycle`, `2026-09-24-002031_rename_status_to_installed_uninstalled`
- Product context: `docs/IDEATION.md` (change events)
- test: domain::tests::changes_json_uses_old_new_pairs_with_null_for_a_missing_side
- test: domain::plan::tests::a_version_change_is_an_updated_event_with_old_and_new
- test: domain::plan::tests::an_uninstalled_tool_that_comes_back_is_installed_again
- test: domain::plan::tests::a_source_that_vanishes_is_uninstalled_and_keeps_its_last_version
- test: store::apply::tests::a_returning_tool_reuses_its_row_and_keeps_its_first_seen_date
- test: store::apply::tests::a_source_that_vanishes_and_returns_gets_events_without_touching_its_tools

---
feature: change-events
status: built
issues: [16]
decisions: [0005, 0006, 0007, 0008, 0010]
---

# Change events

## Purpose
Record when a tool or a source is installed, updated, or uninstalled, so its history can be shown
later. Nothing is recorded when nothing changed.

## Rules
| Situation | Result |
|---|---|
| First successful scan of a source | State is recorded, **no events** (the baseline) |
| New tool | Row inserted, `installed` event `{"version":[null,"2.3.2"]}` |
| Version differs | `updated` event `{"version":["2.1","2.2"]}` |
| On record, not found, confirmed gone | Status `uninstalled`, `uninstalled` event `{"version":["2.3.2",null]}` |
| On record, not found, still there or unknown | Nothing recorded; the tool is left as it was |
| Uninstalled tool found again | The same row flips back to `installed`; an `installed` event |
| Already uninstalled and still missing | Nothing (an uninstall is never repeated) |
| Unchanged | Nothing; the stored details are refreshed silently |
| A source appears, changes version, or vanishes | The same events on the source; its tools are not touched |

What counts as a change: **`version` only.** A changed path or name is refreshed silently. A renamed
formula or a changed bundle identifier appears as an uninstall plus an install.

## Status and dates
- `tools.status` and `sources.status` are `installed` or `uninstalled`: the outcome of the latest
  event. An `updated` event leaves it `installed`.
- `first_seen_at`: when Legacy first recorded the tool. A source's is set only when its first
  successful scan is recorded, which is what makes that scan the baseline.
- `installed_at`: the source's own install date, when it has one (Homebrew receipts do; apps do not yet).
- `occurred_at` (on an event): when Legacy noticed the change.

## Invariants
1. An uninstall is never recorded twice for the same tool.
2. A failed scan records nothing and leaves `scans.finished_at` empty.
3. Read, plan, and write happen inside one write transaction.
4. One record per identifier per scan. A duplicate resolves to the newest install, then the greater path.
5. A source that vanishes does not change its tools' status.

## Code map
`src/domain/plan.rs` decides (`plan_tools`, `plan_source`). `src/store/apply.rs` writes
(`record_scan`). `src/run.rs` calls it once per source. Value contracts are in `src/domain/mod.rs`.

## Guarded by
- test: store::apply::tests::the_first_scan_records_state_and_no_events
- test: store::apply::tests::a_second_identical_scan_records_nothing
- test: store::apply::tests::a_tool_installed_after_the_baseline_is_an_installed_event
- test: store::apply::tests::a_version_change_records_updated_and_stores_the_new_version
- test: store::apply::tests::an_uninstall_is_recorded_once_and_then_stays_quiet
- test: store::apply::tests::a_returning_tool_reuses_its_row_and_keeps_its_first_seen_date
- test: store::apply::tests::a_real_install_date_from_the_source_is_stored_as_utc_text
- test: store::apply::tests::a_failing_write_rolls_everything_back
- test: store::apply::tests::two_connections_racing_with_the_same_old_view_record_each_change_exactly_once
- test: domain::plan::tests::duplicate_identifiers_keep_the_newest_install_regardless_of_order
- test: domain::plan::tests::duplicate_identifiers_with_no_dates_break_ties_by_path_deterministically
- test: domain::plan::tests::an_unknown_version_staying_unknown_is_not_a_change
- test: domain::time::tests::formats_known_moments

## Verify by hand
Run `cargo run` twice: the first records the baseline, the second says "no changes". Then, only with
the owner's permission, `brew install <small formula>`, run again, and read the `installed` event
in `change_events`; remove it and run twice more to see one `uninstalled` event and then nothing.
(This was done live with `tree`.)

## Known limits
Only `version` is tracked. Apps report the marketing version, so a build-number-only update is missed.
Sources report no version yet, so a source never gets an `updated` event. Renames look like an
uninstall plus an install.

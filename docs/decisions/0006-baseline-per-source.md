---
id: 0006
title: Baseline per source, derived from sources.first_seen_at
status: accepted
date: 2026-09-24
issues: [16]
supersedes: []
related: [0005, 0007]
---

## Context
The first time Legacy sees a source, every tool in it looks new. Recording them as `installed`
events would say they were all installed on the day Legacy first ran, which is false and would
bury the real history.

## Decision
The **first successful scan of each source** records its tools (status, `first_seen_at`, and
`installed_at` where the source knows it) but **no events**. That is the baseline.

- A source is baselined exactly when `sources.first_seen_at` is set. It is written only in the same
  transaction as that source's first successful scan, so a failed first scan leaves it empty and the
  retry is still a baseline.
- "First run" (no source has `first_seen_at` yet) is evaluated once at the start of a run. In the
  first run a source's own appearance also gets no event, so the whole starting state is silent.
- A source that appears in a **later** run gets its own `installed` event, but its tools are still
  baselined without events.
- The baseline suppresses events, not state: it still corrects statuses.

## Consequences
- History for a source begins at its `first_seen_at`; anything earlier is unknown, and a real
  install date, when the source has one, is kept in `installed_at`.
- Tools recorded before events existed (the first 58 on the author's machine) were baselined by
  their next scan.

## Alternatives rejected
- **One global "first scan" rule:** a source that appears later would flood fake `installed` events.
- **"The source has no tools yet":** breaks when the first scan partly fails, and for a source that is
  legitimately empty at first.
- **A stored `baselined_at` column:** the same moment as `first_seen_at` in the normal case, so a
  second copy of one fact. It was added, then dropped (migration `2026-09-24-011443`).
- **A `baseline` event type:** an event that describes nothing that happened.

## Evidence
- Code: `detector/src/domain/plan.rs` (`plan_source`, the `baseline` argument of `plan_tools`), `detector/src/store/apply.rs` (`record_scan`), `detector/src/store/mod.rs` (`is_first_run`)
- test: domain::plan::tests::the_baseline_records_state_but_no_events
- test: domain::plan::tests::the_baseline_still_corrects_state_but_records_no_events
- test: domain::plan::tests::a_source_seen_on_the_very_first_run_is_recorded_without_an_event
- test: domain::plan::tests::a_source_that_appears_later_gets_an_installed_event
- test: store::apply::tests::the_first_scan_records_state_and_no_events
- test: store::apply::tests::a_failed_scan_leaves_no_trace_and_the_next_scan_is_still_the_baseline
- test: store::apply::tests::a_source_that_appears_later_gets_an_event_but_its_tools_are_baselined
- test: run::tests::the_first_run_baselines_every_source_without_events

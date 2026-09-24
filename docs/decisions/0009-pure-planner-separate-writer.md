---
id: 0009
title: A pure planner, a separate writer, a Store struct, and a lib plus binary
status: accepted
date: 2026-09-24
issues: [16]
supersedes: []
related: [0008, 0010]
---

## Context
The rules for what to record (0005–0008) must be testable without a database or a real filesystem.
Storage must have one boundary that SQL does not leak past. The automatic monitor (issue 17) will
be a second entry point, and integration tests need something to import.

## Decision
- **`domain/` is pure:** no database, no filesystem. It decides. `plan_tools` and `plan_source` turn
  what is on record, what a scan found, whether this is a baseline, and an "is it really gone?"
  function into a plan, which is plain data.
- **`store/` is the only code that talks to SQLite**, and it speaks in domain types. `store/apply.rs`
  writes a plan.
- **`scanner/` adapts the outside world** (filesystem, package managers) and never touches the database.
- **`run.rs` orchestrates** and contains no SQL.
- Storage is a concrete **`Store` struct, not a trait, for now.** Turning it into a trait later is a
  mechanical change, made when a second implementation or the review UI needs it.
- The crate is a **library plus a thin `main.rs`**, so the monitor and integration tests share the core.
- `DiscoveredTool` and `Absence` are plain data and live in `domain/`, re-exported from `scanner/`,
  so `domain/` never imports `scanner/`.

## Consequences
- The rules are covered by tests that use plain data only.
- This **deliberately defers the `WorkingStore` interface** described in `docs/IDEATION.md`. Revisit
  it when a second implementation exists.
- A new kind of decision means changing the planner and the writer together.

## Alternatives rejected
- **Rules interleaved with SQL:** untestable without a database.
- **A `WorkingStore` trait now:** an in-memory SQLite with the real migrations is a more faithful test
  than a fake, and swapping the engine is speculative.
- **A single binary crate:** the monitor and integration tests could not share it.
- **`run.rs` calling Diesel directly:** SQL would spread through the orchestration.

## Evidence
- Code: `detector/src/domain/plan.rs`, `detector/src/store/apply.rs`, `detector/src/run.rs`, `detector/src/lib.rs`
- test: domain::plan::tests::a_scan_after_applying_a_plan_would_record_nothing
- test: run::tests::a_later_run_records_installs_updates_and_uninstalls
- test: store::apply::tests::the_first_scan_records_state_and_no_events

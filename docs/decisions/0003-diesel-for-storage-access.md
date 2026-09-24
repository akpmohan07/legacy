---
id: 0003
title: Diesel for storage access
status: accepted
date: 2026-09-22
issues: [13]
supersedes: []
related: [0004, 0009]
---

## Context
The store is SQLite (recorded in `docs/IDEATION.md`). The question was how Rust code talks to it: raw
`rusqlite`, or an ORM. The schema is expected to grow across many tables over years, maintained by one
person and by agents.

## Decision
Use **Diesel with SQLite**. `schema.rs` is generated from the migrated database, so a renamed or removed
column fails at build time instead of when that code path next runs.

- Raw SQL only where the query builder cannot express something (currently one `sql_query`, for the
  random run id).
- Async ORMs are not used: the detector is synchronous, and they would force an async runtime.

## Consequences (accepted costs)
- A development tool, `diesel_cli`, plus a git-ignored `.env` with `DATABASE_URL` for generating
  `schema.rs`. End users never need either (see 0004).
- The generated `schema.rs` is committed and must not be edited by hand.
- SQLite's `INTEGER PRIMARY KEY` is inferred as nullable, so model ids are `Option<i32>`.
- The query builder is a second vocabulary over SQL, and a raw `sql_query` gives up the compile-time check.

## Alternatives rejected
- **`rusqlite` plus `rusqlite_migration`:** simpler and needs no command-line tool, but column errors
  appear only at run time as the schema grows. It was the first recommendation and was reversed after
  weighing maintainability.
- **`sqlx` and `sea-orm`:** built around an async runtime. For scale, crates.io downloads when checked
  were 36.9 million for `diesel`, 150.1 million for `sqlx`, and 25.3 million for `sea-orm`; adoption
  was not the concern.
- **A hand-written `schema.rs`:** removes the guarantee that makes Diesel worth it.
- **Other engines** (Postgres, DuckDB, libSQL, embedded key-value stores): reviewed and rejected for
  this workload. The engine choice itself is recorded in `docs/IDEATION.md`.

## Evidence
- Code: `detector/Cargo.toml` (Diesel with the `sqlite` and `returning_clauses_for_sqlite_3_35` features, and the bundled SQLite), `detector/diesel.toml`, `detector/src/schema.rs`, `detector/src/models.rs`
- test: store::apply::tests::the_first_scan_records_state_and_no_events

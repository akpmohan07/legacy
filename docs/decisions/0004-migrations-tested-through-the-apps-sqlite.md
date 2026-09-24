---
id: 0004
title: Migrations are embedded and tested through the app's own SQLite
status: accepted
date: 2026-09-23
issues: [16]
supersedes: []
related: [0005]
---

## Context
The `diesel` command-line tool links macOS's system SQLite (3.43.2), where `PRAGMA foreign_keys`
is **off**. The app links a SQLite compiled into its own binary (`libsqlite3-sys` with the `bundled`
feature), whose build script sets `-DSQLITE_DEFAULT_FOREIGN_KEYS=1`, so foreign keys are **on**.
A migration that dropped and recreated the `sources` table passed through the command-line tool and
then failed inside the app with `FOREIGN KEY constraint failed`. The change rolled back and no data
was lost, but the test had proved less than it seemed to.

## Decision
- Migrations are embedded in the binary (`embed_migrations!`) and applied automatically when the
  store opens. End users never install `diesel_cli`; contributors use it only to generate
  migrations and `schema.rs`.
- **Every migration is tested through the app's own SQLite before it touches real data:** back up the
  real database, copy it into a throwaway home folder, run the built binary with `HOME` pointing
  there (the app finds its data folder through `HOME`), and only then apply it for real. Also run
  `diesel migration redo` to exercise `down.sql`.
- **Never drop a parent table** (one that other tables reference). Add nullable columns in place with
  `ALTER TABLE … ADD COLUMN`. Rebuild only child tables, because SQLite cannot add a column whose
  default is not a constant (such as `datetime('now')`). To change a column-level `CHECK`, drop the
  column and add it again.
- A committed migration is never edited. Change it with a new migration.
- `schema.rs` is generated from the migrated database and is not edited by hand. Model field order
  follows the generated column order (a dropped and re-added column moves to the end).

## Consequences
- Adding a migration takes a few more steps, and they matter.
- Schema changes that need a parent-table rebuild are constrained. The one untested way around it is
  a migration with `run_in_transaction = false` and `PRAGMA foreign_keys = OFF` around a manual
  transaction.

## Alternatives rejected
- **`PRAGMA defer_foreign_keys = ON` inside the migration:** tried, and the commit still failed.
  SQLite's deferred-violation counter does not go back down when a dropped parent table is recreated
  under the same name.
- **A hand-maintained `schema.rs` to avoid needing `diesel_cli`:** generation from the migrated
  database is what keeps the file correct.
- **Testing only through the command-line tool:** the failure above is exactly what that misses.

## Evidence
- Code: `detector/src/store/mod.rs` (`MIGRATIONS`, `open_store`)
- Migrations: `2026-09-23-232014_add_scans_change_events_and_lifecycle` (in-place `ADD COLUMN`, child-table rebuild), `2026-09-24-002031_rename_status_to_installed_uninstalled` (drop and re-add a column), `2026-09-24-011443_drop_sources_baselined_at`
- Observed: `libsqlite3-sys` 0.35.0 `build.rs` compiles with `-DSQLITE_DEFAULT_FOREIGN_KEYS=1`; the system SQLite reports `PRAGMA foreign_keys` = 0
- test: store::apply::tests::the_database_enforces_foreign_keys_like_the_shipped_app

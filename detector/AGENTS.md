# Detector guide for agents

The Rust detector: it finds the tools on this Mac and records how they change. This file is a cheat
sheet. The reasons live in `../docs/decisions/`, and behavior lives in `../docs/features/` and
`../docs/sources/`. Link to them; do not restate them here.

## Commands
- `cargo test`: 61 tests in about a second. They use an in-memory SQLite and fake folders and touch
  nothing real.
- `cargo run`: scans **this machine** and writes the **real** database at
  `~/Library/Application Support/Legacy/legacy.db`. `LEGACY_LOG=debug` for details. Exit code 0 ok,
  1 a source failed, 2 database error (0011).
- `diesel migration run|redo|revert`: a development tool. It needs `detector/.env` containing
  `DATABASE_URL=dev.db` (git-ignored) and regenerates `src/schema.rs`.

## Before you run anything
Never run installs, uninstalls, `launchd` jobs, or anything else that changes the machine without
asking first. Back up the real database before touching it (see 0012 for why).

## Module map
| Path | Owns |
|---|---|
| `src/scanner/` | discovery adapters (`Scanner` trait, registry, `macos/`); never touch the database |
| `src/domain/` | pure rules and value contracts (`plan.rs`, enums, `Changes`); no database, no filesystem |
| `src/store/` | the only code that talks to SQLite (`Store`, `apply.rs`) |
| `src/run.rs` | one run: probe, scan, record, report; no SQL |
| `src/identity.rs` | the one-time machine identity |
| `migrations/`, `src/schema.rs` | SQL history; `schema.rs` is generated, never hand-edited |

## Invariants (do not break; the number is the decision record)
1. A tool becomes `uninstalled` only when its scanner confirms it is gone (0008).
2. A failed scan records nothing and leaves `scans.finished_at` empty (0007, 0011).
3. Read, plan, and write happen inside one write transaction (0010).
4. Only `store/` has SQL. `domain/` stays pure. Scanners never touch the database (0001, 0009).
5. Events are written only for transitions, and a source's first successful scan records none (0005, 0006).
6. Never drop a parent table in a migration. A committed migration is never edited (0004).

## Checklists
**Add a scanner.** New file under `src/scanner/<os>/`. Implement `source_name` (it must match a seeded
`sources.name`; if not, add a migration that seeds it), `probe`, and `scan`. `scan` returns an error, never
an empty success, when it cannot read its source. Override `confirm_absent` if a path check is wrong for
it. Register it in `ScannerRegistry::build` inside the `cfg` block. Test the core logic against a fake
folder (see the existing scanner tests). Add `../docs/sources/<name>.md`.

**Add a migration.** Follow 0004. `diesel migration generate <name>`, write `up.sql` and `down.sql`, run
`migration run` and `redo`, then update the field order in `src/models.rs` to match the regenerated
`schema.rs`. `touch src/store/mod.rs` and rebuild so the embedded copy refreshes. Then test through the
app's own SQLite: copy the real database to `<tmp>/Library/Application Support/Legacy/legacy.db` and run
`HOME=<tmp> ./target/debug/legacy-detector`. Check `PRAGMA integrity_check` and `PRAGMA foreign_key_check`.
Only then run it for real, with a backup.

**Add a value to a stored enum** (event type, status, trigger). Change the Rust enum in `src/domain/mod.rs`
first, since the contract lives in code. Add a database `CHECK` only for a tiny closed set. Add parse and
round-trip tests.

## Docs
- A decision an agent could break, or keep re-proposing, gets a record in `../docs/decisions/` (read its
  README). To reverse one, add a new record and mark the old one `superseded`.
- Cite tests as `- test: path::to::test`, so they can be checked against `cargo test -- --list`.
- Timestamps use an `_at` suffix and are UTC text (`../docs/LESSONS.md`). Names of tests read as sentences.

## Known gaps
`identity.rs` is not gated per OS (it calls `system_profiler`). Failures while opening the store panic
(exit 101). The exit-2 path has no test. The automatic monitor is not built (0012). Only two sources exist:
`application` and `homebrew-cellar`.

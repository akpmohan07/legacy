# Decision records

Engineering decisions for the detector (`detector/`), one file per decision. Product and strategy
decisions stay in `docs/IDEATION.md`; this folder does not restate them.

## When a decision gets a record

Only when an agent (or person) could **break correctness, data integrity, or the architecture** by
getting it wrong, or would keep **re-proposing an alternative we already rejected**. Naming and
other cosmetic choices belong in the feature doc, `detector/AGENTS.md`, or `docs/LESSONS.md`.

## Conventions

- File name: `NNNN-short-title.md`. The number never changes; a replaced decision is marked
  `superseded` and points to its replacement.
- Front matter: `id`, `title`, `status` (`proposed` | `accepted` | `superseded` | `deprecated`),
  `date`, `issues`, `supersedes`, `related`.
- Sections, in order: **Context**, **Decision**, **Consequences**, **Alternatives rejected**, **Evidence**.
- Evidence lines that name a test use exactly `- test: path::to::test`, so they can be checked
  against `cargo test -- --list`.
- Keep each record short. Behavior rules belong in `docs/features/`, not here.

## Index

| # | Decision | Status |
|---|---|---|
| [0001](0001-scanner-architecture-and-portability.md) | Scanner architecture and portability | accepted |
| [0002](0002-single-user-single-machine.md) | Single user, single machine; one `local_identity` row | accepted |
| [0003](0003-diesel-for-storage-access.md) | Diesel for storage access | accepted |
| [0004](0004-migrations-tested-through-the-apps-sqlite.md) | Migrations are embedded and tested through the app's own SQLite | accepted |
| [0005](0005-change-events-are-transitions.md) | Change events are transitions | accepted |
| [0006](0006-baseline-per-source.md) | Baseline per source, derived from `sources.first_seen_at` | accepted |
| [0007](0007-scans-per-source-and-freshness.md) | Scans per source, and freshness | accepted |
| [0008](0008-uninstall-only-when-confirmed-gone.md) | Uninstall only when confirmed gone | accepted |
| [0009](0009-pure-planner-separate-writer.md) | A pure planner, a separate writer, a Store struct, and a lib plus binary | accepted |
| [0010](0010-plan-inside-the-write-transaction.md) | Plan inside the write transaction | accepted |
| [0011](0011-error-policy-and-exit-codes.md) | Error policy, exit codes, and logging | accepted |
| [0012](0012-automatic-trigger-mechanism.md) | Automatic trigger mechanism | proposed |

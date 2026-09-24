# Feature docs

What each detector feature does, exactly, and how to verify it. One file per feature. The reasons
behind a rule live in `../decisions/`; do not restate them here.

## Conventions
- Front matter: `feature`, `status` (`planned` | `built`), `issues`, `decisions`.
- Sections, in order: **Purpose**, **Rules**, **Invariants** (must stay true), **Code map**,
  **Guarded by** (tests, as `- test: path::to::test`), **Verify by hand**, **Known limits**.
- A planned feature says so at the top and lists only what is decided and what is unknown.
- If you change a rule, change its tests and this file in the same commit.

## Index
| Feature | Status |
|---|---|
| [change-events](change-events.md) | built |
| [scan-run](scan-run.md) | built |
| [local-identity](local-identity.md) | built |
| [automatic-monitoring](automatic-monitoring.md) | planned |

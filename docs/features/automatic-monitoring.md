---
feature: automatic-monitoring
status: planned          # nothing is built yet
issues: [17]
decisions: [0012, 0010, 0007]
---

# Automatic monitoring (planned)

## Purpose
Start a scan by itself when something changes on the machine, whatever caused it (a person, a script,
or an AI agent), so the history stays current without anyone running a scan.

## What is decided
See `../decisions/0012-automatic-trigger-mechanism.md` (status `proposed`): a small resident process
using `notify` with debouncing, each source scanned on its own, a catch-up scan at start, a scheduled
backstop, and one `launchd` plist to start it at login. Scan results and events follow the existing
rules (`change-events`, `scan-run`); only the trigger is new.

## What is needed before it can be built
- The `Scanner` trait has no way to name the paths a source wants watched.
- `TriggeredBy` needs `watcher`, `scheduled`, and `startup` values.
- Per-source locking and coalescing of overlapping scans (deferred in decision 0010).

## Not yet known
Noise from watching `/Applications` (app bundles write many files inside), the watcher's idle memory
and CPU, how it is started at login and kept alive, and what to do about a source that keeps failing.

## Guarded by
Nothing yet.

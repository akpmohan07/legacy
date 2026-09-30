---
id: 0013
title: Publishing recorded change events to consumers
status: proposed          # agreed in discussion 2026-09-30; nothing built yet
date: 2026-09-30
issues: [21, 22]
supersedes: []
related: [0005, 0010, 0012]
---

## Context
Several features want to react when Legacy records a change: a desktop notification, regenerating
`legacy.json`, a live review UI, user hooks, a digest. Wiring each one into the scan loop would couple the
core to every feature. The change events are already durable rows in `change_events`, committed before
anything else happens (0010).

## Decision (proposed)
- After a scan commits, the core **publishes** the events that scan recorded to registered **consumers**.
  The core does not know who they are. A consumer is a small interface (for example
  `ChangeListener::on_events(&[RecordedEvent])`), implemented at the edge, like scanners.
- **`change_events` is the only source of truth.** Every other way of emitting events (in-process calls,
  a CLI stream, SSE, hooks) is a view of that table, never a second copy.
- **Start with in-process, non-durable delivery.** Events themselves are never lost; only a delivery can
  be missed (a crash between commit and dispatch, a failing consumer, a disabled consumer). A consumer
  failure is logged and ignored and never affects scanning or the history.
- **The rule for consumers:**
  - *Fire-and-forget* (notification, live UI): non-durable delivery is fine.
  - *Derived from the table* (export, eras, digest): rebuild from `change_events` when triggered; a missed
    delivery only delays them.
  - *Must see every event once* (hooks that post outside, feeds): may not rely on non-durable delivery.
    They either derive from the table or get a per-consumer **cursor** (the last event id handled).
- **Nothing leaves the machine by default.** A consumer that sends data out is publishing, so it is
  opt-in and explicit (IDEATION privacy rules); Legacy's own consumers stay local.
- The first consumer is the desktop notifier (issue 22): opt-in, `notify-rust` behind a `Notifier`
  interface.

## Consequences
- The scan loop calls "publish", not any feature directly. Adding a consumer does not touch the core.
- Adding cursors later changes only the dispatcher and adds a small table; recorded events need no
  migration or recovery.
- Consumers see `occurred_at`, so issue 20 (notice time recorded instead of change time) matters more.
- The writer must return which events it wrote, not only counts, so consumers get names and versions.

## Alternatives rejected
- **Call each feature from the scan loop:** couples the core to every consumer.
- **Cursors from the start:** durable delivery for a single fire-and-forget consumer is machinery
  without a user.
- **An append-only JSONL events file:** duplicates `change_events`; the two could drift.
- **OS event buses** (macOS distributed notifications, Linux D-Bus): a different API per OS for little gain
  over a CLI stream or hooks.

## Evidence
- No code yet. Discussion of use cases and emit mechanisms: issue 21.

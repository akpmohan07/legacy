---
id: 0012
title: Automatic trigger mechanism
status: proposed          # leaning; nothing is built yet
date: 2026-09-23
issues: [17]
supersedes: []
related: [0007, 0010]
---

## Context
Issue 17 needs a scan to start by itself when the machine changes, for any cause: a person, a script,
or an AI agent. Because unchanged scans record nothing, a trigger only needs to say *when* to scan.
The scan-and-diff logic (0005–0010) is the same whatever starts it.

## Decision (proposed)
- A small resident process using the Rust **`notify`** crate with debouncing (`notify-debouncer-mini`).
- Each source names the paths worth watching, so a change scans **only that source**. (The `Scanner`
  trait does not have this yet.)
- A **catch-up scan on start** and a **scheduled backstop scan** cover anything missed.
- On macOS, one `launchd` plist only starts the watcher at login and keeps it alive.
- Each scan records `triggered_by` (0007): `watcher`, `scheduled`, or `startup`.

## What was found
- **`launchd` `WatchPaths` works.** In a live test, creating, editing, renaming, and deleting files and
  folders (including one level inside a subfolder) each started the job, and doing nothing did not. But
  it passes the program **no path and no kind of change**, it starts a program at most about every ten
  seconds, and it is macOS-only. `launchctl print` shows it is backed by FSEvents through
  `UserEventAgent`, not `kqueue` as first believed.
- **Homebrew has no install hook** (an open request, Homebrew/brew#2202). npm's `.hooks` only fire if
  the package itself defines a script, so they miss some installs.
- **A shell `preexec` hook fires only in an interactive shell.** Commands run by scripts or AI agents
  are invisible to it, so it suits usage capture, not change detection.
- **`notify`** works on macOS, Linux, and Windows, reports the kind of change and the paths, and has a
  debounce companion. It costs a resident process that needs supervising.
- **Not measured:** how noisy `/Applications` is under `notify` (app bundles write many files inside),
  and its idle memory and CPU.

## Consequences
- A small resident process is a new runtime commitment. Everything built so far starts, scans, and exits.
- Per-source locking and coalescing of overlapping scans, deferred in 0010, become necessary here.
- **Do not run persistent `launchd` experiments on someone's machine without asking.** The machine hung
  and restarted during the `WatchPaths` test. Nothing tied it to the job (it had not run since the
  restart), and the cause was never established. The experiment job was removed.

## Alternatives rejected
- **`launchd` `WatchPaths` alone:** no path detail, needs one plist per source, macOS-only.
- **Shell-hook triggers:** blind to scripts and AI agents.
- **Polling only:** kept as the backstop, but too slow to be the main trigger.
- **The macOS Endpoint Security framework:** would see every process, but it is understood to need a
  special Apple entitlement and a root extension. Not investigated in depth.

## Evidence
- No code yet. The experiment files were deleted.
- Product context: `docs/IDEATION.md` (Open, not yet decided).

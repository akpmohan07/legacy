# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

Pre-build ideation and research for **Legacy** (formerly named "Loadout" — the rename is final, see Naming below). Legacy will be a tool that auto-detects a developer's full tool/hardware/app inventory on their own machine and presents it as a shareable, "Wrapped"-style retrospective of their tool history over time, rather than a live dashboard.

There is no code yet — this repo currently holds only the planning documents below. No build, lint, or test commands apply.

## The documents

- **`NORTH-STAR.md`** — one-page distilled reference: the pitch, the three value pillars (inventory → sharing → credibility, in that sequencing order), the feature list, and who benefits. Extracted from the other two docs, not a source of new reasoning — read this first for a fast refresh, read `IDEATION.md`/`research-log.md` for the "why" behind anything in it. Keep it in sync when `IDEATION.md`'s locked decisions change; it should never contradict them.
- **`IDEATION.md`** — the standing decisions doc. Read this first. Contains the problem statement, feasibility findings, locked product decisions, and open questions. Treat sections marked "locked" or "decided" as settled — don't re-litigate them without new information that actually contradicts the decision.
- **`ARCHITECTURE.md`** — the system-level component map: what gets built (detector, local SQLite store, `legacy.json` export, template renderer, distribution add-ons, registry, the parked marketplace), how they connect, and what's local vs. published vs. deferred. Diagram is Mermaid, kept in a markdown file for git-diffability and native GitHub rendering — no external diagramming tool. Keep in sync with `IDEATION.md`'s locked decisions the same way `NORTH-STAR.md` is.
- **`research-log.md`** — a chronological log of *how* the thinking evolved, separate from `IDEATION.md`'s polished conclusions. Each entry has a `*Thread:*` line stating where it came from and what it opens into, so the log reads as one continuous line of reasoning. Update rule stated in the file: no schedule — add an entry only when something shifts the thinking (new evidence, a reframe, a decision/pivot), skip entries that just restate the last one. The final `### Open` section is always last and gets *rewritten in place*, not appended to, as the live edge of the thinking moves. **Never retroactively edit a dated entry's wording (e.g. to rename something) — the log preserves what was actually said/decided at that point in time; only the live `### Open` section gets terminology updates.**
- **`related-products.md`** — prior art / competitor research, organized by why each item came up (not a flat competitor list). Check here before claiming something is novel — most adjacent tools already researched are cataloged with the specific gap that still holds against Legacy.

When adding research or making a new decision in a future session, follow the same pattern: update the relevant section of `IDEATION.md` for the standing conclusion, and add a dated entry to `research-log.md` for the reasoning trail that produced it.

## Task tracking: every task is a GitHub issue on the Legacy project

This project tracks work in **GitHub Issues + the "Legacy" GitHub Project** (github.com/users/akpmohan07/projects/4, `akpmohan07/legacy` repo), not just in conversation or in the markdown docs. Don't let a concrete task, decision, or build item stay only in chat — create it as an issue and keep it moving. This is a standing instruction; do it without asking each time.

- **An issue is a transaction, not a checklist export.** Create one only for a task that was actually discussed and resolved-to-a-decision-point in conversation — never by mechanically converting a list already sitting in `IDEATION.md`/`research-log.md` into issues in one batch. (History: issues #3–#11 were bulk-created straight from the `IDEATION.md` feasibility checklist without real discussion behind each one, and were deleted for exactly that reason — 2026-09-19. Issues #1, #2, #12 survived because each traces back to genuine back-and-forth in `research-log.md`.) A backlog item sitting in the docs stays prose-only until it's actually talked through, even if it looks issue-shaped.
- **Create on surfacing.** Once a task/decision has been discussed this way, run `gh issue create --repo akpmohan07/legacy` with a clear title, then add it to the project: `gh project item-add 4 --owner akpmohan07 --url <issue-url>`.
- **Label consistently.** Existing convention: `phase:0` (decisions before any code), `phase:1` (detector implementation) — add a new `phase:N` label when a later stage starts (renderer, registry, etc.) rather than overloading an existing one. Type labels: `type:decision`, `type:idea`, `type:chore`, `type:tech-debt`. Add `privacy` for anything touching the allowlist/publishing hard rules.
- **Status has three stages: Todo → In Progress → Done.** New items start at Todo. Move an item to In Progress when work on it actually starts in a session, and to Done — plus close the issue — the moment it's resolved. Don't leave a finished item sitting at Todo or In Progress.
  - Update via: `gh project item-edit --project-id PVT_kwHOAi9PQc4BNJUo --id <item-id> --field-id PVTSSF_lAHOAi9PQc4BNJUozg8OcQA --single-select-option-id <option-id>` — option ids: Todo=`f75ad846`, In Progress=`47fc9ee4`, Done=`98236657`.
  - Close the issue too when marking Done: `gh issue close <number> --repo akpmohan07/legacy`.
- **Keep the docs and the board in sync.** When an issue's status changes, reflect it in `IDEATION.md`'s locked-decisions / `research-log.md`'s `### Open` section too (and vice versa) — the board is the task-status source of truth, the docs are the reasoning/decision record; they should never contradict each other.

## Locked decisions (don't re-derive these)

- **Name: Legacy.** Final as of 2026-09-19, with known, accepted risks (npm name `legacy` is taken by an unrelated package; "legacy" is a common GitHub convention for "deprecated" — positioning should lean into that ambiguity rather than avoid it). Do not reopen naming.
- **Two separate problems, ranked:** Problem 1 (tool amnesia — validated, build first) and Problem 2 (shareable "what's your stack" — weaker evidence, secondary, cut before the detector if forced to choose). Problem 3 (hiring-credibility signal) is a third, later direction — fully parked until Problem 1/2 have real adoption; don't pick it back up speculatively.
- **v1 detection scope: CLI tools + desktop apps only** (macOS, via Homebrew/npm/Cargo receipts and `/Applications` `Info.plist` reads). Browser extensions and hardware are feasible and researched but deferred to v1.1.
- **Build order: detector and `legacy.json` format before the renderer.** The simulated-desktop UI is a deferred, de-risked-by-prior-art step, not the next thing to build.
- **Deployment model: decentralized, uses.tech-style.** No hosted platform, no accounts, no server database — a cloned template site reads each person's own `legacy.json` from their own repo/gist.
- **Privacy model: allowlist-by-source, not denylist-by-name.** Tools sourced from public registries (Homebrew core, npm, crates.io) default visible; anything `.local`/`.path`/custom-tap-sourced defaults hidden. Detection is always automatic; **publishing is never automatic** — requires explicit human review every time.
- **Local storage: SQLite as the detector's private working store; `legacy.json` is a generated, filtered export of it — not the same thing.** SQLite is local/embedded (a file, nothing to host), so this doesn't reopen the "no database" deployment decision above, which is about a server backend. See `ARCHITECTURE.md` for the full pipeline.

## Open — not yet decided

- Detector stack (leaning Node/TS over Swift, unconfirmed).
- `legacy.json` schema and the SQLite table structure it's exported from (the one artifact every downstream idea depends on — design carefully, keep the exported format stable/versioned since a future registry/aggregator depends on parsing it indefinitely).
- Repo layout: single repo with detector + template as separate packages, vs. two repos.

See `research-log.md`'s final `### Open` section for the fullest, most current version of this list.

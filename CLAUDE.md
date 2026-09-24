# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

Pre-build ideation and research for **Legacy** (formerly named "Loadout" — the rename is final, see Naming below). Legacy will be a tool that auto-detects a developer's full tool/hardware/app inventory on their own machine and presents it as a shareable, "Wrapped"-style retrospective of their tool history over time, rather than a live dashboard.

The repo holds the planning documents below and the Rust detector in `detector/`. **For any detector work (build, test, invariants, checklists), read `detector/AGENTS.md` first.**

## The documents

Planning docs live in `docs/`; this file stays at repo root (that's where Claude Code looks for it). **See `docs/manifest.md` for what each file owns, what it must never contain, and the single-home rule that keeps them from drifting back into duplication.** Filenames: `docs/IDEATION.md` (product decisions + why), `docs/decisions/` (detector engineering decisions), `docs/ARCHITECTURE.md` (structure only), `docs/research-log.md` (chronological narrative), `docs/NORTH-STAR.md` (one-page pitch), `docs/related-products.md` (prior art), `docs/LESSONS.md` (reusable engineering guidelines), `docs/manifest.md` (this index's source), `CLAUDE.md` (this file — operational guidance only).

When adding research or making a new *product* decision in a future session (a detector engineering decision gets a record in `docs/decisions/` instead, see its README): update `docs/IDEATION.md` for the standing conclusion + reasoning, add a dated entry to `docs/research-log.md` for the trail that produced it, update `docs/ARCHITECTURE.md` only if the *structure* changed — and check `docs/manifest.md` before writing an explanation anywhere, since that's the rule that prevents the duplication this section used to have.

## Task tracking: every task is a GitHub issue on the Legacy project

This project tracks work in **GitHub Issues + the "Legacy" GitHub Project** (github.com/users/akpmohan07/projects/4, `akpmohan07/legacy` repo), not just in conversation or in the markdown docs. Don't let a concrete task, decision, or build item stay only in chat — create it as an issue and keep it moving. This is a standing instruction; do it without asking each time.

- **An issue is a transaction, not a checklist export.** Create one only for a task that was actually discussed and resolved-to-a-decision-point in conversation — never by mechanically converting a list already sitting in `docs/IDEATION.md`/`docs/research-log.md` into issues in one batch. (History: issues #3–#11 were bulk-created straight from the `docs/IDEATION.md` feasibility checklist without real discussion behind each one, and were deleted for exactly that reason — 2026-09-19. Issues #1, #2, #12 survived because each traces back to genuine back-and-forth in `docs/research-log.md`.) A backlog item sitting in the docs stays prose-only until it's actually talked through, even if it looks issue-shaped.
- **Create on surfacing.** Once a task/decision has been discussed this way, run `gh issue create --repo akpmohan07/legacy` with a clear title, then add it to the project: `gh project item-add 4 --owner akpmohan07 --url <issue-url>`.
- **Label consistently.** Existing convention: `phase:0` (decisions before any code), `phase:1` (detector implementation) — add a new `phase:N` label when a later stage starts (renderer, registry, etc.) rather than overloading an existing one. Type labels: `type:decision`, `type:idea`, `type:chore`, `type:tech-debt`. Add `privacy` for anything touching the allowlist/publishing hard rules.
- **Status has three stages: Todo → In Progress → Done.** New items start at Todo. Move an item to In Progress when work on it actually starts in a session, and to Done — plus close the issue — the moment it's resolved. Don't leave a finished item sitting at Todo or In Progress.
  - Update via: `gh project item-edit --project-id PVT_kwHOAi9PQc4BNJUo --id <item-id> --field-id PVTSSF_lAHOAi9PQc4BNJUozg8OcQA --single-select-option-id <option-id>` — option ids: Todo=`f75ad846`, In Progress=`47fc9ee4`, Done=`98236657`.
  - Close the issue too when marking Done: `gh issue close <number> --repo akpmohan07/legacy`.
- **Keep the docs and the board in sync.** When an issue's status changes, reflect it in `docs/IDEATION.md`'s locked-decisions / `docs/research-log.md`'s `### Open` section too (and vice versa) — the board is the task-status source of truth, the docs are the reasoning/decision record; they should never contradict each other.

## Locked decisions (index — full text and reasoning lives in `docs/IDEATION.md`)

See `docs/IDEATION.md`'s "Product decisions locked so far" for the actual content. Index, so a session knows what's settled without opening it:

- Name: Legacy. Do not reopen.
- Problem sequencing: Problem 1 (tool amnesia) build-first; Problem 2 (sharing) secondary; Problem 3 (credibility) parked until 1/2 have adoption.
- Platform: macOS-only for v1.
- v1 detection scope: CLI tools + desktop apps only; browser extensions/hardware deferred to v1.1.
- Build order: detector + `legacy.json` format before the renderer.
- Deployment: decentralized, uses.tech-style — no hosted platform, no accounts, no server database, GitHub Pages for both the registry and each cloned template.
- Privacy: allowlist-by-source, not denylist-by-name; publishing is never automatic.
- Local storage: private `WorkingStore` (SQLite v1 adapter, behind an interface) + public exports (`legacy.json` required, `legacy.db` optional), never the same artifact.
- Registry updates: push-via-PR preferred (fork-scoped PAT, self-service), scheduled pull as fallback.
- **Detector language: Rust.** Local review UI: also Rust (`axum`/`warp`-served localhost page, not a native app yet). Renderer stays TS/JS, unaffected.
- Continuous usage capture (v-next): a Rust shell hook, same methodology as `atuin`/`mcfly` — tool name + timestamp only, never the raw command line.
- **Repo layout: single repo (this one), monorepo-style.** `detector/` (Rust), `renderer/` (TS, later), `schema/` (shared).

## Decision-making heuristic (standing rule as of 2026-09-20)

Before locking any architecture decision, weigh it against **whole-system fit**, **scalability** (access-pattern behavior over years of history, not raw throughput — this is one person's data), and **multiplatform support** (macOS-only is locked for *v1 scope*, but a technical choice shouldn't quietly foreclose Windows/Linux later). Don't mark something "locked" after a single pitch — see `docs/IDEATION.md`'s "Local storage engine options" for what happened the first time this wasn't followed (SQLite got locked, walked back, re-decided properly). When real uncertainty remains even after that analysis, prefer locking *behind an interface* over leaving something fully open or welding it in.

## Open — not yet decided

Canonical list lives in `docs/IDEATION.md`'s "Open, not yet decided" — don't maintain a separate copy here; it drifts. (`docs/research-log.md`'s final `### Open` section is different in kind — the narrative live-edge, not a duplicate — and can stay as its own thing.)

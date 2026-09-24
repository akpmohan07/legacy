# Legacy — Documentation Manifest

What each planning doc in this repo owns, so any piece of information has exactly one home. Written 2026-09-20, after duplicated decision text across `CLAUDE.md`/`ARCHITECTURE.md`/`IDEATION.md` made a single correction (the SQLite walk-back) cost four file edits instead of one.

**Not to be confused with `legacy.json`/`legacy.db`** — those are the *product's* own output manifest, described in `IDEATION.md`/`ARCHITECTURE.md`. This file is about the repo's own planning docs.

**Location: everything in this table lives in `docs/`, except `CLAUDE.md`, which stays at repo root** (2026-09-22 — that's where Claude Code looks for it, so it can't move with the rest).

## The rule

Before writing an explanation anywhere, ask: **which file owns this kind of information?** Put it only there. Everywhere else gets a pointer (doc name + section), never a restatement. If a "why" sentence is being written in anything other than `IDEATION.md` (product/strategy) or `decisions/` (detector engineering), that's the signal something is about to get duplicated again.

## Ownership

| File | Owns | Never contains | May reference |
|---|---|---|---|
| **`IDEATION.md`** | **Product and strategy** decisions **and their full reasoning** — the "why." Canonical "Open, not yet decided" list. (The detector's *engineering* decisions live in `decisions/`.) | — this is the root; nothing outranks it | — |
| **`decisions/`** | **Engineering decisions for the detector**, one record each (context, decision, consequences, rejected alternatives, evidence) — the "why" behind how `detector/` is built. Only decisions an agent could break or keep re-proposing; see `decisions/README.md`. | Product/strategy reasoning (that's `IDEATION.md`); behavior rules (those go in feature docs) | `IDEATION.md` for product context; tests and code by name |
| **`ARCHITECTURE.md`** | **Structure only** — the component diagram, what talks to what, local/published/deferred. The "what," not the "why." | Decision reasoning/justification | `IDEATION.md` for why any piece is shaped that way |
| **`research-log.md`** | The **chronological narrative** — dated entries, never rewritten retroactively, including dead ends and reversed calls (e.g. the SQLite walk-back). A journal, not a current-state snapshot. | A "current answer" stated without its dated trail framing | Can quote `IDEATION.md`'s conclusions *inside* a dated entry, as part of that entry's narrative |
| **`NORTH-STAR.md`** | The **one-page distilled pitch** — fast orientation, nothing more. | Any new reasoning or decision detail not already in `IDEATION.md` | `IDEATION.md`/`research-log.md` for "why" |
| **`related-products.md`** | External prior art / competitor research, self-contained. | Internal decisions | Rarely needs to reference the others |
| **`CLAUDE.md`** | **Operational guidance for a Claude Code session** — task-tracking workflow, decision-making heuristic, and an index pointing into every doc below (including this one). Not a content doc itself. | Decision content or reasoning | Points to all of the above |
| **`manifest.md`** (this file) | The ownership table itself — what each doc is for. | Decision content | — |
| **`LESSONS.md`** | General engineering guidelines/rules — reusable across future projects, not specific to Legacy's own decisions. | Legacy-specific decision reasoning (that's `IDEATION.md`'s job) | `IDEATION.md` when a guideline here actually shaped a Legacy decision |

## Filenames at a glance

`IDEATION.md` · `ARCHITECTURE.md` · `research-log.md` · `NORTH-STAR.md` · `related-products.md` · `manifest.md` · `CLAUDE.md` · `LESSONS.md` · `decisions/`

# Legacy — Architecture

System-level component map, covering every feature in `NORTH-STAR.md` — built, deferred, and parked. For *why* any of this is shaped this way, see `IDEATION.md`; for locked decisions, see `CLAUDE.md`. This doc is the "what talks to what," not a new source of reasoning — keep it in sync with `IDEATION.md` rather than drifting from it.

## The shape that matters

Everything downstream of the approved snapshot is a pure reader. The renderer, the GitHub badge action, the Raycast extension, and the registry/aggregator never talk to each other or to a server — each just fetches a file over plain HTTP. The detector is the only component with real privileges (filesystem/shell access), and the marketplace is the only component that would need any (accounts/DB) — which is exactly why it's a separate, later build rather than a feature bolted onto the rest.

```mermaid
flowchart TB
    subgraph machine["ON THE USER'S OWN MACHINE"]
        direction TB
        scanners["Source scanners<br/>brew · cargo · npm · Info.plist<br/><i>(deferred: browser ext, hardware)</i>"]
        usage["Usage readers<br/>shell history · agent transcripts"]
        classifier["Privacy / allowlist classifier<br/>public-registry → visible<br/>.local / custom-tap → hidden"]
        store[("WorkingStore interface<br/>adapter: SQLite (v1)<br/>private, full history, never published")]
        export["getApprovedSnapshot()<br/>(allowlist-visible items only)"]
        review{{"Manual review gate<br/>(human)"}}
        writerjson["legacy.json writer"]
        writerdb["legacy.db writer<br/>(optional companion)"]
        tamper["Tamper-resistance<br/>hash + submit"]

        scanners --> classifier
        usage --> classifier
        classifier -->|recordScan| store
        store -->|on publish| export
        export --> review
        review -->|approved| writerjson
        review -->|approved| writerdb
        writerjson --> tamper
    end

    legacyjson[("legacy.json<br/>(required, canonical —<br/>versioned, stable schema)")]
    legacydb[("legacy.db<br/>(optional companion —<br/>filtered SQLite export)")]
    tamper -->|anchors hash of| legacyjson
    writerjson -->|writes| legacyjson
    writerdb -->|writes| legacydb

    ots["OpenTimestamps<br/>(external, free, zero infra)"]
    tamper -.->|submits hash to| ots

    legacyjson --> renderer
    legacyjson --> badge
    legacyjson --> raycast
    legacyjson --> registry
    legacydb -.->|event-level detail,<br/>when needed| registry

    subgraph renderer["Template renderer"]
        direction TB
        r1["journey / retrospective view"]
        r2["<i>deferred:</i> simulated desktop UI"]
        r3["command-palette search overlay"]
        r4["<i>parked:</i> hireable / credibility view"]
        r5["<i>idea:</i> explore-with-SQL view (legacy.db, sql.js)"]
    end

    subgraph badge["GitHub README badge"]
        b1["scheduled Action → SVG/markdown snippet"]
    end

    subgraph raycast["Raycast snippet extension"]
        ra1["hotkey → top-5-tools text + link"]
    end

    subgraph registry["Registry / aggregator (v-later)"]
        direction TB
        reg0["person's own Action, on data change<br/>(opt-in) → PR via fork-scoped PAT"]
        reg1["registry repo: data/&lt;username&gt;.json<br/>+ registry list"]
        reg2["CI: schema-validate → auto-merge<br/>(anomalies hold for review)"]
        reg2b["scheduled pull sweep<br/>(fallback for non-push registrants)"]
        reg3["aggregation on merge"]
        reg4["static leaderboard page"]
        reg0 --> reg2
        reg2b --> reg1
        reg2 --> reg1 --> reg3 --> reg4
    end

    marketplace["Marketplace (Problem 3 endgame)<br/><i>not designed, correctly parked</i><br/>needs centralized accounts/DB —<br/>the one piece that breaks the<br/>no-server model everywhere else"]

    legacyjson -.->|evidence layer, later| marketplace

    style store fill:#4a5568,color:#fff
    style legacyjson fill:#2c5282,color:#fff
    style legacydb fill:#2c5282,color:#fff
    style marketplace stroke-dasharray: 5 5
    style registry stroke-dasharray: 5 5
    style r2 color:#888
    style r4 color:#888
    style r5 color:#888
```

## Components

Structure only — for *why* each piece is shaped this way, see `IDEATION.md`'s "Product decisions locked so far" and "Local storage engine options." Not repeated here.

**Detector** (local, the only privileged component) — source scanners (pluggable per ecosystem) → privacy/allowlist classifier → `WorkingStore` interface (`recordScan`, `getToolHistory`, `diffSince`, `recordReviewDecision`, `getApprovedSnapshot`, `exportRaw`/`importRaw`; `SqliteWorkingStore` is the only v1 adapter) → `getApprovedSnapshot()` → manual review gate (human) → two writers reading the same approved output → tamper-resistance (hashes + OpenTimestamps).

**`legacy.json`** (required) — the contract every other piece can assume exists. **`legacy.db`** (optional companion) — filtered SQLite export of the same snapshot, for direct querying.

**Template renderer** — cloned per person, no server, reads the owner's `legacy.json`. Journey/retrospective view is default; simulated desktop, search overlay, and an SQL-explore view (if `legacy.db` present) layer on top later; hireable/credibility view parked.

**Distribution add-ons** — GitHub README badge Action, Raycast snippet extension. Both read-only against `legacy.json`, no new detection logic.

**Registry/aggregator** (v-later, own repo) — push: each person's opt-in Action PRs `data/<username>.json` to the registry on change, auto-merged after CI schema validation; pull: scheduled fallback sweep for anyone not on push; aggregation runs on merge, recomputing the leaderboard from the registry's own `data/`.

**Marketplace** (Problem 3's endgame) — not designed. The one component needing centralized accounts/a real database.

## Open questions this doesn't answer

Canonical list: `IDEATION.md`'s "Open, not yet decided." This diagram reflects the shape of the system, not the resolution of those.

# Legacy — North Star

One page, read this when drifting. Extracted from `IDEATION.md` and `research-log.md` — this is the distilled final state, not the reasoning trail. Read those two for the "why."

## The pitch

Detected tool/system history, presented as **the story of your journey** — "you used Docker for 3 years before switching to Podman" — instead of a live dashboard of what's currently installed. That distinction is why the name is Legacy, not a cosmetic choice.

## The three pillars of value

Three real value props, sequenced deliberately — not three separate ideas, three layers on the same data.

**1. Inventory — solving tool amnesia (primary, build first).** No trustworthy record exists today of what you had, why, or when you moved on. This is the base layer everything else stands on.

**2. Sharing — an always-current "what's your stack" (secondary, same data).** A README badge / portfolio segment, no shared infra needed. Framed as *your journey*, not a static list — the same hook as Spotify Wrapped or GitHub Wrapped, which is what makes it worth sharing at all.

**3. Credibility — real usage as a hiring/expertise signal (parked, not abandoned).** The same evidence, later positioned as proof of real tool experience — analogous to Git AI for code attribution. Sequenced last on purpose: needs pillar 1's adoption first, and there's an honest open doubt about whether usage frequency still proves skill in the AI-assisted-coding era.

## Features

**Detection**
- CLI tools — Homebrew Cellar `INSTALL_RECEIPT.json` (filtered to `installed_on_request`), `brew info --json=v2 --installed` for casks, `cargo install --list`, npm global `bin` fields
- Desktop apps — `/Applications` + `~/Applications`, `Info.plist`
- *Deferred:* browser extensions (per-extension `manifest.json` in browser profile), hardware (`system_profiler`)
- *Idea:* remote/cloud usage capture via tmux/iTerm2 hotkey, tagged as remote-observed

**Output**
- `legacy.json` — stable, versioned; the shared data layer every other feature reads from
- Local SQLite store (private, never published) is the detector's actual working state — full scan history, diffs, manual review decisions; `legacy.json` is a filtered, human-reviewed export of it, not the same thing

**Presentation**
- Journey/retrospective rendering, not a raw table
- *Deferred:* simulated-desktop UI (once the detector is proven)
- *Idea:* command-palette search overlay, grouped by job/verb, for large collections

**Distribution / sharing**
- GitHub README "top 5 tools" embed
- *Idea:* Raycast snippet trigger — a keystroke expands to a shareable summary + link

**Deployment**
- Decentralized clone-and-point template — no server, no accounts, no database; growth by forking

**Privacy / safety**
- Allowlist-by-source publishing gate — public-registry tools visible by default, `.local`/custom-tap tools hidden by default
- Manual review required before anything publishes — detection is automatic, publishing never is

**Credibility / trust (parked)**
- Hireable/credibility profile view — usage evidence positioned as a hiring signal
- Tamper-resistance — hash the manifest, anchor via OpenTimestamps, so pillar 3 has a foundation whenever it's picked back up

**Cross-deployment (deferred, designed for)**
- Registry repo + scheduled GitHub Action + static leaderboard — manifest schema stays stable specifically so this stays easy later

## Who benefits (value by actor)

- **Author (you):** never has to remember or maintain the record manually; a personal, nostalgic artifact of your own career evolution — real even with zero viewers; an always-current portfolio badge with no upkeep; the same data can back a credibility claim later for free; full control via the manual-review gate, protecting against the exact accidental-exposure mistake that started this project.
- **Peers / viewers** (recruiters, collaborators, potential clients, fellow devs): a genuinely interesting story, not just a utility; a real fit signal, more trustworthy than a resume line; fast discovery via the search overlay in a large collection; a specific, timestamped instance to actually ask about in conversation.
- **Company, as a hiring employer:** a harder-to-fake signal than self-reported claims, and something to probe live — matches how real 2026 hiring already works.
- **Company, as the author's current employer:** not a beneficiary — a party whose interests are actively protected by the privacy allowlist and mandatory review gate, without needing their permission or participation.
- **Future self:** the nostalgia value stands alone — looking back at your own "eras" is the point, not a side effect of being seen.
- **Other developers who clone the template:** growth is by forking, not sign-up — each adopter gets the same author-value, and an improving shared template benefits everyone using it.
- **A future leaderboard viewer** (deferred feature): a macro view — "what's trending across everyone" — different from any single profile.
- **Tool/library maintainers:** aggregate, anonymized adoption data richer than download counts — shows *sustained* usage over time, a signal no current metric captures well.

## Not building right now

If you catch yourself starting one of the deferred/idea items above as if it were core scope, stop — it means you've drifted, not that the pillar lost value. The credibility feature itself is the biggest one to watch: parked, not cut.

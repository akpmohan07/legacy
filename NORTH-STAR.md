# Legacy — North Star

One page, read this when drifting. Extracted from `IDEATION.md` and `research-log.md` — this is the distilled final state, not the reasoning trail. Read those two for the "why."

## The pitch

Detected tool/system history, presented as **the story of your journey** — "you used Docker for 3 years before switching to Podman" — instead of a live dashboard of what's currently installed. That distinction is why the name is Legacy, not a cosmetic choice.

## The three pillars of value

Three real value props came out of the research, sequenced deliberately — not three separate ideas, three layers on the same data.

**1. Inventory — solving tool amnesia (primary, build first).** Your environment accumulates invisibly over years: CLI tools, apps, whole systems you upgraded through. There's no trustworthy record of what you had, why, or when you moved on. This is the base layer everything else stands on.

**2. Sharing — an always-current "what's your stack" (secondary, same data).** A README badge / portfolio segment generated from your own manifest, no shared infra needed. Real itch (people already hand-maintain tech-stack cards today), weaker engagement evidence on its own — but framed as *your journey*, not a static list, it's structurally the same hook as Spotify Wrapped or GitHub Wrapped, which is what makes it worth sharing at all.

**3. Credibility — real usage as a hiring/expertise signal (parked, not abandoned).** The same evidence, positioned later as proof of real tool experience — analogous to what Git AI does for AI-code attribution. Deliberately sequenced last: it needs pillar 1 to have real adoption first (no populated profiles, no reason for an employer to look), the privacy/allowlist model and OpenTimestamps tamper-resistance are already designed for it, and there's an honest open doubt about whether usage frequency still proves skill in the AI-assisted-coding era. Not cut — parked until the ground under it stops shifting.

## Features (with the technical detail behind each)

**1. CLI tool detection** — no manual entry, all read from existing package-manager state:
- Homebrew formulae: Cellar `INSTALL_RECEIPT.json`, filtered to `installed_on_request` (excludes pulled-in dependencies)
- Homebrew casks: `brew info --json=v2 --installed`
- Rust tools: `cargo install --list`
- npm globals: read `bin` fields from installed global packages

**2. Desktop app detection** — scan `/Applications` + `~/Applications`, read each `.app`'s `Info.plist` for name/version/bundle ID.

**3. `manifest.json` output** — a single, versioned file that's the detector's entire output, stable from day one so a future aggregator can parse any historical version. This file is the seam between detector and renderer, and the shared data layer all three pillars read from.

**4. Privacy gate before anything is public** — allowlist-by-source, not a denylist: tools from Homebrew core / public npm / crates.io default **visible**; tools from `.local`/`.path`/a custom Homebrew tap default **hidden** (that's where internal/proprietary tools live). Detection is automatic; publishing never is — always a manual review step.

**5. Rendering** — reads the manifest and presents it as the journey narrative (see The pitch above), not a raw table. Serves pillars 1 and 2 directly.

**6. Deployment: decentralized, clone-and-point** — no server, no accounts, no database. A template site (this repo) is cloned per person and pointed at their own `manifest.json` URL (their own repo/gist). Growth by forking, not sign-up.

**7. Tamper-resistance (designed for, not built yet)** — hash the manifest, anchor the hash via OpenTimestamps (free, no infra of your own). Exists so pillar 3 has a foundation whenever it's picked back up: proves data hasn't been altered since a specific point, without needing to build or trust a central authority.

## Not building right now

If you catch yourself building one of these, stop — it means you've drifted from what's actually locked in, not that the underlying pillar lost value:
- Browser extension or hardware detection
- Simulated-desktop UI (the render stays plain until the detector itself is proven)
- Cross-deployment leaderboard/aggregator (designed for — manifest schema stays stable so this stays easy later)
- The credibility feature itself (see pillar 3 — parked, not cut)

## Why it matters (value props)

- **For you, the author:** an accurate record you never had to remember or maintain yourself, that can also back up a credibility claim later without extra work now.
- **For a viewer:** a human, nostalgic story about someone's dev journey — more interesting than a static tech-stack list, which is what makes it worth sharing at all.
- **For an employer, eventually:** a specific, timestamped, hard-to-fake instance to ask about in an interview — feeding a real conversation, not replacing one.

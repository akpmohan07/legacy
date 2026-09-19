# Legacy — North Star

One page, read this when drifting. Extracted from `IDEATION.md` and `research-log.md` — this is the distilled final state, not the reasoning trail. Read those two for the "why."

## The pitch

A nostalgic, always-current record of the tools and systems you've used across your career — not a live inventory dashboard, a "Wrapped"-style retrospective of your journey.

## The problem (why this exists)

**Tool amnesia.** Your environment accumulates invisibly over years — CLI tools, apps, whole systems you've upgraded through — and there's no trustworthy record of what you had, why, or when you moved on.

## The core idea

Present detected tool/system history as **a story of your journey**, not a current-state snapshot. "You used Docker for 3 years before switching to Podman" beats "here are your installed tools." This is *why* the name is Legacy, not a cosmetic choice.

## Features (with the technical detail behind each)

**1. CLI tool detection** — no manual entry, all read from existing package-manager state:
- Homebrew formulae: Cellar `INSTALL_RECEIPT.json`, filtered to `installed_on_request` (excludes pulled-in dependencies)
- Homebrew casks: `brew info --json=v2 --installed`
- Rust tools: `cargo install --list`
- npm globals: read `bin` fields from installed global packages

**2. Desktop app detection** — scan `/Applications` + `~/Applications`, read each `.app`'s `Info.plist` for name/version/bundle ID.

**3. `manifest.json` output** — a single, versioned file that's the detector's entire output, stable from day one so a future aggregator can parse any historical version. This file is the seam between detector and renderer — everything downstream depends on its shape being stable.

**4. Privacy gate before anything is public** — allowlist-by-source, not a denylist: tools from Homebrew core / public npm / crates.io default **visible**; tools from `.local`/`.path`/a custom Homebrew tap default **hidden** (that's where internal/proprietary tools live). Detection is automatic; publishing never is — always a manual review step.

**5. Rendering** — reads the manifest and presents it as the "journey" narrative (see Core idea above), not a raw table.

**6. Deployment: decentralized, clone-and-point** — no server, no accounts, no database. A template site (this repo) is cloned per person and pointed at their own `manifest.json` URL (their own repo/gist). Growth by forking, not sign-up.

## Decided against, for now

If you catch yourself building one of these, stop — it means you've drifted from what's actually locked in:
- Browser extension or hardware detection
- Simulated-desktop UI (the render stays plain until the detector itself is proven)
- Cross-deployment leaderboard/aggregator
- Credibility/hiring-signal feature — parked until the base detector and showcase have real adoption

## Why it matters (value props)

- **For you, the author:** an accurate record you never had to remember or maintain yourself.
- **For a viewer:** a human, nostalgic story about someone's dev journey — more interesting than a static tech-stack list, which is what makes it worth sharing at all.

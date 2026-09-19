# Legacy — North Star

One page, read this when drifting. Full reasoning lives in `IDEATION.md`; this is just the compass.

## The pitch

A nostalgic, always-current record of the tools and systems you've used across your career — not a live inventory dashboard, a "Wrapped"-style retrospective of your journey.

## The problem (why this exists)

**Tool amnesia.** Your environment accumulates invisibly over years — CLI tools, apps, whole systems you've upgraded through — and there's no trustworthy record of what you had, why, or when you moved on.

## The core idea

Present detected tool/system history as **a story of your journey**, not a current-state snapshot. "You used Docker for 3 years before switching to Podman" beats "here are your installed tools." This is *why* the name is Legacy, not a cosmetic choice.

## v1 — feature list (with the technical detail behind each)

**1. CLI tool detection** — no manual entry, all read from existing package-manager state:
- Homebrew formulae: Cellar `INSTALL_RECEIPT.json`, filtered to `installed_on_request` (excludes pulled-in dependencies)
- Homebrew casks: `brew info --json=v2 --installed`
- Rust tools: `cargo install --list`
- npm globals: read `bin` fields from installed global packages

**2. Desktop app detection** — scan `/Applications` + `~/Applications`, read each `.app`'s `Info.plist` for name/version/bundle ID.

**3. `manifest.json` output** — a single, versioned file that's the detector's entire output. Versioned from v1 so a future aggregator can parse any historical version. This file is the seam between detector and renderer — everything downstream depends on its shape being stable.

**4. Privacy gate before anything is public** — allowlist-by-source, not a denylist: tools from Homebrew core / public npm / crates.io default **visible**; tools from `.local`/`.path`/a custom Homebrew tap default **hidden** (that's where internal/proprietary tools live). Detection is automatic; publishing never is — always a manual review step.

**5. Rendering** — reads the manifest and presents it as the "journey" narrative (see Core idea above), not a raw table. Plain page is fine for v1; simulated-desktop UI is v1.1+.

**6. Deployment: decentralized, clone-and-point** — no server, no accounts, no database. A template site (this repo) is cloned per person and pointed at their own `manifest.json` URL (their own repo/gist). Growth by forking, not sign-up.

## v1 — explicitly NOT doing yet

If you're building one of these, stop and check `TASKS`/the project board first — it means you've drifted from v1 scope:
- Browser extension or hardware detection (v1.1)
- Simulated-desktop UI (v1.1+)
- Cross-deployment leaderboard/aggregator
- Credibility/hiring-signal feature — fully parked pending real adoption of the above

## Why it matters (value props)

- **For you, the author:** an accurate record you never had to remember or maintain yourself.
- **For a viewer:** a human, nostalgic story about someone's dev journey — more interesting than a static tech-stack list, which is what makes it worth sharing at all.

# Legacy — North Star

One page, read this when drifting. Full reasoning lives in `IDEATION.md`; this is just the compass.

## The pitch

A nostalgic, always-current record of the tools and systems you've used across your career — not a live inventory dashboard, a "Wrapped"-style retrospective of your journey.

## The problem (why this exists)

**Tool amnesia.** Your environment accumulates invisibly over years — CLI tools, apps, whole systems you've upgraded through — and there's no trustworthy record of what you had, why, or when you moved on.

## The core idea

Present detected tool/system history as **a story of your journey**, not a current-state snapshot. "You used Docker for 3 years before switching to Podman" beats "here are your installed tools." This is *why* the name is Legacy, not a cosmetic choice.

## v1 — what it actually does

- Auto-detects CLI tools (Homebrew, cargo, npm globals) and desktop apps. Nothing manually typed.
- Emits a stable, versioned `manifest.json`.
- Renders as a shareable page — plain is fine for v1, simulated-desktop polish comes later.
- Deploys decentralized: clone the template, point it at your own manifest. No server, no accounts, no database.

## v1 — explicitly NOT doing yet

If you're building one of these, stop and check `TASKS`/the project board first — it means you've drifted from v1 scope:
- Browser extension or hardware detection (v1.1)
- Simulated-desktop UI (v1.1+)
- Cross-deployment leaderboard/aggregator
- Credibility/hiring-signal feature — fully parked pending real adoption of the above

## Why it matters (value props)

- **For you, the author:** an accurate record you never had to remember or maintain yourself.
- **For a viewer:** a human, nostalgic story about someone's dev journey — more interesting than a static tech-stack list, which is what makes it worth sharing at all.

## Hard rules (never optimize away)

- Detection is fully automatic and local. **Publishing is never automatic** — always a human review step first.
- Privacy default is allowlist-by-source: public-registry-sourced tools visible by default, `.local`/custom-tap-sourced tools hidden by default.

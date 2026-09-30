# Related products & prior art

Every existing tool, product, or project encountered while researching Legacy (formerly "Loadout"), organized by why it came up. Most are adjacent, not competitors — but a few personal multi-source inventory tools (`Installory`, `inventory`, `cli-tools`) and several developer-machine security products do overlap directly; see the notes on each for how it relates and where the gap still is.

**Last updated 2026-09-24** (added the direct competitors, developer-first enterprise products, and a traction snapshot). Facts below come from each project's own README/docs or GitHub API, read that day; where only a search-result snippet was seen, the row says so. Star counts are a crude proxy for adoption, not usage.

## At a glance

| Tool | Summary | How it relates | Our value / the gap |
|---|---|---|---|
| [cli-tools](https://github.com/flaviocopes/cli-tools) | macOS app + CLI; detects Homebrew/npm/Cargo tools, reads shell + AI-agent history (35 stars, created 2026-09-09) | Direct architecture reference for detection design | Standalone build, full-spectrum (apps/extensions/hardware too, not CLI-only); adds the sharing layer it lacks |
| [StackShare](https://stackshare.io/) | Company tech-stack pages, auto-detect + public sharing | The accidental publish here is the project's origin story; proves auto-detect+share can work | Decentralized, so no company exists to decay; personal, not company-scoped |
| [awesome-uses](https://github.com/wesbos/awesome-uses) / uses.tech | Manually curated "here's my setup" page list, 9 years old (5,302 stars) | Proves the itch to declare your setup is real and durable | Auto-updates from real usage; uses.tech is one-time manual entry, never revisited |
| [profile_stack](https://github.com/gleich/profile_stack) | GitHub Action rendering a README tech-stack table from a config file | Same "badge in your README" idea | Badge is auto-detected from a real machine, not manually typed config |
| [github-readme-tech-stack](https://github.com/0l1v3rr/github-readme-tech-stack) | Similar README tech-stack cards | Same badge category | Same gap — manual config, not detected |
| [Installory](https://github.com/william-ricchiuti/Installory) | macOS app inventorying Homebrew, pip, pipx, npm, Cargo, RubyGems, App Store and AI tools; snapshots, baseline compare, install timing; local, exports CSV/Markdown/JSON | **Direct competitor** for personal multi-source inventory + change detection | Manual scans only (no automatic watching mentioned); no timeline-retrospective or sharing layer seen. 0 stars |
| [inventory](https://github.com/lightisbeauty/inventory) (lightisbeauty) | macOS scanner: `/Applications`, Homebrew, App Store, pip, npm, gems, cargo, conda, Nix, launch agents; save snapshots and diff two | **Direct competitor**, wider source coverage than Legacy today | Manual snapshots; no install timestamps, no automatic watching; PDF/HTML report only. 3 stars |
| [Helm](https://github.com/jasoncavinder/Helm) | Menu-bar app unifying 15+ package managers into one control plane (snippet only; not read in full) | Adjacent: package management across ecosystems | Answers "update/manage", not "what changed and when" |
| Enterprise IT inventory (ManageEngine, Spiceworks, Total Network Inventory, InvGate, Lansweeper, NinjaOne, Action1) | Automated software/hardware inventory across a company fleet; Lansweeper keeps history as add/remove actions (an update is a removal plus an addition) | Same detection categories, and a paid, proven market | Audience is IT admins auditing others; not personal or shareable-as-identity |
| [Kolide](https://www.kolide.com/features/device-inventory/properties/mac-package-install-history) / [Fleet](https://github.com/fleetdm/fleet) / Jamf / Iru (Kandji) / Munki | Fleet device inventory with software history. Kolide's Mac install history stores `installed_at` per record, sources `appstoreagent`, `softwareupdated`, `installer` (Homebrew/npm/cargo not listed for it; separate Homebrew and npm inventories exist). Fleet: MIT core, osquery, GitOps | Same install-history idea, at fleet scale | Central admin consoles for companies. Jamf's history is per computer and not exportable org-wide |
| [osquery](https://osquery.readthedocs.io/en/stable/deployment/logging/) | Differential logging: each scheduled query logs `added`/`removed` rows versus the last result; `file_events` via FSEvents | Closest mechanism to Legacy's change events | Built for security teams. Its first run reports every row as `added`; Legacy records the first scan as a baseline with no events |
| [StepSecurity Dev Machine Guard](https://docs.stepsecurity.io/dev-machine-guard) | Apache 2.0 scanner of dev machines: AI agents, MCP servers, IDE extensions, npm/Python/Homebrew packages; macOS/Windows/Linux; runs locally, JSON/HTML output | **Developer-first with an enterprise tier** (see below) | Security purpose. Enterprise-only: dashboard, policy, scheduled scans, historical trends. 177 stars |
| [SafeDep](https://safedep.io/endpoint-protection/) | Open-source local CLIs (PMG intercepts package installs, VET discovers agents/MCP/extensions) plus a cloud Endpoint Hub with inventory snapshots and install-event timelines | Developer-first with a hosted console | Supply-chain security focus; timeline lives in their hub |
| [Aikido Device Protection](https://www.aikido.dev/protect/device-protection) | Inventory of packages (npm, PyPI, Cargo/Rust, Homebrew, ...), IDE and browser extensions, AI tools; continuous monitoring; macOS/Windows/Linux; free tier | Developer-machine visibility, commercial | Security purpose; team dashboard, not personal |
| [Workbrew](https://workbrew.com/homebrew) | Fleet management layer on Homebrew (it sponsors Homebrew): inventory via MDM, package policy, audit trail; Workbrew Free tier | Homebrew-only fleet inventory | Company governance; whether it keeps install history over time is not stated |
| Fake-desktop portfolio genre ([portfolio-os](https://github.com/DareDev256/portfolio-os), [my-portfolio](https://github.com/Justinianus2001/my-portfolio), [writeup](https://dev.to/dustinbrett/how-i-made-a-desktop-environment-in-the-browser-15oi), [HN post](https://news.ycombinator.com/item?id=27084995)) | Personal portfolio sites styled as a fake OS desktop, draggable windows | The exact UI concept planned for the shareable page | All hand-built with static content — de-risks the rendering technique; wiring it to real detected data is still the open gap |
| [stashapp/stash](https://github.com/stashapp/stash) | Unrelated, well-known self-hosted adult-content media organizer | Surfaced as a naming collision for "Stash" | Ruled the name out; no functional relation |
| Memento (various — [example](https://github.com/machawk1/awesome-memento)) | HTTP-caching tool / content aggregator / web-archive CLI | Surfaced as a naming collision for "Memento" | Lesser collision than Stash but still occupied; name demoted |
| [WakaTime](https://definable.ai/apps/wakatime/) | Automatic editor-activity tracking, public leaderboard filterable by "hireable" status | Closest real analogue to the credibility-signal idea | Validates the mechanism (automatic evidence → public profile → hiring signal); different data — editor time, not tool/app/hardware usage |
| Synthetic skill-assessment platforms (HackerRank, [CodeSignal](https://codesignal.com/skills-validation/), Testlify) | Test/challenge-based hiring verification | The dominant existing hiring-verification paradigm | Different paradigm entirely (tests vs. real usage evidence) — a less crowded lane |
| [Git AI](https://usegitai.com/) | Line-level AI-code attribution via Git Notes, for engineering teams | The direct analogy that sparked the credibility-signal idea | Applied the same "evidence, not claims" logic to tool usage instead of code authorship |
| [OpenTimestamps](https://opentimestamps.org/) | Free hash-timestamping anchored into Bitcoin, no signup required | Identified as the tamper-resistance mechanism | Adopted directly as the plan — zero infrastructure of our own needed |
| [tmux-logging](https://github.com/tmux-plugins/tmux-logging) | tmux plugin for continuous pane-to-file logging | Confirmed `capture-pane` as a real, hotkey-bindable primitive | Building block for remote-usage capture; no product assembles this for personal tracking yet |
| [SSHLog](https://github.com/sshlog/agent) | eBPF daemon monitoring an SSH server's session activity | Same "capture remote command usage" goal | Requires remote install, wrong audience (sysadmin auditing others) — our approach needs zero remote install |
| auditd | Linux kernel-level audit logging | Same audit-logging space | Same — server-side compliance tool, not personal |
| [MintMCP](https://www.mintmcp.com/blog/claude-code-monitoring) | Proxy-based monitor of AI-agent commands/MCP calls across Claude Code/Cursor/Copilot | Same "capture agent-run commands" goal, more robust than passive transcript parsing | Enterprise compliance tool, employer-owned data — this is the ownership fork that led to staying personal/self-sovereign |
| [Nylas CLI](https://cli.nylas.com/guides/audit-ai-agent-activity) | Audit logging for AI agent actions, SIEM-exportable | Same category as MintMCP | Same — enterprise/employer-owned, not personal |
| Bifrost | Per-MCP-call audit records (tool, server, args, result, latency) | Same category as MintMCP/Nylas | Same — enterprise, not personal |
| Homebrew GUI managers ([Cork](https://github.com/buresdv/Cork), [BrewStore](https://brewstore.app/), [Brew Browser](https://brew-browser.zerologic.com/), [Cellar](https://github.com/tuhage/Cellar), [ForgedBrew](https://github.com/HighfieldLondon/ForgedBrew), [Applite](https://github.com/milanvarady/Applite)) | GUIs for browsing/installing/upgrading/removing Homebrew formulae and casks | Adjacent to `cli-tools`' Homebrew detection, surfaced when researching alternatives to it | Answer "what can I install or update" (package management); `cli-tools`/Legacy answer "what do I have, where's it from, how do I use it, do I still use it" (cross-ecosystem inventory) — genuinely different question, not a competitor |
| Homebrew Bundle / `Brewfile` (built into Homebrew) | `brew bundle dump` exports installed formulae/casks/taps to a file | Closest built-in tool for recording a Homebrew setup | Reproducibility/backup tool, not a searchable catalog — no explanations, no usage history, no cross-ecosystem view |
| [mise](https://github.com/jdx/mise) | Manages developer tool *versions*, env vars, and tasks per-project | Adjacent "what tools does my project need" tool | Solves reproducible project environments, not a personal inventory of everything installed on the machine |
| [GitHub Wrapped](https://github.com/mtwn105/GitHubWrapped) / [Git Wrapped](https://git-wrapped.com/) | Spotify-Wrapped-style yearly recap of git commit activity (languages, top repos, commit trends) | Same nostalgic, shareable retrospective *format* the reframing is built on | Analyzes git commit metadata, not tool/environment inventory — different data entirely |
| [Gource](https://gource.io/) / [Codebase Timeline Visualizer](https://github.com/Adrijan-Petek/codebase-timeline-visualizer) / [Repo Visualizer](https://github.com/AbanteAI/repo-visualizer) | Animate a single repository's code/file structure evolving over time | Same "watch history unfold" visual idea | Scoped to one repo's code, not a person's tool inventory across multiple machines and years |

## Direct architectural inspiration

- **[flaviocopes/cli-tools](https://github.com/flaviocopes/cli-tools)** — MIT, macOS app + `clitools` CLI. Detects CLI tools via Homebrew/npm/Cargo, reads shell history (zsh/bash/fish) and AI-agent transcripts (Cursor/Codex/Claude Code) for usage. The direct architecture reference for detection design (`ToolDiscovery`, `CatalogRepository`, `ShellHistory`, `AgentHistory`). Solves tool amnesia for CLI tools only, privately (stays on one Mac), no sharing layer. Legacy is standalone, not a fork — full-spectrum (CLI + apps + extensions + hardware), not CLI-only.

## Sharing / "uses" page precedents

- **[StackShare](https://stackshare.io/)** — company tech-stack pages, auto-detect + public sharing for companies. Decayed as a business. The `npm install -g stackshare` CLI run that accidentally published a public page for the `panther` repo is literally what started this whole project.
- **[awesome-uses](https://github.com/wesbos/awesome-uses)** / uses.tech — manually curated list of personal "here's my setup" pages. 9 years old, still gets PRs in 2026, but never evolved past a one-time static submission. Proves the *itch* to declare your setup is real and durable; proves manual upkeep never sticks.

## GitHub README tech-stack tools (partial overlap with the badge idea)

- **[profile_stack](https://github.com/gleich/profile_stack)** — GitHub Action rendering a tech-stack table in your README from a manually-maintained config file.
- **[github-readme-tech-stack](https://github.com/0l1v3rr/github-readme-tech-stack)** — similar tech-stack cards for a README.
- Both solve a sliver of the badge idea, but from manually-typed config, never auto-detected from a real machine — the gap (auto-detected, not manually curated) still holds.

## Personal multi-source inventory (direct competitors; added 2026-09-24)

- **[Installory](https://github.com/william-ricchiuti/Installory)** — MIT, macOS app, v1.5.0, 107 commits, 0 stars, created 2026-05-15. Read-only inventory of Homebrew (formulae and casks), pip, pipx, uv tools, npm, Cargo, RubyGems, receipt-bearing App Store apps and AI agent stack. README lists "snapshots, baseline compare with change detection and reinstall scripts", records install timing, exports CSV/Markdown/JSON, "no network, no tracking". Optional provenance from shell history and Claude Code session records, off by default. No automatic watching mentioned.
- **[inventory](https://github.com/lightisbeauty/inventory)** (lightisbeauty) — GPL-3.0, macOS 12+, Python 3, 3 stars, created 2026-06-24. Covers `/Applications` and `~/Applications`, Homebrew, App Store, pip, npm, gems, cargo, conda/mamba, MacPorts, Fink, Nix, launch agents and system info. Saves snapshots into a library and compares two (added/removed/updated). Manual; no install timestamps; PDF/HTML export; local only.
- **[cli-tools](https://github.com/flaviocopes/cli-tools)** — see "Direct architectural inspiration" below.
- Neither of the first two watches automatically, keeps a continuous timeline, or offers a shareable page. Not verified: whether either is planning to.

## Enterprise IT/hardware inventory (different audience and buyer)

- **ManageEngine Endpoint Central**, **Spiceworks Inventory**, **Total Network Inventory**, **InvGate**, **NinjaOne**, **Action1** — automated software/hardware inventory across a fleet, for IT admins doing license/compliance audits.
- **Lansweeper** — history tracking for scan items; each history row has an Action (added / removed), so an updated program appears as a removal of the old version plus an addition of the new (community/docs snippets; not tested).
- **[Kolide](https://www.kolide.com/features/device-inventory/properties/mac-package-install-history)** — device inventory including browser extensions and Mac package install history (stored, with `installed_at`; sources `appstoreagent`, `softwareupdated`, `installer`). Separate inventories exist for Homebrew and npm packages; whether those keep history was not checked.
- **[Fleet](https://github.com/fleetdm/fleet)** — MIT-core open-source device management on osquery, with GitOps, REST API and `fleetctl`; `homebrew_packages`, `npm_packages`, `python_packages` tables. **Jamf** shows per-computer software history between inventory reports, not exportable org-wide (community post). **MunkiReport**'s `installhistory` module is described only as "Apple and 3rd party install history"; the docs do not say how it collects. **Iru (Kandji)** is Apple-only device management.
- All auto-detect similar categories to Legacy but for a company auditing *other people's* machines, never personal, never shareable-as-identity. This is a paid, proven market, unlike the personal one.

## Developer-first with an enterprise tier (open-core; added 2026-09-24)

- **[StepSecurity Dev Machine Guard](https://docs.stepsecurity.io/dev-machine-guard)** — Apache 2.0, 177 stars, created 2026-03-10. Fully local scan (terminal, `--json`, `--html`) of AI agents, MCP servers, IDE extensions, optional Node packages; macOS/Windows/Linux. "There is no separate closed-source version." Enterprise-only (activated with credentials): central dashboard, policy enforcement and alerting, scheduled automated scans, **historical trends and reporting**.
- **[SafeDep](https://safedep.io/endpoint-protection/)** — open-source local CLIs (PMG: package-install interception; VET: coding agents, MCP servers, skills, IDE extensions) plus Endpoint Hub (cloud) with inventory snapshots and install-event timelines per machine.
- **[Aikido Device Protection](https://www.aikido.dev/protect/device-protection)** — packages (npm, PyPI, Maven, NuGet, Go, Ruby, Rust, PHP, Homebrew), IDE and browser extensions, AI tools; "continuous monitoring"; macOS/Windows/Linux (WSL planned Q4 2026); free tier, paid pricing not shown.
- **[Workbrew](https://workbrew.com/homebrew)** — sponsors Homebrew; fleet Homebrew inventory through MDM (Intune, Jamf, Mosyle, Iru), package policy, audit trail; Workbrew Free tier. Install history over time not stated.
- Pricing, customers, and real-world quality of these were not checked.
- Adjacent, checked only at the description level: **Backstage** (software catalog of services, not machines), **Coder** (governed workspaces), **Devbox**/**Nix** (reproducible environments; Nix generations diffable with `nvd`), **mise** (tool versions). None inventories the tools on a developer's own machine.

## Traction snapshot (GitHub API, 2026-09-24)

| Repo | Stars | Created | Last push |
|---|---|---|---|
| jdx/mise | 34,236 | 2023-01-09 | 2026-09-24 |
| fleetdm/fleet | 6,905 | 2020-11-03 | 2026-09-24 |
| wesbos/awesome-uses | 5,302 | 2017-06-12 | 2026-09-20 |
| buresdv/Cork | 4,704 | 2022-07-03 | 2026-09-20 |
| step-security/dev-machine-guard | 177 | 2026-03-10 | 2026-09-23 |
| mtwn105/YourYearInCode (GitHub Wrapped-style; the `GitHubWrapped` link redirects here) | 106 | 2024-12-14 | 2025-12-26 |
| gleich/profile_stack | 57 | 2020-07-04 | 2024-06-17 |
| flaviocopes/cli-tools | 35 | 2026-09-09 | 2026-09-09 |
| jasoncavinder/Helm | 4 | 2026-02-11 | 2026-09-24 |
| lightisbeauty/inventory | 3 | 2026-06-24 | 2026-09-04 |
| william-ricchiuti/Installory | 0 | 2026-05-15 | 2026-09-23 |

Reading these: stars are a weak proxy (young repos, people use tools without starring them, download counts not checked). `cli-tools` gained its stars in 15 days; whether that is due to its author's existing audience was not checked.

## Simulated-desktop / fake-OS portfolio UI (the rendering genre, not the data)

- **[portfolio-os](https://github.com/DareDev256/portfolio-os)** — "a desktop operating system in a browser tab," draggable windows, vanilla JS + Vite.
- **[my-portfolio](https://github.com/Justinianus2001/my-portfolio)** — interactive desktop OS portfolio with window management, procedural music.
- **[dustinbrett's "How I made a desktop environment in the browser"](https://dev.to/dustinbrett/how-i-made-a-desktop-environment-in-the-browser-15oi)** — writeup on building the same pattern.
- **[HN Show HN: portfolio site simulating macOS's GUI in React](https://news.ycombinator.com/item?id=27084995)** — well-received example of the same genre.
- All prove the fake-desktop-UI technique is de-risked, well-documented, and appreciated — but every example is hand-built with static About/Projects content, never wired to real auto-detected data. That fusion is still the open gap.

## Name-collision findings (from the naming search)

- **[stashapp/stash](https://github.com/stashapp/stash)** — a well-known, unrelated adult-content media organizer. Ruled "Stash" out hard as a project name.
- **Memento** — multiple unrelated existing uses: an HTTP-caching dev tool, a personal content aggregator, a [Memento-protocol/web-archive CLI](https://github.com/machawk1/awesome-memento) (RFC 7089, Wayback-Machine-style). Real but lesser collisions than Stash.
- **"legacy" (npm)** — the exact package name is already taken by an old, low-traffic "Legacy browser style sheet generator," which even ships its own `legacy` CLI binary. The eventual detector package will need a different registry name.
- **"legacy" (GitHub convention)** — not a single collision but a strong existing naming convention: `Homebrew/legacy-homebrew`, `BoostNote-Legacy`, `tenacity-legacy`, and a bare `ErsatzTV/legacy` all use "legacy" to mean "deprecated, superseded by something else." Accepted knowingly rather than avoided — see `IDEATION.md` Naming section for the positioning mitigation.
- **"MyLegacy" / "My Legacy"** — heavily used by an unrelated cluster of digital-inheritance/estate-planning products: [my-legacy.ai](https://my-legacy.ai/) ("Secure Your Digital Legacy & Estate Plan"), `blockchainology/mylegacy` (inheritance/wills asset management), `amisatoshi/mylegacy` ("Islamic Estate Planner"). Ruled out this variant as a mitigation for the plain "Legacy" collision above.

## Credibility-signal prior art

- **[WakaTime](https://definable.ai/apps/wakatime/)** — the closest real analogue to the credibility-signal idea. Editor plugins passively track real coding time/language per project; public profiles include a browsable leaderboard filterable by language, country, and a "hireable" flag. Proves the mechanism (automatic evidence → public profile → hiring signal) is real and has years of adoption. Different data than Legacy (editor-time, not tool/app/hardware usage) and shares the same known limitation: only sees editor activity, misses everything else (meetings, reviews, remote work).
- **HackerRank, [CodeSignal](https://codesignal.com/skills-validation/), Testlify** — dominant hiring-verification paradigm: synthetic assessments/tests taken to prove skill, not evidence from real historical usage. A genuinely different, less crowded lane than what Legacy or WakaTime offer.

## The analogy that sparked the credibility-signal idea

- **[Git AI / usegitai.com](https://usegitai.com/)** — a Git extension giving engineering teams line-level, per-agent attribution of AI-generated code via `git ai checkpoint` + Git Notes, turning "how much AI code ships" from a claim into evidence. The direct inspiration for asking whether the same evidence-based-attribution approach could apply to tool usage instead of code authorship.

## Tamper-resistance building block

- **[OpenTimestamps](https://opentimestamps.org/)** ([client](https://github.com/opentimestamps/opentimestamps-client), [server source](https://github.com/opentimestamps/opentimestamps-server)) — free, open-source, no-signup protocol: hash data locally, submit the hash to free public calendar servers (`a.pool.opentimestamps.org`, `b.pool.opentimestamps.org`, `a.pool.eternitywall.com`, `alice.btc.calendar.opentimestamps.org`, `bob.btc.calendar.opentimestamps.org`), get a small proof anchored into Bitcoin. The identified mechanism for making manifest checkpoints tamper-evident, with zero infrastructure of your own required.

## Remote/agent-command capture & audit tooling

- **[tmux-logging](https://github.com/tmux-plugins/tmux-logging)** — tmux plugin for continuous pane-to-file logging. Confirmed `tmux capture-pane` itself as a standard, hotkey-bindable primitive — the building block for the "capture remote/SSH usage via local terminal pane" idea. No existing tool assembles this specifically for personal tool-usage tracking.
- **[SSHLog](https://github.com/sshlog/agent)** — eBPF daemon monitoring an OpenSSH *server* to record all connecting users' session activity. Requires installing on the remote machine; built for sysadmins auditing *other* users, not self-tracking.
- **auditd** — Linux kernel-level audit logging, same server-side/compliance audience as SSHLog.
- **[MintMCP](https://www.mintmcp.com/blog/claude-code-monitoring)** — proxy-based monitor capturing prompts, file access, commands, and MCP tool calls across Claude Code, Cursor, Codex, and Copilot in one governance layer.
- **[Nylas CLI](https://cli.nylas.com/guides/audit-ai-agent-activity)** — auto-detects agent sources (Claude Code, Copilot, MCP), logs every command with timestamps to `~/.config/nylas/audit.log`, SIEM-exportable.
- **Bifrost** — captures each MCP tool invocation as a dedicated audit record (tool, server, args, result, latency).
- All four AI-agent-audit tools are real, mature, and some already implement append-only hash-verified immutable logs — the same tamper-evidence idea as OpenTimestamps, already productionized. But all are enterprise security/compliance tools, auditing a company's own employees — none are built for an individual to own or export their own usage evidence. That gap, and the ownership tension it exposes (the data belongs to the employer, not the person), is what led to deciding Legacy should stay personal/self-sovereign rather than build on top of these.

## Homebrew package-management GUIs (adjacent to `cli-tools`, not to Legacy directly)

Surfaced from a user-supplied analysis researching alternatives to `flaviocopes/cli-tools` specifically.

- **[Cork](https://github.com/buresdv/Cork)** — the closest macOS GUI alternative to a Homebrew manager: formulae, casks, dependencies, services, tags, updates, cleanup.
- **[BrewStore](https://brewstore.app/)** — presents formulae and casks together; better for browsing/installing/upgrading/uninstalling than for documenting tools from npm, Cargo, or custom directories.
- **[Brew Browser](https://brew-browser.zerologic.com/)** — broader Homebrew manager with installed-package views, upgrades, dependency info, services, and vulnerability info; supports macOS and Linux.
- **[Cellar](https://github.com/tuhage/Cellar)** — native macOS GUI for Homebrew packages/casks/services, with a companion CLI.
- **[ForgedBrew](https://github.com/HighfieldLondon/ForgedBrew)** — polished Homebrew catalog: searchable formulae/casks, package details, screenshots, versions, dependencies, install/maintenance tools.
- **[Applite](https://github.com/milanvarady/Applite)** — simple open-source macOS GUI for Homebrew *casks* specifically (GUI apps, not CLI tools).
- **Homebrew Bundle / `Brewfile`** (built into Homebrew) — `brew bundle dump` exports installed formulae/casks/taps for reproducing a setup elsewhere. The closest built-in equivalent, but a backup/reproducibility file, not a searchable, explained catalog.
- **[mise](https://github.com/jdx/mise)** — manages developer tool *versions*, env vars, and tasks for reproducible project environments — a different question ("what does this project need") than a personal machine-wide inventory.

**The distinction that matters:** all of these answer *"what Homebrew packages can I install, update, or remove?"* — a package-management question. `cli-tools` (and Legacy) answer *"what command-line tools do I actually have installed, where did they come from, how do I use them, and do I still use them?"* — a cross-ecosystem inventory-and-understanding question (Homebrew + npm + Cargo + custom directories, usage history, `--help`/tldr explanations). Genuinely different jobs, which is why `cli-tools` reads as closer to a unique niche than a clone of any of these, even among six real, mature Homebrew-GUI alternatives.

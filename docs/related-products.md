# Related products & prior art

Every existing tool, product, or project encountered while researching Legacy (formerly "Loadout"), organized by why it came up. None of these are competitors in the direct sense — see the notes on each for how it relates and where the actual gap still is.

## At a glance

| Tool | Summary | How it relates | Our value / the gap |
|---|---|---|---|
| [cli-tools](https://github.com/flaviocopes/cli-tools) | macOS app + CLI; detects Homebrew/npm/Cargo tools, reads shell + AI-agent history | Direct architecture reference for detection design | Standalone build, full-spectrum (apps/extensions/hardware too, not CLI-only); adds the sharing layer it lacks |
| [StackShare](https://stackshare.io/) | Company tech-stack pages, auto-detect + public sharing | The accidental publish here is the project's origin story; proves auto-detect+share can work | Decentralized, so no company exists to decay; personal, not company-scoped |
| [awesome-uses](https://github.com/wesbos/awesome-uses) / uses.tech | Manually curated "here's my setup" page list, 9 years old | Proves the itch to declare your setup is real and durable | Auto-updates from real usage; uses.tech is one-time manual entry, never revisited |
| [profile_stack](https://github.com/gleich/profile_stack) | GitHub Action rendering a README tech-stack table from a config file | Same "badge in your README" idea | Badge is auto-detected from a real machine, not manually typed config |
| [github-readme-tech-stack](https://github.com/0l1v3rr/github-readme-tech-stack) | Similar README tech-stack cards | Same badge category | Same gap — manual config, not detected |
| Enterprise IT inventory (ManageEngine, Spiceworks, Total Network Inventory, InvGate) | Automated software/hardware inventory across a company fleet | Same detection categories (hardware + software) | Wrong audience (IT admins auditing others) and tone; not personal or shareable-as-identity |
| [Kolide](https://www.kolide.com/features/device-inventory/properties/device-chrome-extensions) | Device inventory including browser extensions, enterprise fleet | Same extension-detection idea | Same — built for enterprise fleets, not a person |
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

## Enterprise IT/hardware inventory (wrong audience, not a competitor)

- **ManageEngine Endpoint Central**, **Spiceworks Inventory**, **Total Network Inventory**, **InvGate** — automated software/hardware inventory across a fleet, for IT admins doing license/compliance audits.
- **[Kolide](https://www.kolide.com/features/device-inventory/properties/device-chrome-extensions)** — device inventory including browser extensions, same enterprise-fleet audience.
- All auto-detect similar categories to Legacy but for a company auditing *other people's* machines, never personal, never shareable-as-identity.

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

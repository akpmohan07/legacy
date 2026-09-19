# Legacy — Research Log

What should Legacy (formerly "Loadout") actually be, and which parts — the personal inventory tool, the shareable badge, the hiring-credibility signal — are worth building, in what order, and how do we make the credibility layer trustworthy if we ever get there?

**Update rule:** no schedule. Add an entry when something shifts the thinking — a new perspective, a reframe, a piece of evidence that changes the shape of the problem, or a decision/pivot. Skip entries that just restate the last one.

**Entry format:** each entry opens with a *Thread* line — where it came from, where it's pointing next — so the log reads as one continuous line of thinking, not a pile of disconnected notes.

*Last updated: 2026-09-19 22:58*

---

### 2026-09-16 · 22:07–22:16 — Resumed from the panther-session origin story

*Thread: picked up a new session in `mconfig` after a prior session (in the `panther` repo) ended by launching this one. Opens into: the scope-expansion push right after.*

- The whole idea traces back to an accident: running `npm install -g stackshare` inside the `panther` repo auto-published a public stack page at `stackshare.io/akpmohan07/panther`, exposing that project's existence tied to the user's identity.
- That triggered research into precedents (StackShare, `awesome-uses`/uses.tech) and a deep read of [`flaviocopes/cli-tools`](https://github.com/flaviocopes/cli-tools) — a real, well-built local-only detector (Homebrew/npm/Cargo + shell/agent history) with no sharing layer.
- **Why this matters:** the gap identified from the start — nobody combines auto-detection with a persistent, shareable profile — is the seed everything else grew from.

---

### 2026-09-16 · 22:26–22:45 — Scope pushed from "CLI tools" to "the whole machine"

*Thread: asked to finish ideation and start setup; user immediately widened it. Opens into: the first naming pass.*

- User rejected CLI-only scope: "it is like whole thing, physical hardware, desktop apps, extension, browser tools."
- Introduced two ideas that shaped everything downstream: a **simulated desktop UI** (the profile should *look like* a real machine, not read as a list) and a **decentralized, uses.tech-style deploy** (no accounts/hosting — users clone a template and point it at their own data repo).
- Also named the GitHub-README "top 5 tools" embed as a feature, and confirmed feasibility: on macOS, hardware (`system_profiler`), apps (`/Applications` + `Info.plist`), and browser extensions (per-extension `manifest.json` files) are *all* auto-detectable, not manual-entry like uses.tech.
- **Why this matters:** this is the moment the project stopped being "a competitor to cli-tools" and became a distinct, bigger thing.

---

### 2026-09-16 · 22:47–22:52 — First naming pass finds nothing that sticks

*Thread: tried unthemed (Loadout, Rig, Stackfolio), then space-mission (Manifest, Payload, Telemetry, Beacon), then archaeology/identity (Artifact, Fingerprint, Snapshot, Habitat). Opens into: the reframe that made naming tractable.*

- None of these were wrong, but none were being judged against a clear target — naming was drifting on vibes.
- **Why this matters:** the fix wasn't a better name, it was a better problem statement (next entry).

---

### 2026-09-16 · 22:52 — Reframe: two problems, not one

*Thread: asked directly "define the actual problem it's solving." Opens into: naming re-run, and the whole v1/v-later sequencing that followed for the rest of the night.*

- Split the idea into **Problem 1 — tool amnesia** (validated: flavio's own stated reason for building `cli-tools`, an independent tweet reply confirming the same pain, the `stackshare` incident itself) and **Problem 2 — no good way to show your setup** (weaker evidence: `awesome-uses` gets one-time submissions for 9 years but nobody revisits it; StackShare decayed as a company).
- Called it explicitly: if something has to be cut under time pressure, cut Problem 2, not the detector.
- **Why this matters:** every later scoping decision (v1 = detector first, Problem 3 sequenced last) traces back to keeping these two problems separate instead of solving a blended, mushier one.

---

### 2026-09-16 · 22:54–22:55 — Naming re-run against Problem 1 promotes Loadout and Manifest

*Thread: re-ran the naming list through the "solves tool amnesia" lens instead of "sounds nice." Opens into: the collision search.*

- **Beacon**, **Payload**, **Telemetry** got demoted — they all describe *broadcasting outward* (Problem 2's job), not *remembering* (Problem 1's job).
- **Loadout** ("what you're currently equipped with") and **Manifest** (a cargo manifest exists precisely so nobody has to re-derive what's aboard) got promoted — direct fits for the primary problem.
- **Memento** introduced as a new pure-memory-themed option.
- **Why this matters:** first time naming was actually being *evaluated* against a criterion instead of vibes.

---

### 2026-09-16 · 23:04–23:11 — Web search: Stash is a bad collision, the UI idea is a known genre, verdict is go

*Thread: asked to search the web for prior art and name collisions. Opens into: the actual project setup (folder + IDEATION.md).*

- **Stash** ruled out hard: [`stashapp/stash`](https://github.com/stashapp/stash) is a well-known, unrelated adult-content media organizer — not just "taken," actively confusing to be found next to.
- **Memento** has real but lesser collisions (an HTTP-caching tool, a web-archive CLI). **Loadout** came back clean.
- Searched for the simulated-desktop UI idea specifically and found it's an *established genre* — [`portfolio-os`](https://github.com/DareDev256/portfolio-os), [`my-portfolio`](https://github.com/Justinianus2001/my-portfolio), a [dev.to writeup](https://dev.to/dustinbrett/how-i-made-a-desktop-environment-in-the-browser-15oi) — but every example is hand-built with static content, never wired to *real* detected data. De-risks the rendering technique without erasing the novelty.
- Also checked: enterprise IT/hardware inventory tools (ManageEngine, Spiceworks) exist but for a completely different audience (sysadmins auditing a fleet), confirming the personal/shareable angle is still open.
- Consensus reached: **build it** — Problem 1 is real, nothing combines full-spectrum auto-detection the way this would, the UI technique is de-risked.
- **Why this matters:** this is the last point before any project artifact existed — everything after this is either building the doc or drifting into Problem 3.

---

### 2026-09-16 · 23:13–23:16 — Registry/leaderboard designed as v-later; project actually created

*Thread: asked whether cross-deployment metrics ("top tools") are possible under a decentralized, no-server model. Opens into: the folder and `IDEATION.md` getting written.*

- Designed the mechanism without building it: an opt-in registry repo (PR-based submission, same mechanic as `awesome-uses`) + a scheduled GitHub Action that fetches everyone's public `manifest.json` and computes aggregate stats. Safe to defer because it's a batch job on free compute, not a service that can go down — every individual page keeps working even if the registry is abandoned.
- Created `~/Workspace/tools/loadout/` and wrote `IDEATION.md` capturing everything up to this point.
- **Why this matters:** first working artifact of the night — the strategy stopped living only in chat.

---

### 2026-09-16 · 23:47–23:56 — Author/reader value exercise surfaces Problem 3

*Thread: asked to define use cases and value for the author vs. the reader of a published profile. Opens into: the rest of the night, essentially.*

- Reader use case "evaluating a person (recruiters, collaborators)" led the user to draw a direct analogy to [usegitai.com](https://usegitai.com/) (Git AI) — a tool that gives line-level, per-agent attribution of AI-generated code via Git Notes, turning an invisible claim into verifiable evidence.
- Named **Problem 3**: job descriptions demand "expertise in X," resumes claim it, nobody can verify it — but the evidence (install date, frequency, recency, and the human-vs-agent usage split already designed into `cli-tools`' `AgentHistory`) already exists locally.
- **Why this matters:** this is a genuinely different audience (employers, not just the developer) and different stakes (real harm if done carelessly) than Problems 1 and 2 — flagged as such immediately, but pursued anyway for the rest of the session.

---

### 2026-09-17 · 00:19–00:38 — Problem 3 risk pass, plus three Raycast-inspired side ideas

*Thread: asked to dig one level deeper into the honest risks named for Problem 3. Opens into: the prior-art correction and the WakaTime discovery.*

- Three risks named: (1) evaluative stakes / false negatives from data that only exists on one machine, (2) local shell history undercounting remote/SSH/cloud work, (3) gameability once usage stats have stakes.
- Comparing to Raycast's snippet-triggering produced two distinct, unrelated features: a **snippet-style distribution trigger** (keystroke expands your top-tools list anywhere on the system) and a **command-palette search overlay** inside the simulated desktop itself (also the concrete mechanism for AXIALIS's "group tools by job" reply on flavio's original tweet).
- Risk 2 specifically produced a mitigation: capture the *local terminal's* screen buffer via `tmux capture-pane` (or iTerm2's API) on a keyboard trigger — works for SSH/remote sessions because tmux just renders bytes, doesn't care what's producing them, so zero remote install is needed.
- **Why this matters:** the risk analysis is what eventually forces the "are we going in the right direction" reckoning two hours later.

---

### 2026-09-17 · 01:00–01:03 — Prior-art search corrected, WakaTime found

*Thread: first search targeted the tmux-capture mechanism narrowly (nothing found); user corrected the ask back to Problem 3 broadly. Opens into: the consensus check.*

- No existing tool combines a keyboard-triggered, zero-remote-install capture with personal tool-usage tracking — the adjacent tools (SSHLog, auditd) are security/audit daemons for sysadmins monitoring *other* users, requiring server-side install.
- **[WakaTime](https://definable.ai/apps/wakatime/)** found as the real, important precedent: automatic editor-activity tracking, public profiles, a browsable leaderboard with a "hireable" filter — validates the *mechanism* (automatic tracking → public profile → hiring signal) but tracks *coding time*, not tools/apps/hardware. Different data, same shape.
- Notable: WakaTime's model is an openly browsable, recruiter-searchable leaderboard — a more open design than the "candidate-controlled, no query-by-recruiter" model proposed earlier, and apparently acceptable to its userbase for years.
- **Why this matters:** first concrete evidence that the "automatic evidence → hiring signal" category is real and has working precedent, not just theory.

---

### 2026-09-17 · 01:16–01:21 — Consensus check flags the effort-allocation problem; user's rebuttal sharpens the sequencing

*Thread: asked directly "what's the consensus you think." Opens into: the bootstrap-mechanism answer.*

- Consensus given: Problem 1 = build now (validated, feasible, no competitor). Everything else = well-reasoned "maybe later," not "build now." Flagged directly: three hours in, zero code exists yet.
- User pushed back with the real reason Problem 3 matters: if the platform plays a role in getting people hired, "there will be more transactions... it will create value for both parties" — the two-sided-marketplace argument, the actual difference between a nice tool and a business with network effects.
- Response sharpened rather than dropped the sequencing point: a marketplace with no supply has no value for demand and vice versa — Problem 1/2 adoption *is* the only mechanism that makes Problem 3 reachable, proven by how WakaTime itself got there (personal tracking first, hireable leaderboard only meaningful once a base already existed).
- Also surfaced a structural tension: an actual transaction marketplace needs centralized matching/accounts eventually — a different build than the decentralized template, sharing only the manifest data layer.
- **Why this matters:** this is the entry that turns "build Problem 1 first" from a cautious default into the *only* viable path to the ambitious version.

---

### 2026-09-17 · 01:38–01:42 — Bootstrap mechanism concretized; company-policy risk surfaces the privacy model

*Thread: asked whether devs could be pitched on showcasing this in their GitHub profile. Opens into: the tamper-resistance rabbit hole.*

- Confirmed as the actual bootstrap mechanism: get devs to adopt the badge for vanity/fun (zero hiring intent), exactly like people add tech-stack stat cards today — credibility framing must be the *second act*, never the opening pitch, or self-consciousness about being evaluated kills adoption before it starts.
- User immediately asked the sharper question: does showcasing tool usage risk violating an employer's policy? Real risk identified — security/EDR/MDM tool names are reconnaissance value for attackers, NDA clauses cover "systems/tools/vendors" broadly, internal package names can leak context.
- User's own proposed fix — track only "approved," generic tools (aws-cli, terraform) — was upgraded into a general design principle: **allowlist by source, not denylist by name.** Tools from public registries (Homebrew core, public npm, crates.io) default visible; tools from `.local`/`.path`/custom taps default hidden, since internal company tools are essentially never published to public registries. Fails closed instead of relying on anticipating every possible sensitive tool.
- Hard rule locked: detection stays fully automatic; **publishing never is** — a direct, deliberate callback to the `stackshare` incident that started this whole project.
- **Why this matters:** the privacy model is now a v1 design constraint, not a Problem-3-only concern — it can bite on the very first "fun badge" use case.

---

### 2026-09-17 · 01:44–01:57 — Tamper-resistance: burst-detection dead-ends, OpenTimestamps is the real fix

*Thread: asked how to tackle someone run-looping a command to fake usage. Opens into: the checkpoint-cadence decision.*

- First-pass mitigations proposed: burst-timing detection, dedup by distinct (command+args+cwd), daily caps, calendar-heatmap display instead of a bare number.
- User's next question broke all of it at once: **"can't they tamper the time?"** — yes, completely. Shell history is a plain-text file anyone can hand-edit; the system clock can be changed before running commands; file `mtime` is trivially set with `touch -t`; even a *local* git commit's author-date is a field git lets you set explicitly. No purely local, client-side signal can bind a timestamp to truth when the subject controls the whole machine generating the "evidence."
- Follow-up ("hashing techniques?") clarified the actual fix: hashing alone doesn't help (a hash of fabricated data is still a valid hash) — the fix is submitting *only the hash* to an independent third party and letting *them* record the observation time, using collision-resistance to make retroactive data-swapping detectable.
- **[OpenTimestamps](https://opentimestamps.org/)** identified as the concrete, already-existing, zero-infrastructure mechanism: hash locally, submit to free public calendar servers (`a.pool.opentimestamps.org`, `b.pool.opentimestamps.org`, `a.pool.eternitywall.com`, `alice.btc.calendar.opentimestamps.org`, `bob.btc.calendar.opentimestamps.org`), get a small `.ots` proof anchored into Bitcoin, verify later against the public blockchain with no service needed.
- **Why this matters:** this is the sharpest adversarial exchange of the night — a two-question pushback ("can't they tamper the time," "hashing techniques") took the design from "plausible-sounding heuristics" to "the actual correct cryptographic primitive," and exposed that the earlier git-commit-timeline idea had the identical flaw (local commit dates are also just a settable field).

---

### 2026-09-17 · 02:04–02:07 — Checkpoint cadence settled; self-critique closes the night

*Thread: asked whether every usage event needs timestamping. Opens into: nothing downstream yet — this is the most recent entry.*

- Settled: snapshot the *full* aggregate state and hash-anchor it on each manifest publish, never per individual command — per-event timestamping is impractical (a network round-trip per shell command) and unnecessary, since checkpoint-to-checkpoint diffs already reveal an implausible jump.
- Asked directly "are we going in the right direction" — answered honestly: the thinking is sound and the sequencing logic has only gotten *more* solid, but the last two hours designed cryptographic tamper-resistance for a feature (Problem 3) sitting three dependency-levels above code that doesn't exist yet (Problem 3 needs the registry, which needs Problem 2 adoption, which needs the Problem 1 detector — none built).
- Findings folded into `IDEATION.md` (Problem 3 section, privacy model, tamper-resistance mechanism, the three Raycast-inspired ideas); this research log started immediately after, at the user's request, to preserve the actual path of reasoning rather than only the polished conclusions.
- **Why this matters:** names the pattern explicitly so it doesn't repeat silently next session — the doc captures conclusions, this log captures how they were reached, and the open thread now is deliberately about narrowing, not widening further.

---

### 2026-09-17 · 02:24 — AI-era doubt: does usage frequency still mean skill?

*Thread: asked directly whether these metrics still show a person's real experience with a tool, in the AI era. Opens into: the production-practices research that followed immediately after.*

- Named three compounding problems, worse than tampering because they apply to a completely honest user: agentic tool-use is only partly caught by the human-vs-agent split; copy-pasting an AI-suggested command into your own terminal is invisible to shell history entirely; even self-typed commands increasingly execute AI-authored logic, so "ran it" no longer implies "understood it" the way it did pre-AI.
- **Why this matters:** this is a validity problem, not a gaming problem — it applies even if nobody is trying to cheat. A second, independent reason (beyond the cold-start problem) the credibility signal stays parked.

### 2026-09-17 · 02:36 — What real hiring practice already does, researched directly

*Thread: asked what companies actually do in production given this. Opens into: the agent-audit tooling search.*

- Found: 71% of engineering leaders say AI makes skill assessment harder, and the industry response has been to move *away* from async, self-generated evidence (take-home tests specifically), not toward better versions of it — because a usage-history profile is structurally the same shape as a take-home.
- What replaced it: **AI-disabled live rounds** (raw fundamentals, observed) and **AI-assisted live rounds** (watching how someone prompts/judges AI output) — both synchronous and observed, neither async. Concrete techniques already in use: "explain your reasoning" follow-ups, and demanding "a specific example placed in time" rather than trusting a claim.
- The valued skill has explicitly shifted from "can you produce" to "can you judge."
- **Why this matters:** reframed the eventual credibility feature from "a score that replaces the interview" to "material that feeds one" — directly shaped the white-paper positioning decided later in the night.

### 2026-09-17 · 02:39–02:54 — Agent-audit tooling found; the ownership fork gets resolved

*Thread: asked whether any tool captures commands run by coding agents specifically. Opens into: nothing downstream yet — this is the most recent entry.*

- Found a real, mature enterprise category doing exactly this: **[MintMCP](https://www.mintmcp.com/blog/claude-code-monitoring)** (proxy-based monitoring of Claude Code/Cursor/Codex/Copilot commands and MCP calls), **[Nylas CLI](https://cli.nylas.com/guides/audit-ai-agent-activity)** (auto-detects agent sources, logs to `~/.config/nylas/audit.log`, SIEM-exportable), and **Bifrost** (per-MCP-call audit records) — some already implement append-only, hash-verified immutable logs, i.e. the same tamper-evidence idea as OpenTimestamps, already productionized for security compliance rather than personal credibility.
- This surfaced a real fork: employer-controlled audit logs are a *more* trustworthy third-party witness than anything an individual's own machine can produce (solves the "who's the independent observer" problem tamper-resistance kept running into) — but that data belongs to the employer, not the person, and no employer has an incentive to help an employee export proof of their own market value.
- **User resolved the fork directly:** keep the personal, self-sovereign design — individuals use it on their own laptop, side projects, public repos, and it holds its own value with zero company participation, exactly like tool amnesia already does. Company interest becomes optional and pull-based, never a dependency.
- For the credibility claim specifically, proposed mechanism: **a white paper**, not a company-vouches-for-you model — since there's no central trusted operator in a decentralized system, credibility has to come from the same "publish the mechanism, make it independently verifiable" approach OpenTimestamps and Bitcoin themselves use. Sketched its required contents: open-source auditable detection, the privacy/allowlist model (a case *for* companies too — it's built not to leak their tooling), the tamper-resistance mechanism explained precisely, honest limits stated up front (not buried), and explicit positioning as material that *feeds* live interview verification rather than replacing it.
- **Why this matters:** resolves the ownership tension from the previous entry without picking a side — avoids both the B2B2C-audit-vendor pivot and the "wait for employer cooperation" dead end, while keeping every earlier tamper-resistance/privacy design decision intact and reusable.

---

### 2026-09-17 · 15:45–15:53 — Reframed from current-snapshot to journey-over-time; naming reopened

*Thread: reconsidering whether "Loadout" still fit given the credibility-signal scope surfaced a bigger shift — describing the product as capturing a developer's whole tool/system history across career upgrades, not a live inventory. Opens into: a full naming reconsideration, picked back up in the next session.*

- Talked through whether "Loadout" (chosen for the tool-amnesia framing) still fit once credibility was in view — concluded it does for tool amnesia specifically, but the conversation surfaced a distinct, stronger idea: a **retrospective/timeline of systems used over a career**, not a snapshot of current gear (see "Reframing" section added to `IDEATION.md`).
- Considered Manifest, Artifact, Provenance, Sourced as dual-fit names bridging inventory + credibility, before the reframing made all of them the wrong shape of question — the real gap was "current gear" vs. "history."
- **Why this matters:** this is the pivot that eventually produces the final name — everything from here forward is downstream of treating the product as a journey/history, not a live dashboard.

### 2026-09-19 · 21:48–22:38 — Naming reconsidered under the new frame; Legacy stress-tested

*Thread: resumed after a session gap with the reframing already in place. Opens into: the final naming decision.*

- User's gut instinct pointed to **"Legacy"** — nostalgia after a period of usage, plus a genuine pun on "legacy systems" (the old setups you outgrow). Flagged the real counter-risk directly: "legacy" is used pejoratively in dev culture ("legacy code," something to escape), the opposite of the intended pride/nostalgia.
- Ran real collision checks instead of guessing: `gh search repos` showed **every** top hit for "legacy" (`Homebrew/legacy-homebrew`, `BoostNote-Legacy`, `tenacity-legacy`, even a bare `ErsatzTV/legacy`) means "deprecated, superseded" — a strong, confirmed convention, not just a vibe. npm registry check found the exact package name `legacy` **already taken** (an old "Legacy browser style sheet generator," with its own `legacy` CLI binary).
- Tried "My Legacy" as a mitigation (dodges the adjectival "legacy code" pattern grammatically) — ruled out after search found it heavily colonized by an unrelated **digital-inheritance/estate-planning app cluster** (`my-legacy.ai`, `blockchainology/mylegacy`, an "Islamic Estate Planner," etc.), and confirmed "My ___" is itself a dated 2003–2005 naming convention (MySpace, MyFitnessPal), not a fresh one.
- Considered and rejected safer alternatives on other grounds: **Provenance** (best conceptual fit, but genuinely obscure vocabulary — user had never heard the word, same failure mode that cut Diorama/Terrarium earlier), **Chronicle** (safe but flat), **Trail/Track/Tracks** (plain and dev-adjacent via "audit trail"/"track record," but read generic, "like a company name").
- Re-verified the underlying product gap still holds under this framing: "Wrapped"-style tools (GitHub Wrapped, Git Wrapped) and codebase-history visualizers (Gource, Codebase Timeline Visualizer) both nail the nostalgic/shareable *format* but analyze commits/code, not a person's tool/environment inventory across machines and years — the combination this project targets is still unbuilt.
- **Why this matters:** every safer alternative lost to genericness or obscurity; only Legacy kept getting reconfirmed on instinct across multiple separate check-ins, which is itself signal for a founder-driven personal project, even carrying real, now-quantified risk.

### 2026-09-19 · 22:58 — Legacy locked; repo created

*Thread: closes the naming thread opened above. Opens into: nothing downstream yet — next is the detector stack/schema work already parked in "Open" below.*

- **Decision: bare "Legacy," risks accepted knowingly**, not resolved away — the npm package name will need a different registry name later (independent of the project/brand name), and positioning should lean into the "legacy code" ambiguity directly (e.g. "the systems you've outgrown, kept") rather than avoid it.
- Folder renamed `loadout` → `legacy`; GitHub repo, project board, and file sync set up same session.
- **Why this matters:** naming is no longer open — future sessions should stop revisiting it unless new information actually contradicts this decision, not just re-litigate the same tradeoffs.

---

### Open — live edge, as of 2026-09-17 02:54

*Thread: this section is the live edge of the log — always last, always open, rewritten (not appended to) as the thinking moves.*

- **Nothing has been built yet.** Zero lines of detector code, zero `manifest.json` schema, four+ hours in. The explicit next-session priority is narrowing to this, not further Problem 3 exploration.
- Detector stack still undecided — leaning Node/TS (shares an ecosystem with npm detection, keeps the door open for a template renderer in the same language) over Swift (macOS-only, ties to the same ecosystem `cli-tools` already occupies) — not confirmed.
- `manifest.json` schema not yet designed — the single artifact every downstream idea (v1 render, the registry, Problem 3's evidence view, OpenTimestamps anchoring) depends on getting right first.
- The credibility-signal direction is fully parked pending real adoption of the personal tool and the badge — not to be picked up again until there's an actual population of users to make it meaningful.
- Repo layout undecided: single repo with detector + template as separate packages, vs. two repos.
- A white paper (open-source detection, the privacy/allowlist model, the tamper-resistance mechanism, honest limits) was identified as the right eventual vehicle for the credibility claim — not written, not urgent, but now has a defined shape for whenever it's picked back up.

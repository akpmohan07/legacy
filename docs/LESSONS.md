# Guidelines — engineering rules worth following

General software-engineering guidelines, not specific to Legacy's own decisions. Kept here because they're reusable across future projects, not because they belong to this one — this file is meant to be copied into whatever comes next. When one of these actually shapes a Legacy-specific decision, the decision + reasoning goes in `IDEATION.md` as usual; this file gets nothing more than a pointer back, never a restatement (see `manifest.md`).

## Data model

- **Version your schema from day one.** A version field lets future column/type changes handle old rows that don't have it — added after the fact, it can't cover data already written.
- **Don't assume a "unique" key stays unique.** Treat things like bundle IDs or package names as likely-unique, not guaranteed — a real-world edge case will eventually break the assumption.
- **Add `created_at`/timestamps to any row that needs history.** It can't be backfilled once the data that would have populated it is already gone.
- **Fix timestamps to a stated timezone (UTC).** Local-time storage leads to silent "off by N hours" bugs once compared across sources.
- **Name timestamp columns with an `_at` suffix.** `installed_at`, `occurred_at`, `first_seen_at` — the suffix marks a point in time (a bare `installed` reads as a yes/no flag), and one convention across every table removes guessing. Reserve `_on` for date-only values.
- **Keep "when it happened" and "when we noticed" as separate fields.** A system that finds out later can only honestly record when it noticed; store the real date in its own optional field when the source provides one, instead of passing detection time off as the real event time.

## Error handling

- **Don't swallow errors with a default value everywhere.** Reserve that pattern for genuinely optional data — elsewhere it hides real failures behind "unknown."
- **Distinguish "can't happen" from "can fail."** Handle a malformed external input differently than a bug in your own logic — collapsing both into the same panic/unwrap loses that distinction.

## Concurrency

- **Design for more than one writer up front.** If two independent processes (e.g. a periodic scan and a continuous event capture) can touch the same store, decide the locking/contention behavior explicitly — don't let a "database is locked" error be the first time it's considered.

## Compatibility / portability

- **Build an export/import path from the start.** Retrofitting a way to get your own data back out is much harder than designing it in early.
- **Check the access pattern before locking a design, not after.** Something fine at small volume can fall over at real volume.

## Unicode / i18n

- **Don't assume ASCII or stable character length.** Multi-byte UTF-8, combining characters, emoji — truncation/length logic needs to handle non-ASCII data correctly.
- **Don't build sentences by string concatenation.** Word order and grammar break the moment more than one language is in play.
- **Handle pluralization as more than singular/plural.** Many languages have 3–6 plural forms; branch on more than just `count == 1`.
- **Don't hardcode date/number formats.** `MM/DD/YYYY` vs `DD/MM/YYYY`, `,` vs `.` as decimal separator — respect locale instead of assuming one.
- **Respect locale-sensitive sorting/case rules.** E.g. Turkish "İ/i" case-folding — `.to_lowercase()` doesn't mean the same thing everywhere.

## Process

- **Plan and analyze requirements before writing code.** Skipping this leads to scope creep and requirements discovered mid-build.
- **Gather feedback early**, rather than building in isolation and finding out late that what got built isn't what was needed.
- **Optimize only once there's evidence it's needed.** Don't build elaborate caching or complex data layers preemptively.
- **Never hardcode secrets/config directly in code.** Treat this as a flexibility and security requirement, not a nice-to-have.
- **Invest in documentation as you go.** Code becomes hard to maintain once the original author's context fades.

## Dependency / library selection

- **Write out every dimension a decision needs before calling a winner** — don't decide off whatever data happens to already be on hand. A verdict that "feels" data-backed (a benchmark score, one successful test run, a stars comparison) can still rest on a handful of convenient signals rather than the full picture.
- **Weigh real adoption signal over GitHub stars.** For a library, package-registry download counts and reverse-dependency counts are the stronger signal — they measure who actually put it in production, not who bookmarked it.
- **Verify "cross-platform" per field, per platform — don't take the claim at face value.** A library can genuinely support three OSes while individual fields silently return empty/null on just one of them.
- **Check maintenance recency, not just total downloads.** A package can carry a large download count from years of past popularity while being functionally abandoned now.
- **Look for concrete precedent before trusting a criteria table alone.** Finding that other serious, independent projects already depend on the same thing in production resolves a stalled decision faster and more reliably than re-weighing abstract criteria.

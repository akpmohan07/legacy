# Source docs

A **source** is one place tools are installed from (the `sources` table, one `Scanner` each). This
folder says what each source covers, how its tools are identified, and what it cannot see. How a
scanner plugs in is in `../decisions/0001-scanner-architecture-and-portability.md`; the checklist for adding one is in `detector/AGENTS.md`.

## Conventions
- One file per **built** source, named after its `source_name()`. A source that is only planned gets a
  row below, not a file, until someone investigates it. Do not write down how a planned source works
  unless it was checked on a real machine; mark anything else `unverified`.
- Front matter: `source`, `status` (`built` | `planned`), `platform`, `scanner` (file path), `issues`.
- Sections: **Scope** (in / out), **Identifier**, **Discovery**, **Version and install date**,
  **Absence check**, **Platform notes**, **Known gaps**, **Guarded by** (`- test:` lines), **Verified live**.
- A change to what a scanner returns changes its file here in the same commit.

## Index
| Source | Status | Platform | What it covers |
|---|---|---|---|
| [application](application.md) | built | macOS | `.app` bundles directly inside `/Applications` |
| [homebrew-cellar](homebrew-cellar.md) | built | macOS | Homebrew formulae installed on request |
| homebrew-cask | planned | macOS | Apps installed with `brew install --cask`. Today they show up under `application`; see the precedence gap in `application.md` |
| cargo | planned | any | Binaries installed with `cargo install` (unverified) |
| npm-global | planned | any | Packages installed with `npm install -g` (unverified) |
| mac-app-store | planned | macOS | Apps from the Mac App Store. Today they show up under `application` |

`homebrew-cask`, `cargo`, `npm-global`, and `mac-app-store` already exist as rows in the `sources`
table (migration `seed_sources`) but have no scanner, so they are never probed.

## Overlap between sources
One installed thing can be seen by two sources: a cask app is in `/Applications` (source `application`)
and in the Caskroom; a Mac App Store app is in `/Applications` too. Identifiers are per source, so the
same app would be recorded twice. **Which source owns a duplicate is undecided**; it is to be settled
when the first overlapping scanner is written.

# Legacy

Legacy finds the tools and apps on your Mac by itself and records how that set changes over time. The goal is the story of your setup: what you used, when you switched, what you dropped. It runs on your own machine, and nothing is published unless you choose to.

[![A short recording of a simulated desktop of a real Mac, drawn from Legacy's database. Click to open the live demo.](https://akpmohan07.github.io/legacy/preview.gif)](https://akpmohan07.github.io/legacy/)

**[Open the live demo](https://akpmohan07.github.io/legacy/)**: a simulated desktop of the author's own Mac, drawn from what the detector actually recorded. Open Finder, Launchpad, Spotlight (⌘K) or the terminal and click around.

The demo is a proof of concept (`poc/`), not the product. It shows one real machine with almost nothing filtered, to find out what should and should not be shared.

## What exists today

| Part | State |
|---|---|
| `detector/` | Rust, macOS only. Scans `/Applications` and Homebrew, records installs, updates and removals in a local SQLite database, and can keep watching for changes. |
| `poc/` | Throwaway viewers built on that database: a timeline and the simulated desktop above. |
| Review step, `legacy.json` export, renderer, registry | Designed, not built. |

## Where to read more

- [docs/NORTH-STAR.md](docs/NORTH-STAR.md): the one-page pitch
- [docs/IDEATION.md](docs/IDEATION.md): product decisions and the reasoning behind them
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): how the parts fit together
- [docs/research-log.md](docs/research-log.md): how the thinking got here, in order
- [docs/manifest.md](docs/manifest.md): what each document is for
- [detector/AGENTS.md](detector/AGENTS.md): how to build and work on the detector

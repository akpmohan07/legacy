//! `legacy-detector watch`: stay running and scan a source when its files change (issue 17, v1).
//!
//! Flow: a startup scan of every source, then watch each source's folders and loop:
//! wait for an event, drain everything already queued, keep the events that matter to a source,
//! and scan each affected source once. A file event is only a doorbell: its path picks the source,
//! and the scan re-reads the source in full. Events that arrive during a scan wait in the channel
//! and cause one follow-up scan on the next pass.
//!
//! v1 has no quiet window, no backstop scan, and no login item (decision 0012).

use crate::domain::time::utc_string;
use crate::domain::TriggeredBy;
use crate::run::{self, RunError, RunReport};
use crate::scanner::{Scanner, ScannerRegistry, Trigger};
use crate::store::Store;
use notify::{RecursiveMode, Watcher};
use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;
use std::sync::mpsc;

#[derive(Debug)]
pub enum WatchError {
    /// The database failed; same meaning as a manual run's exit 2.
    Database(RunError),
    /// The file watcher could not start.
    Watcher(notify::Error),
    /// No source has a folder to watch on this machine.
    NothingToWatch,
    /// The file watcher stopped sending events.
    Stopped,
}

impl From<RunError> for WatchError {
    fn from(err: RunError) -> Self {
        WatchError::Database(err)
    }
}

impl From<notify::Error> for WatchError {
    fn from(err: notify::Error) -> Self {
        WatchError::Watcher(err)
    }
}

impl fmt::Display for WatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchError::Database(err) => write!(f, "{err}"),
            WatchError::Watcher(err) => write!(f, "file watcher failed: {err}"),
            WatchError::NothingToWatch => write!(f, "no source has a folder to watch"),
            WatchError::Stopped => write!(f, "file watcher stopped"),
        }
    }
}

/// One file-system notification, reduced to what the watcher uses.
#[derive(Debug, Clone)]
pub struct Change {
    /// The OS's kind of change (create, modify, remove, ...); for logs only.
    pub kind: String,
    pub paths: Vec<PathBuf>,
    /// The OS says events may have been lost, so any watched source may have changed.
    pub rescan: bool,
}

impl From<notify::Event> for Change {
    fn from(event: notify::Event) -> Self {
        Change {
            kind: format!("{:?}", event.kind),
            rescan: event.need_rescan(),
            paths: event.paths,
        }
    }
}

/// A watched folder and the scanner it belongs to.
pub struct Root<'a> {
    pub path: PathBuf,
    pub scanner: &'a dyn Scanner,
}

/// The folders to watch, and the sources with nothing to watch. A candidate folder that doesn't
/// exist is skipped. Paths are resolved (symlinks, `/private`) so they match what the OS reports.
pub fn watch_roots(registry: &ScannerRegistry) -> (Vec<Root<'_>>, Vec<&'static str>) {
    let mut roots = Vec::new();
    let mut unwatched = Vec::new();

    for scanner in registry.scanners() {
        match scanner.trigger() {
            Trigger::None => unwatched.push(scanner.source_name()),
            Trigger::Paths(candidates) => {
                let before = roots.len();
                for candidate in candidates {
                    if let Ok(path) = std::fs::canonicalize(&candidate) {
                        roots.push(Root { path, scanner });
                    }
                }
                if roots.len() == before {
                    unwatched.push(scanner.source_name());
                }
            }
        }
    }
    (roots, unwatched)
}

/// What a batch of changes came to: the affected sources, and path counts for the logs.
#[derive(Debug, Default)]
pub struct Routing {
    pub changed: BTreeSet<&'static str>,
    pub relevant: usize,
    pub ignored: usize,
    pub outside: usize,
    pub rescans: usize,
}

/// Which sources a batch of changes affects. A path belongs to the deepest root containing it,
/// and counts only if that root's scanner calls it relevant. No I/O; each path is logged at trace.
pub fn route(roots: &[Root], changes: &[Change]) -> Routing {
    let mut routing = Routing::default();

    for change in changes {
        if change.rescan {
            tracing::debug!(kind = %change.kind, "rescan notice: every watched source marked");
            routing.rescans += 1;
            routing.changed.extend(roots.iter().map(|root| root.scanner.source_name()));
            continue;
        }
        for path in &change.paths {
            let owner = roots
                .iter()
                .filter(|root| path.starts_with(&root.path))
                .max_by_key(|root| root.path.components().count());
            let Some(root) = owner else {
                tracing::trace!(kind = %change.kind, path = %path.display(), "outside every watched folder");
                routing.outside += 1;
                continue;
            };
            let relative = path.strip_prefix(&root.path).unwrap_or(path);
            let source = root.scanner.source_name();
            if root.scanner.is_relevant(relative) {
                tracing::trace!(kind = %change.kind, path = %path.display(), source, "relevant");
                routing.relevant += 1;
                routing.changed.insert(source);
            } else {
                tracing::trace!(kind = %change.kind, path = %path.display(), source, "ignored");
                routing.ignored += 1;
            }
        }
    }
    routing
}

/// The sources a batch of changes affects (see `route`).
pub fn changed_sources(roots: &[Root], changes: &[Change]) -> BTreeSet<&'static str> {
    route(roots, changes).changed
}

/// Scan the sources a batch affects, in one run. `None` when nothing relevant changed.
pub fn process_batch(
    store: &mut Store,
    registry: &ScannerRegistry,
    roots: &[Root],
    changes: &[Change],
) -> Result<Option<RunReport>, RunError> {
    let routing = route(roots, changes);
    tracing::debug!(
        events = changes.len(),
        relevant = routing.relevant,
        ignored = routing.ignored,
        outside = routing.outside,
        rescans = routing.rescans,
        sources = ?routing.changed,
        "batch routed"
    );
    if routing.changed.is_empty() {
        return Ok(None);
    }
    let sources: Vec<&str> = routing.changed.into_iter().collect();
    let started = std::time::Instant::now();
    let report = run::run_only(store, registry, &sources, TriggeredBy::Watcher)?;
    tracing::debug!(?sources, elapsed_ms = started.elapsed().as_millis() as u64, "watcher scan done");
    Ok(Some(report))
}

fn now() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or(0);
    utc_string(seconds)
}

/// Runs until the watcher stops or the database fails. Prints one block per scan.
pub fn watch(store: &mut Store, registry: &ScannerRegistry) -> Result<(), WatchError> {
    let started = std::time::Instant::now();
    let startup = run::run(store, registry, TriggeredBy::Startup)?;
    tracing::debug!(elapsed_ms = started.elapsed().as_millis() as u64, "startup scan done");
    println!("[{} UTC] startup scan\n{}\n", now(), startup.summary());

    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    let (roots, unwatched) = watch_roots(registry);
    let mut watched = Vec::new();
    for root in roots {
        match watcher.watch(&root.path, RecursiveMode::Recursive) {
            Ok(()) => {
                println!("watching {} ({})", root.path.display(), root.scanner.source_name());
                watched.push(root);
            }
            Err(err) => {
                tracing::warn!(path = %root.path.display(), error = %err, "cannot watch; skipped");
            }
        }
    }
    for source in unwatched {
        println!("not watched: {source} (scanned at startup only)");
    }
    if watched.is_empty() {
        return Err(WatchError::NothingToWatch);
    }
    println!();

    loop {
        tracing::debug!("waiting for file events");
        let first = rx.recv().map_err(|_| WatchError::Stopped)?;
        let mut changes = Vec::new();
        for item in std::iter::once(first).chain(rx.try_iter()) {
            match item {
                Ok(event) => changes.push(Change::from(event)),
                Err(err) => tracing::warn!(error = %err, "watch error"),
            }
        }

        match process_batch(store, registry, &watched, &changes)? {
            Some(report) => println!(
                "[{} UTC] watcher scan ({} file events)\n{}\n",
                now(),
                changes.len(),
                report.summary()
            ),
            None => tracing::trace!(events = changes.len(), "no relevant change; no scan"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::DiscoveredTool;
    use crate::scanner::{Probe, ScanError};
    use crate::schema::scans;
    use diesel::prelude::*;
    use std::path::Path;

    /// Treats any path containing "noise" as irrelevant.
    struct Fake(&'static str);

    impl Scanner for Fake {
        fn source_name(&self) -> &'static str {
            self.0
        }
        fn probe(&self) -> Probe {
            Probe::Available { version: None }
        }
        fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
            Ok(vec![])
        }
        fn is_relevant(&self, relative: &Path) -> bool {
            !relative.to_string_lossy().contains("noise")
        }
    }

    static APPS: Fake = Fake("application");
    static CELLAR: Fake = Fake("homebrew-cellar");

    fn roots() -> Vec<Root<'static>> {
        vec![
            Root { path: PathBuf::from("/w/apps"), scanner: &APPS },
            Root { path: PathBuf::from("/w/cellar"), scanner: &CELLAR },
        ]
    }

    fn change(paths: &[&str]) -> Change {
        Change { kind: "Modify".into(), paths: paths.iter().map(PathBuf::from).collect(), rescan: false }
    }

    fn registry() -> ScannerRegistry {
        ScannerRegistry::from_scanners(vec![Box::new(Fake("application")), Box::new(Fake("homebrew-cellar"))])
    }

    fn watcher_scans(store: &mut Store) -> i64 {
        scans::table
            .filter(scans::triggered_by.eq("watcher"))
            .count()
            .get_result(&mut store.conn)
            .unwrap()
    }

    #[test]
    fn a_burst_under_one_folder_marks_only_that_source_once() {
        let changes: Vec<Change> =
            (0..300).map(|i| change(&[&format!("/w/apps/Raycast.app/file-{i}")])).collect();
        assert_eq!(changed_sources(&roots(), &changes), BTreeSet::from(["application"]));
    }

    #[test]
    fn irrelevant_and_outside_paths_mark_nothing() {
        let changes = vec![
            change(&["/w/apps/Raycast.app/noise"]),
            change(&["/elsewhere/file"]),
            change(&["/w/apps-other/Raycast.app"]),
        ];
        assert!(changed_sources(&roots(), &changes).is_empty());
    }

    #[test]
    fn changes_under_two_folders_mark_both_sources() {
        let changes = vec![change(&["/w/apps/Raycast.app", "/w/cellar/gh/2.9.0"])];
        assert_eq!(
            changed_sources(&roots(), &changes),
            BTreeSet::from(["application", "homebrew-cellar"])
        );
    }

    #[test]
    fn routing_counts_relevant_ignored_and_outside_paths() {
        let changes = vec![change(&["/w/apps/Raycast.app", "/w/apps/noise", "/elsewhere"])];
        let routing = route(&roots(), &changes);
        assert_eq!((routing.relevant, routing.ignored, routing.outside), (1, 1, 1));
    }

    #[test]
    fn a_rescan_notice_marks_every_watched_source() {
        let changes = vec![Change { kind: "Other".into(), paths: vec![], rescan: true }];
        assert_eq!(
            changed_sources(&roots(), &changes),
            BTreeSet::from(["application", "homebrew-cellar"])
        );
    }

    #[test]
    fn a_path_belongs_to_the_deepest_folder_that_contains_it() {
        let nested = vec![
            Root { path: PathBuf::from("/w"), scanner: &APPS },
            Root { path: PathBuf::from("/w/cellar"), scanner: &CELLAR },
        ];
        let changes = vec![change(&["/w/cellar/gh"])];
        assert_eq!(changed_sources(&nested, &changes), BTreeSet::from(["homebrew-cellar"]));
    }

    #[test]
    fn a_batch_of_many_events_causes_one_scan_of_the_affected_source() {
        let mut store = Store::in_memory();
        let registry = registry();
        run::run(&mut store, &registry, TriggeredBy::Startup).unwrap();

        let changes: Vec<Change> =
            (0..50).map(|i| change(&[&format!("/w/cellar/gh/file-{i}")])).collect();
        let report = process_batch(&mut store, &registry, &roots(), &changes).unwrap().unwrap();

        assert_eq!(report.sources.len(), 1);
        assert_eq!(report.sources[0].source, "homebrew-cellar");
        assert_eq!(watcher_scans(&mut store), 1);
    }

    #[test]
    fn a_batch_with_no_relevant_change_scans_nothing() {
        let mut store = Store::in_memory();
        let registry = registry();
        run::run(&mut store, &registry, TriggeredBy::Startup).unwrap();

        let report = process_batch(&mut store, &registry, &roots(), &[change(&["/w/apps/noise"])]).unwrap();

        assert!(report.is_none());
        assert_eq!(watcher_scans(&mut store), 0);
    }
}

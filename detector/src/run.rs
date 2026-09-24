//! One run of the detector: discover the sources, scan each one independently, record the
//! results, and report. Contains no SQL and no filesystem access of its own.
//!
//! Error policy: a source that fails (probe or scan) is logged and skipped, the others carry on,
//! and the run exits 1. A database failure stops the run (`RunError`, exit 2): continuing could
//! put the history at risk.

use crate::domain::plan::Observation;
use crate::domain::{Absence, TriggeredBy};
use crate::scanner::{KnownTool, Probe, ScanError, ScannerRegistry, SourceProbe};
use crate::store::apply::ApplyReport;
use crate::store::Store;
use std::fmt;

/// The database failed; the run cannot continue safely.
#[derive(Debug)]
pub struct RunError(pub diesel::result::Error);

impl From<diesel::result::Error> for RunError {
    fn from(err: diesel::result::Error) -> Self {
        RunError(err)
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "database error: {}", self.0)
    }
}

#[derive(Debug)]
pub enum SourceOutcome {
    /// Scanned and recorded. `baseline` is true when this was the source's first recorded scan.
    Recorded { tools: usize, baseline: bool, events: ApplyReport },
    /// The source isn't on this machine. `events` holds its own uninstalled event, if it just left.
    NotPresent { events: ApplyReport },
    /// Nothing was recorded; the scan row keeps an empty `finished_at`.
    Failed { reason: String },
}

#[derive(Debug)]
pub struct SourceReport {
    pub source: &'static str,
    pub outcome: SourceOutcome,
}

#[derive(Debug)]
pub struct RunReport {
    pub sources: Vec<SourceReport>,
}

impl RunReport {
    pub fn failed(&self) -> usize {
        self.sources
            .iter()
            .filter(|s| matches!(s.outcome, SourceOutcome::Failed { .. }))
            .count()
    }

    pub fn events(&self) -> u32 {
        self.sources
            .iter()
            .map(|s| match &s.outcome {
                SourceOutcome::Recorded { events, .. } | SourceOutcome::NotPresent { events } => {
                    event_total(events)
                }
                SourceOutcome::Failed { .. } => 0,
            })
            .sum()
    }

    /// 0 when every source was recorded, 1 when any source failed.
    pub fn exit_code(&self) -> i32 {
        i32::from(self.failed() > 0)
    }

    pub fn summary(&self) -> String {
        let mut lines: Vec<String> = self
            .sources
            .iter()
            .map(|s| format!("{:<16} {}", s.source, describe(&s.outcome)))
            .collect();
        lines.push(format!(
            "{} sources checked, {} failed, {} events",
            self.sources.len(),
            self.failed(),
            self.events()
        ));
        lines.join("\n")
    }
}

fn event_total(events: &ApplyReport) -> u32 {
    events.installed + events.updated + events.uninstalled
}

fn describe_events(events: &ApplyReport) -> String {
    [
        (events.installed, "installed"),
        (events.updated, "updated"),
        (events.uninstalled, "uninstalled"),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, word)| format!("{count} {word}"))
    .collect::<Vec<_>>()
    .join(", ")
}

fn describe(outcome: &SourceOutcome) -> String {
    match outcome {
        SourceOutcome::Recorded { tools, baseline: true, .. } => {
            format!("{tools} tools   baseline recorded (no events)")
        }
        SourceOutcome::Recorded { tools, events, .. } if event_total(events) == 0 => {
            format!("{tools} tools   no changes")
        }
        SourceOutcome::Recorded { tools, events, .. } => {
            format!("{tools} tools   {}", describe_events(events))
        }
        SourceOutcome::NotPresent { events } if event_total(events) == 0 => "not present".to_string(),
        SourceOutcome::NotPresent { events } => format!("not present   {}", describe_events(events)),
        SourceOutcome::Failed { reason } => format!("FAILED: {reason}"),
    }
}

fn failed(what: &str, err: ScanError) -> SourceOutcome {
    tracing::warn!(error = %err, "{}; nothing recorded for this source", what);
    SourceOutcome::Failed { reason: format!("{what}: {err}") }
}

pub fn run(
    store: &mut Store,
    registry: &ScannerRegistry,
    triggered_by: TriggeredBy,
) -> Result<RunReport, RunError> {
    let first_run = store.is_first_run()?;
    let run_id = store.new_run_id()?;
    let mut sources = Vec::new();

    for SourceProbe { scanner, probe } in registry.discover() {
        let name = scanner.source_name();
        let span = tracing::info_span!("source", source = name);
        let _entered = span.enter();

        let source_id = store.source_id(name)?;
        let scan_id = store.begin_scan(&run_id, source_id, &triggered_by)?;

        let outcome = match probe {
            Probe::Failed(err) => failed("probe failed", err),
            Probe::NotPresent => {
                tracing::debug!("source not present");
                let events = store.record_scan(
                    scan_id,
                    source_id,
                    first_run,
                    &Observation::NotPresent,
                    &[],
                    |_| Absence::Unknown,
                )?;
                SourceOutcome::NotPresent { events }
            }
            Probe::Available { version } => match scanner.scan() {
                Err(err) => failed("scan failed", err),
                Ok(found) => {
                    let baseline = !store.load_source(source_id)?.recorded;
                    let events = store.record_scan(
                        scan_id,
                        source_id,
                        first_run,
                        &Observation::Available { version },
                        &found,
                        |tool| {
                            scanner.confirm_absent(&KnownTool {
                                identifier: &tool.identifier,
                                path: tool.path.as_deref(),
                            })
                        },
                    )?;
                    tracing::debug!(tools = found.len(), baseline, "scan recorded");
                    SourceOutcome::Recorded { tools: found.len(), baseline, events }
                }
            },
        };
        sources.push(SourceReport { source: name, outcome });
    }

    store.touch_identity()?;
    Ok(RunReport { sources })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::DiscoveredTool;
    use crate::models::ChangeEvent;
    use crate::scanner::Scanner;
    use crate::schema::{change_events, local_identity, scans, tools};
    use diesel::prelude::*;
    use std::path::PathBuf;

    #[derive(Clone, Copy)]
    enum FakeProbe {
        Available,
        NotPresent,
        Broken,
    }

    struct Fake {
        name: &'static str,
        probe: FakeProbe,
        /// `None` makes the scan fail.
        scan: Option<Vec<DiscoveredTool>>,
        absence: Absence,
    }

    impl Scanner for Fake {
        fn source_name(&self) -> &'static str {
            self.name
        }

        fn probe(&self) -> Probe {
            match self.probe {
                FakeProbe::Available => Probe::Available { version: None },
                FakeProbe::NotPresent => Probe::NotPresent,
                FakeProbe::Broken => Probe::Failed(ScanError::Io(std::io::Error::other("boom"))),
            }
        }

        fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
            self.scan
                .clone()
                .ok_or_else(|| ScanError::Io(std::io::Error::other("cannot read folder")))
        }

        fn confirm_absent(&self, _tool: &KnownTool) -> Absence {
            self.absence
        }
    }

    fn tool(identifier: &str, version: &str) -> DiscoveredTool {
        DiscoveredTool {
            identifier: identifier.to_string(),
            name: identifier.to_string(),
            version: Some(version.to_string()),
            path: PathBuf::from(format!("/apps/{identifier}")),
            installed_at: None,
        }
    }

    fn fake(name: &'static str, tools: Vec<DiscoveredTool>) -> Fake {
        Fake { name, probe: FakeProbe::Available, scan: Some(tools), absence: Absence::Gone }
    }

    fn run_with(store: &mut Store, fakes: Vec<Fake>) -> RunReport {
        let scanners: Vec<Box<dyn Scanner>> =
            fakes.into_iter().map(|f| Box::new(f) as Box<dyn Scanner>).collect();
        run(store, &ScannerRegistry::from_scanners(scanners), TriggeredBy::Manual).unwrap()
    }

    fn events(store: &mut Store) -> Vec<ChangeEvent> {
        change_events::table
            .select(ChangeEvent::as_select())
            .order(change_events::id.asc())
            .load(&mut store.conn)
            .unwrap()
    }

    fn last_scan_finished(store: &mut Store, source: &str) -> Option<String> {
        let id = store.source_id(source).unwrap();
        scans::table
            .filter(scans::source_id.eq(id))
            .order(scans::id.desc())
            .select(scans::finished_at)
            .first::<Option<String>>(&mut store.conn)
            .unwrap()
    }

    fn tool_statuses(store: &mut Store) -> Vec<String> {
        tools::table
            .select(tools::status)
            .order(tools::id.asc())
            .load::<String>(&mut store.conn)
            .unwrap()
    }

    #[test]
    fn the_first_run_baselines_every_source_without_events() {
        let mut store = Store::in_memory();
        let report = run_with(
            &mut store,
            vec![
                fake("application", vec![tool("git", "2.1")]),
                fake("homebrew-cellar", vec![tool("gh", "2.8")]),
            ],
        );

        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.events(), 0);
        assert!(report.sources.iter().all(|s| matches!(s.outcome, SourceOutcome::Recorded { baseline: true, .. })));
        assert!(report.summary().contains("baseline recorded (no events)"));
        assert_eq!(events(&mut store).len(), 0);
        assert_eq!(tool_statuses(&mut store).len(), 2);
        assert!(last_scan_finished(&mut store, "application").is_some());
    }

    #[test]
    fn a_later_run_records_installs_updates_and_uninstalls() {
        let mut store = Store::in_memory();
        run_with(&mut store, vec![fake("application", vec![tool("git", "2.1"), tool("vim", "9")])]);
        let report = run_with(
            &mut store,
            vec![fake("application", vec![tool("git", "2.2"), tool("bat", "1")])],
        );

        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.events(), 3);
        assert!(report.summary().contains("1 installed, 1 updated, 1 uninstalled"));
        let kinds: Vec<String> = events(&mut store).into_iter().map(|e| e.event_type).collect();
        assert_eq!(kinds.len(), 3);
        for expected in ["installed", "updated", "uninstalled"] {
            assert!(kinds.iter().any(|k| k == expected), "missing {expected}");
        }
    }

    #[test]
    fn a_failing_source_does_not_stop_the_others_and_records_nothing_for_itself() {
        let mut store = Store::in_memory();
        run_with(
            &mut store,
            vec![
                fake("application", vec![tool("git", "2.1")]),
                fake("homebrew-cellar", vec![tool("gh", "2.8")]),
            ],
        );

        let broken = Fake { scan: None, ..fake("application", vec![]) };
        let report = run_with(&mut store, vec![broken, fake("homebrew-cellar", vec![tool("gh", "2.9")])]);

        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.failed(), 1);
        assert!(matches!(report.sources[0].outcome, SourceOutcome::Failed { .. }));
        assert!(report.summary().contains("FAILED: scan failed"));
        // The failed source recorded nothing, and its scan never counts as a check.
        assert_eq!(last_scan_finished(&mut store, "application"), None);
        assert_eq!(tool_statuses(&mut store), vec!["installed", "installed"]);
        // The other source still went through: exactly one event, its version change.
        assert_eq!(events(&mut store).len(), 1);
        assert_eq!(events(&mut store)[0].event_type, "updated");
    }

    #[test]
    fn a_broken_probe_is_a_failure_with_nothing_recorded() {
        let mut store = Store::in_memory();
        run_with(&mut store, vec![fake("application", vec![tool("git", "2.1")])]);

        let broken = Fake { probe: FakeProbe::Broken, ..fake("application", vec![]) };
        let report = run_with(&mut store, vec![broken]);

        assert_eq!(report.exit_code(), 1);
        assert!(report.summary().contains("probe failed"));
        assert_eq!(last_scan_finished(&mut store, "application"), None);
        assert_eq!(events(&mut store).len(), 0);
    }

    #[test]
    fn a_source_that_disappears_is_recorded_without_touching_its_tools() {
        let mut store = Store::in_memory();
        run_with(&mut store, vec![fake("application", vec![tool("git", "2.1")])]);

        let gone = Fake { probe: FakeProbe::NotPresent, scan: Some(vec![]), ..fake("application", vec![]) };
        let report = run_with(&mut store, vec![gone]);

        assert_eq!(report.exit_code(), 0);
        assert!(matches!(&report.sources[0].outcome, SourceOutcome::NotPresent { events } if events.uninstalled == 1));
        assert!(report.summary().contains("not present   1 uninstalled"));
        assert_eq!(tool_statuses(&mut store), vec!["installed"]);
    }

    #[test]
    fn a_tool_that_is_still_there_is_left_alone() {
        let mut store = Store::in_memory();
        run_with(&mut store, vec![fake("application", vec![tool("git", "2.1")])]);

        let cautious = Fake { absence: Absence::StillThere, ..fake("application", vec![]) };
        let report = run_with(&mut store, vec![cautious]);

        assert_eq!(report.events(), 0);
        assert_eq!(tool_statuses(&mut store), vec!["installed"]);
    }

    #[test]
    fn the_run_updates_the_installations_heartbeat() {
        let mut store = Store::in_memory();
        diesel::sql_query("INSERT INTO local_identity (platform_uuid) VALUES ('test-machine')")
            .execute(&mut store.conn)
            .unwrap();
        let before: Option<String> = local_identity::table
            .select(local_identity::last_seen_at)
            .first(&mut store.conn)
            .unwrap();
        assert_eq!(before, None);

        run_with(&mut store, vec![fake("application", vec![])]);

        let after: Option<String> = local_identity::table
            .select(local_identity::last_seen_at)
            .first(&mut store.conn)
            .unwrap();
        assert!(after.is_some());
    }

    #[test]
    fn the_summary_reads_cleanly_for_each_outcome() {
        let report = RunReport {
            sources: vec![
                SourceReport {
                    source: "application",
                    outcome: SourceOutcome::Recorded { tools: 39, baseline: false, events: ApplyReport::default() },
                },
                SourceReport {
                    source: "homebrew-cellar",
                    outcome: SourceOutcome::Failed { reason: "scan failed: permission denied".into() },
                },
            ],
        };

        assert_eq!(
            report.summary(),
            "application      39 tools   no changes\n\
             homebrew-cellar  FAILED: scan failed: permission denied\n\
             2 sources checked, 1 failed, 0 events"
        );
    }
}

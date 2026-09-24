//! Writing a plan: everything one source scan decided, saved in a single write transaction.
//! If anything fails, nothing is saved and the scan's `finished_at` stays empty.

use super::{attributes_json, Store};
use crate::domain::plan::{PlannedEvent, SourcePlan, ToolAction, ToolPlan};
use crate::domain::time::utc_string;
use crate::domain::{DiscoveredTool, EventType, Status};
use crate::models::{NewChangeEvent, NewTool};
use crate::schema::{change_events, scans, sources, tools};
use diesel::prelude::*;
use diesel::result::Error as DbError;
use diesel::sqlite::SqliteConnection;
use std::time::{SystemTime, UNIX_EPOCH};

/// How many events a scan recorded, by type.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ApplyReport {
    pub installed: u32,
    pub updated: u32,
    pub uninstalled: u32,
}

fn now_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or(0);
    utc_string(seconds)
}

impl Store {
    /// Saves one source scan. `tools` is `None` when the source wasn't scanned (it is not present).
    pub fn apply_scan(
        &mut self,
        scan_id: i32,
        source_id: i32,
        source: &SourcePlan,
        tools: Option<&ToolPlan>,
    ) -> QueryResult<ApplyReport> {
        self.conn.immediate_transaction(|conn| {
            let mut report = ApplyReport::default();

            if let Some(plan) = tools {
                for action in &plan.actions {
                    apply_tool_action(conn, scan_id, source_id, action, &mut report)?;
                }
            }
            apply_source_plan(conn, scan_id, source_id, source, &mut report)?;

            diesel::update(scans::table.filter(scans::id.eq(scan_id)))
                .set(scans::finished_at.eq(Some(now_utc())))
                .execute(conn)?;
            Ok(report)
        })
    }
}

fn apply_tool_action(
    conn: &mut SqliteConnection,
    scan_id: i32,
    source_id: i32,
    action: &ToolAction,
    report: &mut ApplyReport,
) -> QueryResult<()> {
    let (tool_id, event) = match action {
        ToolAction::Insert { tool, event } => {
            let attributes = attributes_json(tool);
            let new_tool = NewTool {
                name: &tool.name,
                source_id,
                attributes: Some(&attributes),
                identifier: &tool.identifier,
            };
            let id = diesel::insert_into(tools::table)
                .values(&new_tool)
                .returning(tools::id)
                .get_result::<Option<i32>>(conn)?
                .ok_or(DbError::NotFound)?;
            set_installed_at(conn, id, tool)?;
            (id, event)
        }
        ToolAction::MarkReinstalled { id, tool, event } => {
            update_details(conn, *id, tool)?;
            set_status(conn, *id, Status::Installed)?;
            (*id, event)
        }
        ToolAction::UpdateVersion { id, tool, event } => {
            update_details(conn, *id, tool)?;
            (*id, event)
        }
        ToolAction::Refresh { id, tool } => {
            update_details(conn, *id, tool)?;
            return Ok(());
        }
        ToolAction::MarkUninstalled { id, event, .. } => {
            set_status(conn, *id, Status::Uninstalled)?;
            (*id, event)
        }
    };

    if let Some(event) = event {
        insert_event(conn, scan_id, Some(tool_id), event, report)?;
    }
    Ok(())
}

fn update_details(conn: &mut SqliteConnection, id: i32, tool: &DiscoveredTool) -> QueryResult<()> {
    diesel::update(tools::table.filter(tools::id.eq(id)))
        .set((
            tools::name.eq(&tool.name),
            tools::attributes.eq(Some(attributes_json(tool))),
        ))
        .execute(conn)?;
    set_installed_at(conn, id, tool)
}

/// Only overwrites when the source reports a real install date.
fn set_installed_at(conn: &mut SqliteConnection, id: i32, tool: &DiscoveredTool) -> QueryResult<()> {
    if let Some(seconds) = tool.installed_at {
        diesel::update(tools::table.filter(tools::id.eq(id)))
            .set(tools::installed_at.eq(Some(utc_string(seconds))))
            .execute(conn)?;
    }
    Ok(())
}

fn set_status(conn: &mut SqliteConnection, id: i32, status: Status) -> QueryResult<()> {
    diesel::update(tools::table.filter(tools::id.eq(id)))
        .set(tools::status.eq(status.as_str()))
        .execute(conn)?;
    Ok(())
}

fn apply_source_plan(
    conn: &mut SqliteConnection,
    scan_id: i32,
    source_id: i32,
    plan: &SourcePlan,
    report: &mut ApplyReport,
) -> QueryResult<()> {
    if let Some(state) = &plan.change {
        diesel::update(sources::table.filter(sources::id.eq(source_id)))
            .set((
                sources::status.eq(Some(state.status.as_str())),
                sources::version.eq(state.version.as_deref()),
            ))
            .execute(conn)?;
    }
    if plan.record_first_seen {
        diesel::update(sources::table.filter(sources::id.eq(source_id)))
            .set(sources::first_seen_at.eq(Some(now_utc())))
            .execute(conn)?;
    }
    if let Some(event) = &plan.event {
        insert_event(conn, scan_id, None, event, report)?;
    }
    Ok(())
}

fn insert_event(
    conn: &mut SqliteConnection,
    scan_id: i32,
    tool_id: Option<i32>,
    event: &PlannedEvent,
    report: &mut ApplyReport,
) -> QueryResult<()> {
    let changes = event.changes.to_json();
    diesel::insert_into(change_events::table)
        .values(NewChangeEvent {
            scan_id,
            tool_id,
            event_type: event.event_type.as_str(),
            changes: &changes,
        })
        .execute(conn)?;

    match event.event_type {
        EventType::Installed => report.installed += 1,
        EventType::Updated => report.updated += 1,
        EventType::Uninstalled => report.uninstalled += 1,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::plan::{plan_source, plan_tools, Observation};
    use crate::domain::{Absence, Changes, TriggeredBy};
    use crate::models::{ChangeEvent, Scan, Tool};
    use std::path::PathBuf;

    const APP: &str = "application";
    const BREW: &str = "homebrew-cellar";

    fn found(identifier: &str, version: Option<&str>) -> DiscoveredTool {
        DiscoveredTool {
            identifier: identifier.to_string(),
            name: identifier.to_string(),
            version: version.map(String::from),
            path: PathBuf::from(format!("/apps/{identifier}")),
            installed_at: None,
        }
    }

    fn available() -> Observation {
        Observation::Available { version: None }
    }

    /// What `run` does for one source, minus the scanner.
    fn scan_source(
        store: &mut Store,
        source: &str,
        observed: Observation,
        found: &[DiscoveredTool],
        absence: Absence,
    ) -> ApplyReport {
        let first_run = store.is_first_run().unwrap();
        let source_id = store.source_id(source).unwrap();
        let run_id = store.new_run_id().unwrap();
        let scan_id = store.begin_scan(&run_id, source_id, &TriggeredBy::Manual).unwrap();
        let stored_source = store.load_source(source_id).unwrap();
        let source_plan = plan_source(&stored_source, &observed, first_run);
        let tool_plan = match observed {
            Observation::Available { .. } => {
                let stored = store.load_tools(source_id).unwrap();
                Some(plan_tools(&stored, found, !stored_source.recorded, |_| absence))
            }
            Observation::NotPresent => None,
        };
        store.apply_scan(scan_id, source_id, &source_plan, tool_plan.as_ref()).unwrap()
    }

    fn events(store: &mut Store) -> Vec<ChangeEvent> {
        change_events::table
            .select(ChangeEvent::as_select())
            .order(change_events::id.asc())
            .load(&mut store.conn)
            .unwrap()
    }

    /// (event type, is it about a tool?, changes JSON) per event, oldest first.
    fn summary(store: &mut Store) -> Vec<(String, bool, String)> {
        events(store)
            .into_iter()
            .map(|e| (e.event_type, e.tool_id.is_some(), e.changes))
            .collect()
    }

    fn tool_rows(store: &mut Store) -> Vec<Tool> {
        tools::table
            .select(Tool::as_select())
            .order(tools::id.asc())
            .load(&mut store.conn)
            .unwrap()
    }

    fn scan_rows(store: &mut Store) -> Vec<Scan> {
        scans::table
            .select(Scan::as_select())
            .order(scans::id.asc())
            .load(&mut store.conn)
            .unwrap()
    }

    #[test]
    fn the_first_scan_records_state_and_no_events() {
        let mut store = Store::in_memory();
        assert!(store.is_first_run().unwrap());

        scan_source(&mut store, APP, available(), &[found("git", Some("2.1")), found("gh", Some("2.8"))], Absence::Gone);

        assert_eq!(summary(&mut store), vec![]);
        let rows = tool_rows(&mut store);
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|t| t.status == "installed"));
        assert!(!store.is_first_run().unwrap());
        let scans = scan_rows(&mut store);
        assert_eq!(scans.len(), 1);
        assert!(scans[0].finished_at.is_some());
        let id = store.source_id(APP).unwrap();
        let source = store.load_source(id).unwrap();
        assert!(source.recorded);
        assert_eq!(source.status, Some(Status::Installed));
    }

    #[test]
    fn a_second_identical_scan_records_nothing() {
        let mut store = Store::in_memory();
        let tools = [found("git", Some("2.1"))];
        scan_source(&mut store, APP, available(), &tools, Absence::Gone);
        let report = scan_source(&mut store, APP, available(), &tools, Absence::Gone);

        assert_eq!(report, ApplyReport::default());
        assert_eq!(summary(&mut store), vec![]);
        assert_eq!(tool_rows(&mut store).len(), 1);
        assert_eq!(scan_rows(&mut store).len(), 2);
    }

    #[test]
    fn a_tool_installed_after_the_baseline_is_an_installed_event() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1")), found("bat", Some("0.24"))], Absence::Gone);

        assert_eq!(
            summary(&mut store),
            vec![("installed".to_string(), true, r#"{"version":[null,"0.24"]}"#.to_string())]
        );
        assert_eq!(tool_rows(&mut store).len(), 2);
    }

    #[test]
    fn a_version_change_records_updated_and_stores_the_new_version() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, APP, available(), &[found("git", Some("2.2"))], Absence::Gone);

        assert_eq!(
            summary(&mut store),
            vec![("updated".to_string(), true, r#"{"version":["2.1","2.2"]}"#.to_string())]
        );
        let rows = tool_rows(&mut store);
        assert!(rows[0].attributes.as_deref().unwrap().contains("2.2"));
    }

    #[test]
    fn an_uninstall_is_recorded_once_and_then_stays_quiet() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, APP, available(), &[], Absence::Gone);
        scan_source(&mut store, APP, available(), &[], Absence::Gone);

        assert_eq!(
            summary(&mut store),
            vec![("uninstalled".to_string(), true, r#"{"version":["2.1",null]}"#.to_string())]
        );
        assert_eq!(tool_rows(&mut store)[0].status, "uninstalled");
    }

    #[test]
    fn a_missing_tool_that_is_still_there_is_left_alone() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, APP, available(), &[], Absence::StillThere);

        assert_eq!(summary(&mut store), vec![]);
        assert_eq!(tool_rows(&mut store)[0].status, "installed");
    }

    #[test]
    fn a_returning_tool_reuses_its_row_and_keeps_its_first_seen_date() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        let before = tool_rows(&mut store).remove(0);
        scan_source(&mut store, APP, available(), &[], Absence::Gone);
        scan_source(&mut store, APP, available(), &[found("git", Some("2.3"))], Absence::Gone);

        let rows = tool_rows(&mut store);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, before.id);
        assert_eq!(rows[0].first_seen_at, before.first_seen_at);
        assert_eq!(rows[0].status, "installed");
        let kinds: Vec<String> = summary(&mut store).into_iter().map(|(kind, _, _)| kind).collect();
        assert_eq!(kinds, vec!["uninstalled", "installed"]);
    }

    #[test]
    fn a_real_install_date_from_the_source_is_stored_as_utc_text() {
        let mut store = Store::in_memory();
        let mut git = found("git", Some("2.1"));
        git.installed_at = Some(1_775_808_467);
        scan_source(&mut store, BREW, available(), &[git], Absence::Gone);

        assert_eq!(tool_rows(&mut store)[0].installed_at.as_deref(), Some("2026-04-10 08:07:47"));
    }

    #[test]
    fn a_failed_scan_leaves_no_trace_and_the_next_scan_is_still_the_baseline() {
        let mut store = Store::in_memory();
        let source_id = store.source_id(APP).unwrap();
        let run_id = store.new_run_id().unwrap();
        store.begin_scan(&run_id, source_id, &TriggeredBy::Manual).unwrap(); // scanner failed: never applied

        assert!(scan_rows(&mut store)[0].finished_at.is_none());
        assert!(store.is_first_run().unwrap());

        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        assert_eq!(summary(&mut store), vec![]);
    }

    #[test]
    fn a_failing_write_rolls_everything_back() {
        let mut store = Store::in_memory();
        let source_id = store.source_id(APP).unwrap();
        let run_id = store.new_run_id().unwrap();
        let scan_id = store.begin_scan(&run_id, source_id, &TriggeredBy::Manual).unwrap();

        let insert = || ToolAction::Insert {
            tool: found("git", Some("2.1")),
            event: Some(PlannedEvent {
                event_type: EventType::Installed,
                changes: Changes::installed(Some("2.1")),
            }),
        };
        // The second insert breaks the unique (source, identifier) rule.
        let plan = ToolPlan { actions: vec![insert(), insert()], kept: vec![] };
        let source_plan = plan_source(&store.load_source(source_id).unwrap(), &available(), false);

        assert!(store.apply_scan(scan_id, source_id, &source_plan, Some(&plan)).is_err());

        assert_eq!(tool_rows(&mut store).len(), 0);
        assert_eq!(summary(&mut store), vec![]);
        assert!(scan_rows(&mut store)[0].finished_at.is_none());
        assert!(!store.load_source(source_id).unwrap().recorded);
    }

    #[test]
    fn a_source_that_vanishes_and_returns_gets_events_without_touching_its_tools() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, APP, Observation::NotPresent, &[], Absence::Gone);

        let id = store.source_id(APP).unwrap();
        assert_eq!(store.load_source(id).unwrap().status, Some(Status::Uninstalled));
        assert_eq!(tool_rows(&mut store)[0].status, "installed");

        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);

        assert_eq!(
            summary(&mut store),
            vec![
                ("uninstalled".to_string(), false, r#"{"version":[null,null]}"#.to_string()),
                ("installed".to_string(), false, r#"{"version":[null,null]}"#.to_string()),
            ]
        );
        assert_eq!(store.load_source(id).unwrap().status, Some(Status::Installed));
        assert!(scan_rows(&mut store).iter().all(|s| s.finished_at.is_some()));
    }

    #[test]
    fn a_source_that_appears_later_gets_an_event_but_its_tools_are_baselined() {
        let mut store = Store::in_memory();
        scan_source(&mut store, APP, available(), &[found("git", Some("2.1"))], Absence::Gone);
        scan_source(&mut store, BREW, available(), &[found("gh", Some("2.8"))], Absence::Gone);

        assert_eq!(
            summary(&mut store),
            vec![("installed".to_string(), false, r#"{"version":[null,null]}"#.to_string())]
        );
        assert_eq!(tool_rows(&mut store).len(), 2);
    }

    #[test]
    fn the_database_enforces_foreign_keys_like_the_shipped_app() {
        let mut store = Store::in_memory();
        let run_id = store.new_run_id().unwrap();
        assert!(store.begin_scan(&run_id, 9_999, &TriggeredBy::Manual).is_err());
    }

    #[test]
    fn run_ids_are_distinct_32_character_hex() {
        let mut store = Store::in_memory();
        let (a, b) = (store.new_run_id().unwrap(), store.new_run_id().unwrap());
        assert_ne!(a, b);
        assert_eq!(a.len(), 32);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

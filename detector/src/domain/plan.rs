//! The decision step: given what is on record for one source and what a scan just found,
//! decide what to write. Pure: no database, no filesystem. The caller supplies the
//! "is it really gone?" check as a function.

use super::{Absence, Changes, DiscoveredTool, EventType, Status};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A tool already on record for the source.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredTool {
    pub id: i32,
    pub identifier: String,
    pub version: Option<String>,
    pub path: Option<PathBuf>,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlannedEvent {
    pub event_type: EventType,
    pub changes: Changes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolAction {
    /// Never seen before.
    Insert { tool: DiscoveredTool, event: Option<PlannedEvent> },
    /// Was uninstalled, found again.
    MarkInstalled { id: i32, tool: DiscoveredTool, event: Option<PlannedEvent> },
    /// Still installed, version differs.
    UpdateVersion { id: i32, tool: DiscoveredTool, event: Option<PlannedEvent> },
    /// Still installed, nothing worth an event; refresh stored details.
    Refresh { id: i32, tool: DiscoveredTool },
    /// Confirmed gone.
    MarkUninstalled { id: i32, identifier: String, event: Option<PlannedEvent> },
}

impl ToolAction {
    pub fn identifier(&self) -> &str {
        match self {
            ToolAction::Insert { tool, .. }
            | ToolAction::MarkInstalled { tool, .. }
            | ToolAction::UpdateVersion { tool, .. }
            | ToolAction::Refresh { tool, .. } => &tool.identifier,
            ToolAction::MarkUninstalled { identifier, .. } => identifier,
        }
    }

    pub fn event(&self) -> Option<&PlannedEvent> {
        match self {
            ToolAction::Insert { event, .. }
            | ToolAction::MarkInstalled { event, .. }
            | ToolAction::UpdateVersion { event, .. }
            | ToolAction::MarkUninstalled { event, .. } => event.as_ref(),
            ToolAction::Refresh { .. } => None,
        }
    }

    fn without_event(self) -> Self {
        match self {
            ToolAction::Insert { tool, .. } => ToolAction::Insert { tool, event: None },
            ToolAction::MarkInstalled { id, tool, .. } => {
                ToolAction::MarkInstalled { id, tool, event: None }
            }
            ToolAction::UpdateVersion { id, tool, .. } => {
                ToolAction::UpdateVersion { id, tool, event: None }
            }
            ToolAction::MarkUninstalled { id, identifier, .. } => {
                ToolAction::MarkUninstalled { id, identifier, event: None }
            }
            refresh @ ToolAction::Refresh { .. } => refresh,
        }
    }
}

/// A tool that is on record but was not found, and we could not confirm it is gone.
/// Nothing is recorded for it; the caller can log it.
#[derive(Debug, Clone, PartialEq)]
pub struct KeptTool {
    pub identifier: String,
    pub absence: Absence,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolPlan {
    pub actions: Vec<ToolAction>,
    pub kept: Vec<KeptTool>,
}

impl ToolPlan {
    pub fn events(&self) -> impl Iterator<Item = &PlannedEvent> {
        self.actions.iter().filter_map(ToolAction::event)
    }
}

/// `baseline` is true when the source has never been scanned successfully before:
/// state is recorded, but no events (events describe only changes Legacy witnessed).
pub fn plan_tools(
    stored: &[StoredTool],
    found: &[DiscoveredTool],
    baseline: bool,
    confirm_absent: impl Fn(&StoredTool) -> Absence,
) -> ToolPlan {
    let stored_by_identifier: BTreeMap<&str, &StoredTool> = stored
        .iter()
        .map(|tool| (tool.identifier.as_str(), tool))
        .collect();
    let found_by_identifier = dedupe(found);

    let mut plan = ToolPlan::default();

    for (identifier, tool) in &found_by_identifier {
        let tool = (*tool).clone();
        let action = match stored_by_identifier.get(identifier.as_str()) {
            None => ToolAction::Insert {
                event: Some(installed_event(&tool)),
                tool,
            },
            Some(existing) if existing.status == Status::Uninstalled => ToolAction::MarkInstalled {
                id: existing.id,
                event: Some(installed_event(&tool)),
                tool,
            },
            Some(existing) if existing.version != tool.version => ToolAction::UpdateVersion {
                id: existing.id,
                event: Some(PlannedEvent {
                    event_type: EventType::Updated,
                    changes: Changes::updated(existing.version.as_deref(), tool.version.as_deref()),
                }),
                tool,
            },
            Some(existing) => ToolAction::Refresh { id: existing.id, tool },
        };
        plan.actions.push(action);
    }

    for (identifier, existing) in &stored_by_identifier {
        // Already uninstalled and still missing: nothing new happened, so record nothing.
        if found_by_identifier.contains_key(*identifier) || existing.status == Status::Uninstalled {
            continue;
        }
        match confirm_absent(existing) {
            Absence::Gone => plan.actions.push(ToolAction::MarkUninstalled {
                id: existing.id,
                identifier: existing.identifier.clone(),
                event: Some(PlannedEvent {
                    event_type: EventType::Uninstalled,
                    changes: Changes::uninstalled(existing.version.as_deref()),
                }),
            }),
            absence => plan.kept.push(KeptTool {
                identifier: existing.identifier.clone(),
                absence,
            }),
        }
    }

    if baseline {
        plan.actions = plan.actions.into_iter().map(ToolAction::without_event).collect();
    }
    plan.actions.sort_by(|a, b| a.identifier().cmp(b.identifier()));
    plan
}

fn installed_event(tool: &DiscoveredTool) -> PlannedEvent {
    PlannedEvent {
        event_type: EventType::Installed,
        changes: Changes::installed(tool.version.as_deref()),
    }
}

/// One record per identifier. When a scan returns the same identifier twice (for example a
/// formula with two version folders after an upgrade), the newest install wins, and a tie goes
/// to the greater path. The choice never depends on the order the scan listed things in.
fn dedupe(found: &[DiscoveredTool]) -> BTreeMap<String, &DiscoveredTool> {
    let mut chosen: BTreeMap<String, &DiscoveredTool> = BTreeMap::new();
    for tool in found {
        match chosen.get(&tool.identifier) {
            Some(current)
                if (tool.installed_at, &tool.path) <= (current.installed_at, &current.path) => {}
            _ => {
                chosen.insert(tool.identifier.clone(), tool);
            }
        }
    }
    chosen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(identifier: &str, version: Option<&str>) -> DiscoveredTool {
        DiscoveredTool {
            identifier: identifier.to_string(),
            name: identifier.to_string(),
            version: version.map(String::from),
            path: PathBuf::from(format!("/apps/{identifier}")),
            installed_at: None,
        }
    }

    fn stored(id: i32, identifier: &str, version: Option<&str>, status: Status) -> StoredTool {
        StoredTool {
            id,
            identifier: identifier.to_string(),
            version: version.map(String::from),
            path: Some(PathBuf::from(format!("/apps/{identifier}"))),
            status,
        }
    }

    fn gone(_: &StoredTool) -> Absence {
        Absence::Gone
    }

    fn events(plan: &ToolPlan) -> Vec<(EventType, String)> {
        plan.events()
            .map(|event| (event.event_type, event.changes.to_json()))
            .collect()
    }

    #[test]
    fn a_new_tool_is_inserted_with_an_installed_event() {
        let plan = plan_tools(&[], &[found("git", Some("2.1"))], false, gone);

        assert!(matches!(plan.actions[0], ToolAction::Insert { .. }));
        assert_eq!(
            events(&plan),
            vec![(EventType::Installed, r#"{"version":[null,"2.1"]}"#.to_string())]
        );
    }

    #[test]
    fn the_baseline_records_state_but_no_events() {
        let plan = plan_tools(
            &[],
            &[found("git", Some("2.1")), found("gh", Some("2.8"))],
            true,
            gone,
        );

        assert_eq!(plan.actions.len(), 2);
        assert!(plan.actions.iter().all(|a| matches!(a, ToolAction::Insert { event: None, .. })));
        assert_eq!(events(&plan), vec![]);
    }

    #[test]
    fn an_unchanged_tool_is_only_refreshed_and_records_nothing() {
        let stored = [stored(1, "git", Some("2.1"), Status::Installed)];
        let plan = plan_tools(&stored, &[found("git", Some("2.1"))], false, gone);

        assert!(matches!(plan.actions[0], ToolAction::Refresh { id: 1, .. }));
        assert_eq!(events(&plan), vec![]);
    }

    #[test]
    fn a_version_change_is_an_updated_event_with_old_and_new() {
        let stored = [stored(1, "git", Some("2.1"), Status::Installed)];
        let plan = plan_tools(&stored, &[found("git", Some("2.2"))], false, gone);

        assert!(matches!(plan.actions[0], ToolAction::UpdateVersion { id: 1, .. }));
        assert_eq!(
            events(&plan),
            vec![(EventType::Updated, r#"{"version":["2.1","2.2"]}"#.to_string())]
        );
    }

    #[test]
    fn an_unknown_version_staying_unknown_is_not_a_change() {
        let stored = [stored(1, "git", None, Status::Installed)];
        let plan = plan_tools(&stored, &[found("git", None)], false, gone);

        assert_eq!(events(&plan), vec![]);
    }

    #[test]
    fn a_missing_tool_confirmed_gone_is_uninstalled_with_its_last_version() {
        let stored = [stored(1, "git", Some("2.1"), Status::Installed)];
        let plan = plan_tools(&stored, &[], false, gone);

        assert!(matches!(plan.actions[0], ToolAction::MarkUninstalled { id: 1, .. }));
        assert_eq!(
            events(&plan),
            vec![(EventType::Uninstalled, r#"{"version":["2.1",null]}"#.to_string())]
        );
    }

    #[test]
    fn a_missing_tool_that_is_still_there_or_unknown_records_nothing() {
        let stored = [stored(1, "git", Some("2.1"), Status::Installed)];

        let still_there = plan_tools(&stored, &[], false, |_| Absence::StillThere);
        assert_eq!(still_there.actions, vec![]);
        assert_eq!(
            still_there.kept,
            vec![KeptTool { identifier: "git".into(), absence: Absence::StillThere }]
        );

        let unknown = plan_tools(&stored, &[], false, |_| Absence::Unknown);
        assert_eq!(unknown.actions, vec![]);
        assert_eq!(unknown.kept[0].absence, Absence::Unknown);
    }

    #[test]
    fn an_already_uninstalled_tool_that_stays_missing_is_not_reported_again() {
        let stored = [stored(1, "git", Some("2.1"), Status::Uninstalled)];
        let plan = plan_tools(&stored, &[], false, gone);

        assert_eq!(plan.actions, vec![]);
        assert_eq!(plan.kept, vec![]);
    }

    #[test]
    fn an_uninstalled_tool_that_comes_back_is_installed_again() {
        let stored = [stored(1, "git", Some("2.1"), Status::Uninstalled)];
        let plan = plan_tools(&stored, &[found("git", Some("2.3"))], false, gone);

        assert!(matches!(plan.actions[0], ToolAction::MarkInstalled { id: 1, .. }));
        assert_eq!(
            events(&plan),
            vec![(EventType::Installed, r#"{"version":[null,"2.3"]}"#.to_string())]
        );
    }

    #[test]
    fn the_baseline_still_corrects_state_but_records_no_events() {
        let stored = [stored(1, "git", Some("2.1"), Status::Installed)];
        let plan = plan_tools(&stored, &[], true, gone);

        assert!(matches!(
            plan.actions[0],
            ToolAction::MarkUninstalled { id: 1, event: None, .. }
        ));
        assert_eq!(events(&plan), vec![]);
    }

    #[test]
    fn duplicate_identifiers_keep_the_newest_install_regardless_of_order() {
        let mut old = found("gh", Some("2.87"));
        old.installed_at = Some(100);
        let mut new = found("gh", Some("2.88"));
        new.installed_at = Some(200);

        for input in [vec![old.clone(), new.clone()], vec![new.clone(), old.clone()]] {
            let plan = plan_tools(&[], &input, false, gone);
            assert_eq!(plan.actions.len(), 1);
            match &plan.actions[0] {
                ToolAction::Insert { tool, .. } => assert_eq!(tool.version.as_deref(), Some("2.88")),
                other => panic!("unexpected action {other:?}"),
            }
        }
    }

    #[test]
    fn duplicate_identifiers_with_no_dates_break_ties_by_path_deterministically() {
        let mut a = found("gh", Some("2.87"));
        a.path = PathBuf::from("/cellar/gh/2.87");
        let mut b = found("gh", Some("2.88"));
        b.path = PathBuf::from("/cellar/gh/2.88");

        for input in [vec![a.clone(), b.clone()], vec![b.clone(), a.clone()]] {
            let plan = plan_tools(&[], &input, false, gone);
            match &plan.actions[0] {
                ToolAction::Insert { tool, .. } => assert_eq!(tool.version.as_deref(), Some("2.88")),
                other => panic!("unexpected action {other:?}"),
            }
        }
    }

    #[test]
    fn actions_come_out_sorted_by_identifier() {
        let plan = plan_tools(
            &[],
            &[found("zsh", Some("5")), found("bat", Some("1")), found("git", Some("2"))],
            false,
            gone,
        );
        let order: Vec<&str> = plan.actions.iter().map(ToolAction::identifier).collect();
        assert_eq!(order, vec!["bat", "git", "zsh"]);
    }

    #[test]
    fn a_scan_after_applying_a_plan_would_record_nothing() {
        // What the store holds after applying the first plan.
        let stored = [
            stored(1, "git", Some("2.1"), Status::Installed),
            stored(2, "gh", Some("2.8"), Status::Installed),
        ];
        let plan = plan_tools(
            &stored,
            &[found("git", Some("2.1")), found("gh", Some("2.8"))],
            false,
            gone,
        );
        assert_eq!(events(&plan), vec![]);
        assert!(plan.actions.iter().all(|a| matches!(a, ToolAction::Refresh { .. })));
    }
}

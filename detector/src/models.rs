use crate::schema::{change_events, local_identity, scans, sources, tools};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    #[diesel(column_name = type_)]
    pub source_type: String,
    pub version: Option<String>,
    pub first_seen_at: Option<String>,
    pub installed_at: Option<String>,
    pub baselined_at: Option<String>,
    pub status: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = tools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tool {
    pub id: Option<i32>,
    pub name: String,
    pub source_id: i32,
    pub attributes: Option<String>,
    pub identifier: String,
    pub first_seen_at: String,
    pub installed_at: Option<String>,
    pub status: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tools)]
pub struct NewTool<'a> {
    pub name: &'a str,
    pub source_id: i32,
    pub attributes: Option<&'a str>,
    pub identifier: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = scans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Scan {
    pub id: Option<i32>,
    pub run_id: String,
    pub source_id: i32,
    pub triggered_by: String,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = scans)]
pub struct NewScan<'a> {
    pub run_id: &'a str,
    pub source_id: i32,
    pub triggered_by: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = change_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChangeEvent {
    pub id: Option<i32>,
    pub scan_id: i32,
    pub tool_id: Option<i32>,
    pub event_type: String,
    pub changes: String,
    pub occurred_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = change_events)]
pub struct NewChangeEvent<'a> {
    pub scan_id: i32,
    pub tool_id: Option<i32>,
    pub event_type: &'a str,
    pub changes: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = local_identity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LocalIdentity {
    pub id: Option<i32>,
    pub platform_uuid: String,
    pub device_name: Option<String>,
    pub username: Option<String>,
    pub attributes: Option<String>,
    pub first_seen_at: String,
    pub last_seen_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = local_identity)]
pub struct NewLocalIdentity<'a> {
    pub platform_uuid: &'a str,
    pub device_name: Option<&'a str>,
    pub username: Option<&'a str>,
    pub attributes: Option<&'a str>,
}

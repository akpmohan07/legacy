pub mod apply;

use crate::domain::plan::{StoredSource, StoredTool};
use crate::domain::{DiscoveredTool, Status, TriggeredBy};
use crate::identity;
use crate::models::{NewLocalIdentity, NewScan, NewTool, Source, Tool};
use crate::schema::{scans, sources, tools};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::result::Error as DbError;
use diesel::sql_types::Text;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Extra fields that don't have their own `tools` column,
/// serialized into `tools.attributes` as JSON.
#[derive(Serialize)]
struct ToolAttributes<'a> {
    version: Option<&'a str>,
    path: &'a str,
}

/// The same fields read back; tolerant of missing or malformed JSON.
#[derive(Deserialize, Default)]
struct StoredAttributes {
    version: Option<String>,
    path: Option<String>,
}

pub(crate) fn attributes_json(tool: &DiscoveredTool) -> String {
    serde_json::to_string(&ToolAttributes {
        version: tool.version.as_deref(),
        path: tool.path.to_str().unwrap_or(""),
    })
    .expect("failed to serialize tool attributes")
}

fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("could not resolve platform app-data directory")
        .join("Legacy")
}

/// A second writer waits (up to 5 seconds) for the first to finish instead of failing at once.
fn configure(conn: &mut SqliteConnection) {
    conn.batch_execute("PRAGMA busy_timeout = 5000;")
        .expect("failed to set busy_timeout");
}

pub fn open_store() -> SqliteConnection {
    let dir = app_data_dir();
    std::fs::create_dir_all(&dir).expect("failed to create app-data directory");
    let db_path = dir.join("legacy.db");

    let mut conn = SqliteConnection::establish(db_path.to_str().unwrap())
        .unwrap_or_else(|err| panic!("failed to open {:?}: {}", db_path, err));

    configure(&mut conn);
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");

    ensure_local_identity(&mut conn);

    conn
}

/// Populates `local_identity` once, on first run only. Never re-resolved
/// or upserted afterward — nothing about it changes scan to scan.
fn ensure_local_identity(conn: &mut SqliteConnection) {
    use crate::schema::local_identity;

    let count: i64 = local_identity::table
        .count()
        .get_result(conn)
        .expect("failed to query local_identity");

    if count > 0 {
        return;
    }

    let resolved = identity::resolve();
    let new_identity = NewLocalIdentity {
        platform_uuid: &resolved.platform_uuid,
        device_name: resolved.device_name.as_deref(),
        username: resolved.username.as_deref(),
        attributes: Some(&resolved.attributes_json),
    };

    diesel::insert_into(local_identity::table)
        .values(&new_identity)
        .execute(conn)
        .expect("failed to insert local_identity");
}

pub fn upsert_tool(conn: &mut SqliteConnection, source_name: &str, tool: &DiscoveredTool) {
    let source_id = sources::table
        .filter(sources::name.eq(source_name))
        .select(sources::id)
        .first::<Option<i32>>(conn)
        .unwrap_or_else(|_| panic!("unknown source: {source_name} (not seeded?)"))
        .expect("sources.id is never actually null once a row exists");

    let attributes = attributes_json(tool);

    let new_tool = NewTool {
        name: &tool.name,
        source_id,
        attributes: Some(&attributes),
        identifier: &tool.identifier,
    };

    diesel::insert_into(tools::table)
        .values(&new_tool)
        .on_conflict((tools::source_id, tools::identifier))
        .do_update()
        .set(&new_tool)
        .execute(conn)
        .expect("failed to upsert tool");
}

fn corrupt(message: String) -> DbError {
    DbError::DeserializationError(message.into())
}

fn read_first_run(conn: &mut SqliteConnection) -> QueryResult<bool> {
    let recorded: i64 = sources::table
        .filter(sources::first_seen_at.is_not_null())
        .count()
        .get_result(conn)?;
    Ok(recorded == 0)
}

pub(crate) fn read_source(conn: &mut SqliteConnection, source_id: i32) -> QueryResult<StoredSource> {
    let row: Source = sources::table
        .filter(sources::id.eq(source_id))
        .select(Source::as_select())
        .first(conn)?;

    let status = match row.status.as_deref() {
        None => None,
        Some(raw) => Some(
            Status::parse(raw).ok_or_else(|| corrupt(format!("unknown source status {raw:?}")))?,
        ),
    };
    Ok(StoredSource {
        status,
        version: row.version,
        recorded: row.first_seen_at.is_some(),
    })
}

pub(crate) fn read_tools(conn: &mut SqliteConnection, source_id: i32) -> QueryResult<Vec<StoredTool>> {
    let rows: Vec<Tool> = tools::table
        .filter(tools::source_id.eq(source_id))
        .select(Tool::as_select())
        .load(conn)?;

    rows.into_iter()
        .map(|row| {
            let status = Status::parse(&row.status)
                .ok_or_else(|| corrupt(format!("unknown tool status {:?}", row.status)))?;
            let attributes: StoredAttributes = row
                .attributes
                .as_deref()
                .and_then(|raw| serde_json::from_str(raw).ok())
                .unwrap_or_default();
            Ok(StoredTool {
                id: row.id.ok_or(DbError::NotFound)?,
                identifier: row.identifier,
                version: attributes.version,
                path: attributes.path.map(PathBuf::from),
                status,
            })
        })
        .collect()
}

/// The working store: the only place that talks to SQLite. Speaks in domain types.
pub struct Store {
    pub(crate) conn: SqliteConnection,
}

impl Store {
    pub fn open() -> Store {
        Store { conn: open_store() }
    }

    #[cfg(test)]
    pub fn in_memory() -> Store {
        let mut conn = SqliteConnection::establish(":memory:").expect("in-memory database");
        conn.run_pending_migrations(MIGRATIONS).expect("migrations");
        Store { conn }
    }

    /// A store on a real file, opened the same way the app opens its own (busy timeout included).
    #[cfg(test)]
    pub fn open_at(path: &std::path::Path) -> Store {
        let mut conn = SqliteConnection::establish(path.to_str().unwrap()).expect("database file");
        configure(&mut conn);
        conn.run_pending_migrations(MIGRATIONS).expect("migrations");
        Store { conn }
    }

    /// True when no source has ever had a successful scan recorded. Evaluated once at the start of
    /// a run, so every source in that run is treated the same way.
    pub fn is_first_run(&mut self) -> QueryResult<bool> {
        read_first_run(&mut self.conn)
    }

    pub fn source_id(&mut self, source_name: &str) -> QueryResult<i32> {
        sources::table
            .filter(sources::name.eq(source_name))
            .select(sources::id)
            .first::<Option<i32>>(&mut self.conn)?
            .ok_or(DbError::NotFound)
    }

    pub fn load_source(&mut self, source_id: i32) -> QueryResult<StoredSource> {
        read_source(&mut self.conn, source_id)
    }

    pub fn load_tools(&mut self, source_id: i32) -> QueryResult<Vec<StoredTool>> {
        read_tools(&mut self.conn, source_id)
    }

    /// Opens a scan row. `finished_at` stays empty until `record_scan` succeeds, so a scan that
    /// fails or crashes never counts as a check.
    pub fn begin_scan(&mut self, run_id: &str, source_id: i32, by: &TriggeredBy) -> QueryResult<i32> {
        diesel::insert_into(scans::table)
            .values(NewScan { run_id, source_id, triggered_by: by.as_str() })
            .returning(scans::id)
            .get_result::<Option<i32>>(&mut self.conn)?
            .ok_or(DbError::NotFound)
    }

    /// One random id shared by every scan of a single run.
    pub fn new_run_id(&mut self) -> QueryResult<String> {
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            id: String,
        }
        Ok(diesel::sql_query("SELECT lower(hex(randomblob(16))) AS id")
            .get_result::<Row>(&mut self.conn)?
            .id)
    }
}

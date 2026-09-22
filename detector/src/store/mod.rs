use crate::models::NewTool;
use crate::scanner::DiscoveredTool;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::Serialize;
use std::path::PathBuf;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Extra fields that don't have their own `tools` column,
/// serialized into `tools.attributes` as JSON.
#[derive(Serialize)]
struct ToolAttributes<'a> {
    version: Option<&'a str>,
    path: &'a str,
}

fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("could not resolve platform app-data directory")
        .join("Legacy")
}

pub fn open_store() -> SqliteConnection {
    let dir = app_data_dir();
    std::fs::create_dir_all(&dir).expect("failed to create app-data directory");
    let db_path = dir.join("legacy.db");

    let mut conn = SqliteConnection::establish(db_path.to_str().unwrap())
        .unwrap_or_else(|err| panic!("failed to open {:?}: {}", db_path, err));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");

    conn
}

pub fn upsert_tool(conn: &mut SqliteConnection, source_name: &str, tool: &DiscoveredTool) {
    use crate::schema::sources;
    use crate::schema::tools;

    let source_id = sources::table
        .filter(sources::name.eq(source_name))
        .select(sources::id)
        .first::<Option<i32>>(conn)
        .unwrap_or_else(|_| panic!("unknown source: {source_name} (not seeded?)"))
        .expect("sources.id is never actually null once a row exists");

    let attributes = serde_json::to_string(&ToolAttributes {
        version: tool.version.as_deref(),
        path: tool.path.to_str().unwrap_or(""),
    })
    .expect("failed to serialize tool attributes");

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

use crate::schema::{sources, tools};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    #[diesel(column_name = type_)]
    pub source_type: String,
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
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tools)]
pub struct NewTool<'a> {
    pub name: &'a str,
    pub source_id: i32,
    pub attributes: Option<&'a str>,
    pub identifier: &'a str,
}

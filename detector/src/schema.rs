// @generated automatically by Diesel CLI.

diesel::table! {
    change_events (id) {
        id -> Nullable<Integer>,
        scan_id -> Integer,
        tool_id -> Nullable<Integer>,
        event_type -> Text,
        changes -> Text,
        occurred_at -> Text,
    }
}

diesel::table! {
    local_identity (id) {
        id -> Nullable<Integer>,
        platform_uuid -> Text,
        device_name -> Nullable<Text>,
        username -> Nullable<Text>,
        attributes -> Nullable<Text>,
        first_seen_at -> Text,
        last_seen_at -> Nullable<Text>,
    }
}

diesel::table! {
    scans (id) {
        id -> Nullable<Integer>,
        run_id -> Text,
        source_id -> Integer,
        triggered_by -> Text,
        started_at -> Text,
        finished_at -> Nullable<Text>,
    }
}

diesel::table! {
    sources (id) {
        id -> Nullable<Integer>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        status -> Nullable<Text>,
        version -> Nullable<Text>,
        first_seen_at -> Nullable<Text>,
        installed_at -> Nullable<Text>,
        baselined_at -> Nullable<Text>,
    }
}

diesel::table! {
    tools (id) {
        id -> Nullable<Integer>,
        name -> Text,
        source_id -> Integer,
        attributes -> Nullable<Text>,
        identifier -> Text,
        status -> Text,
        first_seen_at -> Text,
        installed_at -> Nullable<Text>,
    }
}

diesel::joinable!(change_events -> scans (scan_id));
diesel::joinable!(change_events -> tools (tool_id));
diesel::joinable!(scans -> sources (source_id));
diesel::joinable!(tools -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(change_events, local_identity, scans, sources, tools,);

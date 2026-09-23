// @generated automatically by Diesel CLI.

diesel::table! {
    local_identity (id) {
        id -> Nullable<Integer>,
        platform_uuid -> Text,
        device_name -> Nullable<Text>,
        username -> Nullable<Text>,
        attributes -> Nullable<Text>,
        first_seen -> Text,
    }
}

diesel::table! {
    sources (id) {
        id -> Nullable<Integer>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    tools (id) {
        id -> Nullable<Integer>,
        name -> Text,
        source_id -> Integer,
        attributes -> Nullable<Text>,
        identifier -> Text,
    }
}

diesel::joinable!(tools -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(local_identity, sources, tools,);

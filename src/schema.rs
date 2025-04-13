// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::*;

    events (id) {
        id -> Uuid,
        session_id -> Uuid,
        event_type -> Text,
        path -> Text,
        user_agent -> Nullable<Text>,
        referrer -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        created_at -> Timestamptz,
        metadata -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::*;

    sessions (id) {
        id -> Uuid,
        visitor_id -> Uuid,
        started_at -> Timestamptz,
        ended_at -> Nullable<Timestamptz>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::*;

    visitors (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        consent_given -> Bool,
        consent_updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(events -> sessions (session_id));
diesel::joinable!(sessions -> visitors (visitor_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    sessions,
    visitors,
);

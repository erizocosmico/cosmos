table! {
    user_sessions (id) {
        id -> Uuid,
        token -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        active -> Bool,
    }
}

joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);

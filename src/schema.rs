table! {
    refreshtokens (id) {
        id -> Uuid,
        token -> Uuid,
        user_id -> Uuid,
        expiry_date -> Timestamp,
    }
}

table! {
    states (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        progress -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        nr -> Int4,
        state -> Uuid,
        created_by -> Uuid,
        taken_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        taken_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        title -> Varchar,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(refreshtokens -> users (user_id));
joinable!(tasks -> states (state));

allow_tables_to_appear_in_same_query!(
    refreshtokens,
    states,
    tasks,
    users,
);

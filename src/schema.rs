table! {
    states (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        progress -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        state -> Uuid,
        created_by -> Uuid,
        taken_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        taken_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        title -> Varchar,
        description -> Varchar,
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

joinable!(tasks -> states (state));

allow_tables_to_appear_in_same_query!(
    states,
    tasks,
    users,
);

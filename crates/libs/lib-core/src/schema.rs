// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Uuid,
        serial_number -> Varchar,
        device_type -> Varchar,
        status -> Varchar,
        last_uplink -> Timestamp,
        owner_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(devices -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(devices, users,);

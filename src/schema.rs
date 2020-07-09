table! {
    devices (id) {
        id -> Int4,
        name -> Varchar,
        owner -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    logs (id) {
        id -> Int4,
        device_id -> Int4,
        received_at -> Timestamptz,
        updated_at -> Timestamptz,
        int_data -> Nullable<Int4>,
        str_data -> Nullable<Varchar>,
        float_data -> Nullable<Float8>,
        json_data -> Nullable<Jsonb>,
        is_file -> Nullable<Bool>,
    }
}

joinable!(logs -> devices (device_id));

allow_tables_to_appear_in_same_query!(
    devices,
    logs,
);

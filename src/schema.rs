table! {
    clusters (id) {
        id -> Int4,
        created_at -> Timestamptz,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    device_to_cluster (id) {
        id -> Int4,
        device_id -> Int4,
        cluster_id -> Int4,
    }
}

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

joinable!(device_to_cluster -> clusters (cluster_id));
joinable!(device_to_cluster -> devices (device_id));
joinable!(logs -> devices (device_id));

allow_tables_to_appear_in_same_query!(
    clusters,
    device_to_cluster,
    devices,
    logs,
);

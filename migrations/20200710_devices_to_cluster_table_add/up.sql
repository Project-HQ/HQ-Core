-- Your SQL goes here
CREATE TABLE device_to_cluster (
    id SERIAL PRIMARY KEY,
    device_id INTEGER NOT NULL REFERENCES devices(id),
    cluster_id INTEGER NOT NULL REFERENCES clusters(id)
)

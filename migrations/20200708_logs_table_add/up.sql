-- Your SQL goes here
CREATE TABLE logs (
  id SERIAL PRIMARY KEY,
  device_id INTEGER NOT NULL REFERENCES devices (id),
  received_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  int_data INTEGER,
  str_data VARCHAR,
  float_data FLOAT,
  json_data JSONB,
  is_file BOOLEAN
)

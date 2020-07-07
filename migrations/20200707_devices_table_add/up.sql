-- Your SQL goes here
CREATE TABLE devices (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  owner VARCHAR NOT NULL,
  description VARCHAR
)
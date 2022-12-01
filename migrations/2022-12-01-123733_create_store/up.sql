-- Your SQL goes here

CREATE TABLE store (
  id SERIAL PRIMARY KEY,
  store_name VARCHAR NOT NULL,
  cid BYTEA NOT NULL,
  bytes BYTEA NOT NULL
);

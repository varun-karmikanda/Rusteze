-- Your SQL goes here

-- USER
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- DEVICES
CREATE TABLE devices (
  id UUID PRIMARY KEY,
  serial_number VARCHAR NOT NULL,
  device_type VARCHAR NOT NULL,
  status VARCHAR NOT NULL,
  last_uplink TIMESTAMP NOT NULL,
  owner_id UUID REFERENCES users(id)
);
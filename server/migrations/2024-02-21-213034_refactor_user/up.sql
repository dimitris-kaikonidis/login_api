-- Your SQL goes here
ALTER TABLE users
DROP COLUMN password,
ADD COLUMN verifier VARCHAR(255) NOT NULL

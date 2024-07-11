-- Your SQL goes here
ALTER TABLE users
ADD COLUMN salt_new BYTEA NOT NULL;

ALTER TABLE users
DROP COLUMN salt;

ALTER TABLE users
RENAME COLUMN salt_new TO salt;


-- Your SQL goes here
ALTER TABLE users
ADD COLUMN verifier_new BYTEA NOT NULL;

ALTER TABLE users
DROP COLUMN verifier;

ALTER TABLE users
RENAME COLUMN verifier_new TO verifier;


-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN verifier_new VARCHAR(2000);

ALTER TABLE users
DROP COLUMN verifier;

ALTER TABLE users
RENAME COLUMN verifier_new TO verifier;


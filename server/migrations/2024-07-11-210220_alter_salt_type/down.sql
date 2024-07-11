-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN salt_new VARCHAR(2000);

ALTER TABLE users
DROP COLUMN salt;

ALTER TABLE users
RENAME COLUMN salt_new TO salt;


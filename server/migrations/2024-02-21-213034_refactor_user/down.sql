-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN verifier,
ADD COLUMN password VARCHAR(255) NOT NULL


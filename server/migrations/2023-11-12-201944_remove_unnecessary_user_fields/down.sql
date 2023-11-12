-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN first_name VARCHAR(255),
ADD COLUMN last_name VARCHAR(255),
ADD COLUMN display_name VARCHAR(255);


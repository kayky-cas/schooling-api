-- Add down migration script here
ALTER TABLE users
DROP CONSTRAINT fk_users_schools;

ALTER TABLE users
DROP COLUMN school_id;

DROP TABLE schools;

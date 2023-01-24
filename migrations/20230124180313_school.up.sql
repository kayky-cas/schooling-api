-- Add up migration script here
CREATE TABLE schools (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    domain VARCHAR NOT NULL
);

ALTER TABLE users
ADD school_id SERIAL;

ALTER TABLE users
ADD CONSTRAINT fk_users_schools
FOREIGN KEY (school_id)
REFERENCES schools (id);

-- Add up migration script here
CREATE TABLE schools (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    domain VARCHAR NOT NULL UNIQUE
);

ALTER TABLE users
ADD school_id INT;

ALTER TABLE users
ADD CONSTRAINT fk_users_schools
FOREIGN KEY (school_id)
REFERENCES schools (id);

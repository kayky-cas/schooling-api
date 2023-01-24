-- Add up migration script here
CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    cpf VARCHAR(14) UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    role user_role NOT NULL
)

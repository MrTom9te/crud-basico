CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) not null,
    email VARCHAR(255) not null,
    password VARCHAR(255) not null
);
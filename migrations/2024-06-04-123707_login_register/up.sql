CREATE TABLE users (
    id SERIAL PRIMARY KEY,    
    email VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL
);

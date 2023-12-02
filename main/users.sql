create table users
(
    id            INTEGER
        primary key,
    username      VARCHAR(50)  not null
        unique,
    password_hash VARCHAR(255) not null,
    created_at    TIMESTAMP default CURRENT_TIMESTAMP
);


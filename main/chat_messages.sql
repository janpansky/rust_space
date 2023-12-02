create table chat_messages
(
    id          INTEGER
        primary key,
    sender_id   INTEGER
        references users,
    receiver_id INTEGER
        references users,
    content     TEXT,
    timestamp   TIMESTAMP default CURRENT_TIMESTAMP
);


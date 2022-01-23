create table users (
    id serial primary key,
    username text not null
);

create table posts (
    id serial primary key,
    author_id integer references users(id) not null,
    content text
);

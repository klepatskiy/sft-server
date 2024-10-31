create table users_token
(
    id         uuid                                   not null primary key,
    user_id    uuid references users                  not null,
    token      text                                   not null unique,
    active     boolean                  default true  not null,
    expires_at timestamp with time zone               not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

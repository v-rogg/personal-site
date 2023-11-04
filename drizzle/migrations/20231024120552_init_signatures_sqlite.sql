drop table if exists signatures;
create table if not exists signatures (
    id          text primary key not null,
    name        text   not null,
    ts_created  text default current_timestamp not null,
    ts_modified text,
    approved    integer default null,
    signature   text   not null
);

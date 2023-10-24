drop table if exists signatures CASCADE;
create table signatures
(
    id uuid default extensions.uuid_generate_v4(),
    name        text                                   not null,
    ts_created  timestamp   default current_timestamp  not null,
    ts_modified timestamp,
    approved    bool        default null,
    signature   json                                   not null,

    primary key (id)
);

create or replace function update_ts_modified()
returns trigger
language plpgsql
as $$
begin
    NEW.ts_modified := current_timestamp;
    return NEW;
end;
$$;

drop trigger if exists signature_update_ts_modified on signatures;
create trigger signature_update_ts_modified
before update on signatures
for each row
execute procedure update_ts_modified()



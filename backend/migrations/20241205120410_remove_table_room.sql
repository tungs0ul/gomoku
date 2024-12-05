-- Add migration script here
alter table game drop constraint game_room_id_fkey;
drop table room;
drop type room_type;
create type game_type as enum ('bot', 'normal', 'private');
alter table game add column game_type game_type not null default 'normal';
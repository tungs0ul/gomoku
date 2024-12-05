-- Add migration script here
alter table game drop column x_ready;
alter table game drop column o_ready;

create type player_status as enum ('confirmed', 'confirmed_then_left' , 'left', 'ready');
alter table game add column x_status player_status not null default 'ready';
alter table game add column o_status player_status not null default 'ready';

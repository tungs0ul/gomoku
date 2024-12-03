-- Add migration script here
create type game_status as enum ('playing', 'ended', 'ready');
create type player as enum ('x', 'o');

create table game (
    id uuid not null primary key,
    room uuid not null,
    status game_status not null default 'ready',
    x_ready bool not null default false,
    o_ready bool not null default false,
    x uuid,
    o uuid,
    winner jsonb,
    init_player player not null default 'x',
    created_at timestamptz default now()
);

create table game_move(
    game_id uuid not null references game(id) on delete cascade,
    turn int not null check (turn >= 0 AND turn <= 255),
    player player not null,
    row smallint not null check (row >= 0 AND row <= 255),
    col smallint not null check (col >= 0 AND col <= 255),
    unique (game_id, turn)
);

create index idx_game_id_game_move on game_move(game_id);
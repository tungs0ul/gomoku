-- Add migration script here
alter table game enable row level security;
alter table game_move enable row level security;
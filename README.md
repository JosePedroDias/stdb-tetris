# Tetris STDB

## setup / cheat sheet

```
spacetime start --listen-addr='0.0.0.0:3000'
clear && spacetime publish --project-path server-rs tetris-game --delete-data -y
spacetime generate --lang typescript --out-dir client-ts/src/module_bindings --project-path server-rs

spacetime logs tetris-game

spacetime sql tetris-game "SELECT * FROM cell"
spacetime sql tetris-game "SELECT COUNT(*) as count FROM cell"
spacetime sql tetris-game "SELECT COUNT(*) AS count FROM cell WHERE value != 0"

spacetime sql tetris-game "SELECT * FROM board_data"

spacetime call tetris-game move_down

cd client-ts
clear && npm run build && npm run preview
(or) npm run dev
```

## TODO

- C prioritize the player's board vs opponents (order the player to be the leftmost, scale down opponents)
- S a single timer per boards-in-the-game (less resources, should look nicer seeing the movements in sync)
- S send garbage line(s) to random opponent when player breaks above n lines (3 -> 1, 4 -> 2)

- S/C? game table (links all the players in the same game together)
- basic lobby-less synced start by waiting for min number of players
- lobby state -> ready -> start shared game
- expose methods to get client internal state to bots
- add board_filled bool column to board_data to enforce game_over

- create keys for game_id, group players in sessions of N (2, ...)
- if more than X lines (2?), send garbage to another random player
- drop should trigger another piece to be added right away, not on next tick

## Model data

```
Cell
    id: u32
    board_id: u32
    x: u8
    y: u8
    value: u8

BoardData
    id: u32
    owner: Identity,
    selected_piece: u8,
    selected_piece_variant: u8,
    next_piece: u8,
    next_piece_variant: u8,
    pos_x: u8,
    pos_y: u8,
    pub ghost_y: u8,
    pub score: u32,
    pub lines: u32,

ScheduleMoveDown ~> move_down_from_timer
    id: u32
    scheduled_at: spacetimedb::ScheduleAt
    board_id: u32

```

## client-exposed reducers

- move_left()
- move_right()
- move_down()
- drop()
- rotate_left()
- rotate_right()

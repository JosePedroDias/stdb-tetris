# Tetris STDB

## setup / cheat sheet

```
spacetime start --listen-addr='0.0.0.0:3000'
clear && spacetime publish --project-path server-rs tetris-game --delete-data -y
spacetime generate --lang typescript --out-dir client-ts/src/module_bindings --project-path server-rs

spacetime logs tetris-game
spacetime sql tetris-game "SELECT * FROM cell"
spacetime sql tetris-game "SELECT * FROM board_data"
spacetime call tetris-game who_am_i

cd client-ts

clear && npm run build && npm run preview
npm run dev
```

## TODO

- Board::init should use ctx.rng
- Board::from_tables(cells, board_data) -> Board
- board.update_tables(ctx)
- create a "timer" to move bricks down
- expose reducers for player input

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
    selected_piece: u8,
    selected_piece_variant: u8,
    next_piece: u8,
    next_piece_variant: u8,
    pos_x: u8,
    pos_y: u8,
    pub ghost_y: u8,
    pub score: u32,
    pub lines: u32,
```

## client-exposed reducers

- move_left()
- move_right()
- move_down()
- drop()
- rotate_left()
- rotate_right()

use crate::tetris::Board;

use spacetimedb::{ReducerContext, ScheduleAt, Table};
use std::time::Duration;

use crate::tables::{board_data, cell, schedule_move_down, BoardData, Cell, ScheduleMoveDown};

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // called at module start
    log::info!("tetris-game init started");

    let mut b = Board::new(ctx); // TODO randomness from ctx.rng
                                 //b.move_left();
    b.apply_piece();

    //let brick = b.get_piece();
    //log::info!("brick: {:?}", brick); //, brick[0].0);

    //log::info!("display board\n{}", b);
    //log::info!("debug board{:?}", b);

    for (x, y, value) in b.board_iter() {
        ctx.db.cell().insert(Cell { id: 0, x, y, value });
    }
    ctx.db.board_data().insert(BoardData {
        id: 0,
        selected_piece: b.selected_piece,
        selected_piece_variant: b.selected_piece_variant,
        next_piece: b.next_piece,
        next_piece_variant: b.next_piece_variant,
        pos_x: b.position.0,
        pos_y: b.position.1,
        ghost_y: b.ghost_y,
        score: b.score,
        lines: b.lines,
    });

    ctx.db.schedule_move_down().insert(ScheduleMoveDown {
        id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(500).into()), // 2 fps ~ 500 ms
    });

    log::info!("tetris-game init just ran");
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    // Called everytime a new client connects
    log::info!("client {} connected.", ctx.sender);

    log::info!("client connected done");
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Called everytime a client disconnects
    log::info!("client {} disconnected.", ctx.sender);

    log::info!("client {} disconnected done.", ctx.sender);
}

#[spacetimedb::reducer]
pub fn move_down_from_timer(ctx: &ReducerContext, _timer_row: ScheduleMoveDown) {
    move_down_(ctx);
}

#[spacetimedb::reducer]
pub fn move_down(ctx: &ReducerContext) {
    move_down_(ctx);
}

fn move_down_(ctx: &ReducerContext) {
    //log::info!("move_down called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    let down_ok = b.move_down();
    if down_ok {
        b.apply_piece();
    } else {
        b.apply_piece();
        b.detect_lines();
        let game_continues = b.random_piece(ctx);
        if !game_continues {
            //is_game_over = true; // set this value to the game
            log::info!("GAME OVER");
            ctx.db.schedule_move_down().id().delete(1);
        }
        b.apply_piece();
    }

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn drop(ctx: &ReducerContext) {
    //log::info!("drop called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    b.drop();
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn move_left(ctx: &ReducerContext) {
    //log::info!("move_left called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    if !b.move_left() {
        return;
    }
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn move_right(ctx: &ReducerContext) {
    //log::info!("move_right called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    if !b.move_right() {
        return;
    }
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn rotate_left(ctx: &ReducerContext) {
    //log::info!("rotate_left called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    b.rotate_left();
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn rotate_right(ctx: &ReducerContext) {
    //log::info!("rotate_right called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx);

    b.unapply_piece();
    b.rotate_right();
    b.apply_piece();

    b.to_tables(ctx);
}

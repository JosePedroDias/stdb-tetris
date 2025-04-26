//#![allow(dead_code)]

mod bricks;
mod tetris;

use crate::tetris::Board;

//use rand::{seq::SliceRandom, Rng};
use spacetimedb::{ReducerContext, Table};

/////// TABLES

#[spacetimedb::table(name = cell, public)]
#[derive(Debug, Clone)]
pub struct Cell {
    #[auto_inc]
    #[primary_key]
    id: u32,

    //#[index(btree)]
    //board_id: bool,
    //
    x: u8,
    y: u8,
    value: u8,
}

#[spacetimedb::table(name = board_data, public)]
#[derive(Debug, Clone)]
pub struct BoardData {
    #[auto_inc]
    #[primary_key]
    id: u32,

    selected_piece: u8,
    selected_piece_variant: u8,
    next_piece: u8,
    next_piece_variant: u8,
    pos_x: u8,
    pos_y: u8,
    ghost_y: u8,
    score: u32,
    lines: u32,
}

/////// REDUCERS

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
pub fn who_am_i(ctx: &ReducerContext) {
    //let pl = ctx.db.player().id().find(ctx.sender).unwrap();
    log::info!("you are {}!", ctx.sender);
}

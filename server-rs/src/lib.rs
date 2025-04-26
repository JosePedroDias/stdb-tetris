#![allow(dead_code)]

mod bricks;
mod tetris;

//use crate::bricks::{I, J, L, O, S, T, Z};
use crate::tetris::Board;

use rand::{seq::SliceRandom, Rng};
use spacetimedb::{
    client_visibility_filter, Filter, Identity, ReducerContext, SpacetimeType, Table,
};

/////// REDUCERS

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // called at module start
    log::info!("tetris-game init started");

    let mut b = Board::new();
    //b.move_left();
    b.apply_piece();

    let brick = b.get_piece();
    log::info!("brick: {:?}", brick); //, brick[0].0);

    log::info!("display board\n{}", b);
    log::info!("debug board{:?}", b);

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

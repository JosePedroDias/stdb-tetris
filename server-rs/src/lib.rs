#![allow(dead_code)]

use rand::{seq::SliceRandom, Rng};
use spacetimedb::{
    client_visibility_filter, Filter, Identity, ReducerContext, SpacetimeType, Table,
};

/////// REDUCERS

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // called at module start
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

// #[spacetimedb::reducer]
// pub fn who_am_i(ctx: &ReducerContext) {
//     //let pl = ctx.db.player().id().find(ctx.sender).unwrap();
//     log::info!("you are {}!", ctx.sender);
// }

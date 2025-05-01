use crate::{
    tables::{game, player},
    tetris::Board,
};

use spacetimedb::{ReducerContext, ScheduleAt, Table};
use std::time::Duration;

use crate::tables::{
    board_data, cell, schedule_move_down, BoardData, Cell, Game, Player, ScheduleMoveDown,
};

pub const GAME_NR_OF_PLAYERS: u8 = 4;

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // called at module start
    log::info!("tetris-game init started");

    log::info!("tetris-game init just ran");
}

fn prepare_board(ctx: &ReducerContext) -> u32 {
    let mut b = Board::new(ctx);

    b.apply_piece();

    let bd = ctx.db.board_data().insert(BoardData {
        id: 0,
        owner: ctx.sender,
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

    b.board_id = bd.id;

    for (x, y, value) in b.board_iter() {
        ctx.db.cell().insert(Cell {
            id: 0,
            board_id: bd.id,
            x,
            y,
            value,
        });
    }

    bd.id
}

fn release_board(ctx: &ReducerContext) {
    let bd = get_board_data(ctx, None);

    for c in ctx.db.cell().board_id().filter(bd.id) {
        ctx.db.cell().id().delete(c.id);
    }

    ctx.db.board_data().id().delete(bd.id);

    //ctx.db.schedule_move_down().board_id().delete(bd.id);
}

pub fn get_board_data(ctx: &ReducerContext, board_id: Option<u32>) -> BoardData {
    match board_id {
        None => ctx
            .db
            .board_data()
            .owner()
            .filter(ctx.sender)
            .next()
            .unwrap(),
        Some(board_id) => ctx.db.board_data().id().find(board_id).unwrap(),
    }
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    // Called everytime a new client connects
    log::info!("client {} connected.", ctx.sender);

    let board_id = prepare_board(ctx);

    ctx.db.player().insert(Player {
        id: ctx.sender,
        game_id: 0,
        board_id,
    });

    let free_players: Vec<Player> = ctx.db.player().game_id().filter(0u32).collect();

    if free_players.len() == GAME_NR_OF_PLAYERS as usize {
        let game = ctx.db.game().insert(Game { id: 0 });
        for mut pl in free_players {
            pl.game_id = game.id;
            ctx.db.player().id().update(pl);
        }
        ctx.db.schedule_move_down().insert(ScheduleMoveDown {
            id: 0,
            scheduled_at: ScheduleAt::Interval(Duration::from_millis(500).into()), // 2 fps ~ 500 ms
            game_id: game.id,
        });
    }

    log::info!("client connected done");
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Called everytime a client disconnects
    log::info!("client {} disconnected.", ctx.sender);

    release_board(ctx);

    let pl = ctx.db.player().id().find(ctx.sender).unwrap();
    let game_id = pl.game_id;
    ctx.db.player().delete(pl);

    let players_left_in_the_game = ctx.db.player().game_id().filter(game_id).count();
    if players_left_in_the_game == 0 {
        log::info!("game became empty. finishing it...");
        let smd = ctx.db.schedule_move_down().game_id().find(game_id).unwrap();
        ctx.db.schedule_move_down().delete(smd);
        ctx.db.game().id().delete(game_id);
        log::info!("game and timer cleaned");
    } else {
        log::info!(
            "game still in place with {} players",
            players_left_in_the_game
        );
    }

    log::info!("client {} disconnected done.", ctx.sender);
}

#[spacetimedb::reducer]
pub fn move_down_from_timer(ctx: &ReducerContext, smd: ScheduleMoveDown) {
    for pl in ctx.db.player().game_id().filter(smd.game_id) {
        move_down_(ctx, Some(pl.board_id));
    }
}

#[spacetimedb::reducer]
pub fn move_down(ctx: &ReducerContext) {
    move_down_(ctx, None);
}

fn move_down_(ctx: &ReducerContext, board_id: Option<u32>) {
    //log::info!("move_down called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx, board_id);

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

            // TODO kill timer once all players left
            //ctx.db.schedule_move_down().board_id().delete(b.board_id);
        }
        b.apply_piece();
    }

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn drop(ctx: &ReducerContext) {
    //log::info!("drop called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx, None);

    b.unapply_piece();
    b.drop();
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn move_left(ctx: &ReducerContext) {
    //log::info!("move_left called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx, None);

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
    let mut b = Board::from_tables(ctx, None);

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
    let mut b = Board::from_tables(ctx, None);

    b.unapply_piece();
    b.rotate_left();
    b.apply_piece();

    b.to_tables(ctx);
}

#[spacetimedb::reducer]
pub fn rotate_right(ctx: &ReducerContext) {
    //log::info!("rotate_right called by {}.", ctx.sender);
    let mut b = Board::from_tables(ctx, None);

    b.unapply_piece();
    b.rotate_right();
    b.apply_piece();

    b.to_tables(ctx);
}

use spacetimedb::ScheduleAt;

use crate::reducers::move_down_from_timer;

#[spacetimedb::table(name = cell, public)]
#[derive(Debug, Clone)]
pub struct Cell {
    #[auto_inc]
    #[primary_key]
    pub id: u32,

    //#[index(btree)]
    //board_id: bool,
    //
    pub x: u8,
    pub y: u8,
    pub value: u8,
}

#[spacetimedb::table(name = board_data, public)]
#[derive(Debug, Clone)]
pub struct BoardData {
    #[auto_inc]
    #[primary_key]
    pub id: u32,

    pub selected_piece: u8,
    pub selected_piece_variant: u8,
    pub next_piece: u8,
    pub next_piece_variant: u8,
    pub pos_x: u8,
    pub pos_y: u8,
    pub ghost_y: u8,
    pub score: u32,
    pub lines: u32,
}

#[spacetimedb::table(name = schedule_move_down, scheduled(move_down_from_timer))]
#[derive(Debug, Clone)]
pub struct ScheduleMoveDown {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}

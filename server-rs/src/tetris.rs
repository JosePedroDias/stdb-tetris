use spacetimedb::rand::Rng;
use spacetimedb::{ReducerContext, Table};

use crate::bricks::{NUM_BRICKS, NUM_VARIANTS, O_INDEX};
use crate::{
    bricks::{I, J, L, O, S, T, Z},
    tables::board_data,
    tables::cell,
};

pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 20;

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: [[u8; WIDTH as usize]; HEIGHT as usize],
    pub selected_piece: u8,
    pub selected_piece_variant: u8,
    pub next_piece: u8,
    pub next_piece_variant: u8,
    pub position: (u8, u8),
    pub ghost_y: u8,
    pub score: u32,
    pub lines: u32,
}

impl Board {
    pub fn new(ctx: &ReducerContext) -> Self {
        let mut b = Board {
            cells: [[0; WIDTH as usize]; HEIGHT as usize],
            selected_piece: 0,
            selected_piece_variant: 0,
            next_piece: 0,
            next_piece_variant: 0,
            position: (0, 0),
            ghost_y: 0,
            score: 0,
            lines: 0,
        };

        b.random_piece(ctx);

        b
    }

    pub fn from_tables(ctx: &ReducerContext) -> Board {
        let mut b = Board {
            cells: [[0; WIDTH as usize]; HEIGHT as usize],
            selected_piece: 0,
            selected_piece_variant: 0,
            next_piece: 0,
            next_piece_variant: 0,
            position: (0, 0),
            ghost_y: 0,
            score: 0,
            lines: 0,
        };

        let bd = ctx.db.board_data().id().find(1).unwrap();
        b.selected_piece = bd.selected_piece;
        b.selected_piece_variant = bd.selected_piece_variant;
        b.next_piece = bd.next_piece;
        b.next_piece_variant = bd.next_piece_variant;
        b.position = (bd.pos_x, bd.pos_y);
        b.ghost_y = bd.ghost_y;
        b.score = bd.score;
        b.lines = bd.lines;

        for c in ctx.db.cell().iter() {
            b.cells[c.y as usize][c.x as usize] = c.value;
        }

        b
    }

    pub fn to_tables(&self, ctx: &ReducerContext) {
        let mut bd = ctx.db.board_data().id().find(1).unwrap();
        bd.selected_piece = self.selected_piece;
        bd.selected_piece_variant = self.selected_piece_variant;
        bd.next_piece = self.next_piece;
        bd.next_piece_variant = bd.next_piece_variant;
        bd.pos_x = self.position.0;
        bd.pos_y = self.position.1;
        bd.ghost_y = self.ghost_y;
        bd.score = self.score;
        bd.lines = self.lines;
        ctx.db.board_data().id().update(bd);

        for mut c in ctx.db.cell().iter() {
            let value = self.cells[c.y as usize][c.x as usize];
            if c.value != value {
                c.value = value;
                ctx.db.cell().id().update(c);
            }
        }
    }

    fn update_ghost_y(&mut self) {
        let br = self.get_piece();
        let mut ghost_y = self.position.1;
        loop {
            if !self.piece_doesnt_collide((self.position.0, ghost_y), &br) {
                ghost_y = ghost_y - 1;
                break;
            }
            let r = self.piece_fits_bounds((self.position.0, ghost_y), &br);
            if !r.3 {
                ghost_y = ghost_y - 1;
                break;
            }
            ghost_y += 1;
        }
        self.ghost_y = ghost_y;
    }

    // true means all good
    fn adjust_piece_placement(&mut self) -> bool {
        let piece = self.get_piece();
        loop {
            let r = self.piece_fits_bounds(self.position, &piece);
            if !r.3 {
                self.ghost_y = self.position.1;
                return false;
            } else if !r.0 {
                self.position.0 += 1;
            } else if !r.1 {
                self.position.0 -= 1;
            } else if !r.2 {
                self.position.1 += 1;
            } else {
                self.update_ghost_y();
                return true;
            }
        }
    }

    pub fn random_piece(&mut self, ctx: &ReducerContext) -> bool {
        let mut rng = ctx.rng();
        if self.score == 0 {
            self.selected_piece = rng.gen_range(0..NUM_BRICKS);
            self.selected_piece_variant = if self.selected_piece == O_INDEX {
                0
            } else {
                rng.gen_range(0..NUM_VARIANTS)
            };
        } else {
            self.selected_piece = self.next_piece;
            self.selected_piece_variant = self.next_piece_variant;
        }

        self.next_piece = rng.gen_range(0..NUM_BRICKS);
        self.next_piece_variant = if self.next_piece == O_INDEX {
            0
        } else {
            rng.gen_range(0..NUM_VARIANTS)
        };

        self.position = (WIDTH / 2, 0);

        self.adjust_piece_placement();

        let br = self.get_piece();
        return self.piece_doesnt_collide(self.position, &br);
    }

    fn get_piece_(&self, p: u8, v: usize) -> [(i8, i8); 4] {
        match p {
            0 => I[v],
            1 => J[v],
            2 => L[v],
            3 => O[v],
            4 => S[v],
            5 => T[v],
            6 => Z[v],
            _ => panic!("Invalid piece"),
        }
    }

    pub fn get_piece(&self) -> [(i8, i8); 4] {
        self.get_piece_(self.selected_piece, self.selected_piece_variant as usize)
    }

    pub fn get_next_piece(&self) -> [(i8, i8); 4] {
        self.get_piece_(self.next_piece, self.next_piece_variant as usize)
    }

    pub fn rotate_right(&mut self) {
        if self.selected_piece == O_INDEX {
            return;
        }
        self.selected_piece_variant = (self.selected_piece_variant + 1) % 4;
        self.adjust_piece_placement();
    }

    pub fn rotate_left(&mut self) {
        if self.selected_piece == O_INDEX {
            return;
        }
        self.selected_piece_variant = (self.selected_piece_variant + 3) % 4;
        self.adjust_piece_placement();
    }

    pub fn move_left(&mut self) -> bool {
        let p = self.position;
        if p.0 == 0 {
            return false;
        }
        let p1 = (p.0 - 1, p.1);
        let br = self.get_piece();
        if self.piece_fits_bounds(p1, &br) == (true, true, true, true)
            && self.piece_doesnt_collide(p1, &br)
        {
            self.position.0 -= 1;
            self.update_ghost_y();
            return true;
        }
        false
    }

    pub fn move_right(&mut self) -> bool {
        let p = self.position;
        let p1 = (p.0 + 1, p.1);
        let br = self.get_piece();
        if self.piece_fits_bounds(p1, &br) == (true, true, true, true)
            && self.piece_doesnt_collide(p1, &br)
        {
            self.position.0 += 1;
            self.update_ghost_y();
            return true;
        }
        false
    }

    pub fn move_down(&mut self) -> bool {
        let p = self.position;
        let p1 = (p.0, p.1 + 1);
        let br = self.get_piece();
        if self.piece_fits_bounds(p1, &br) == (true, true, true, true)
            && self.piece_doesnt_collide(p1, &br)
        {
            self.position.1 += 1;
            return true;
        }
        false
    }

    // TODO confirm correctness or rewrite
    pub fn drop(&mut self) -> bool {
        if self.ghost_y <= self.position.1 {
            return false;
        }
        self.position.1 = self.ghost_y;
        self.adjust_piece_placement();
        true
    }

    pub fn place_piece(&mut self, pos: (u8, u8), brick: &[(i8, i8); 4], color: u8) {
        let positions: Vec<(i8, i8)> = brick
            .iter()
            .map(move |(x, y)| (x + pos.0 as i8, y + pos.1 as i8))
            .collect();
        for (x, y) in positions {
            if x >= 0 && x < WIDTH as i8 && y >= 0 && y < HEIGHT as i8 {
                self.cells[y as usize][x as usize] = color;
            }
        }
    }

    pub fn piece_doesnt_collide(&mut self, pos: (u8, u8), brick: &[(i8, i8); 4]) -> bool {
        for (x, y) in brick
            .iter()
            .map(move |(x, y)| (x + pos.0 as i8, y + pos.1 as i8))
        {
            if x >= 0 && x < WIDTH as i8 && y >= 0 && y < HEIGHT as i8 {
                if self.cells[y as usize][x as usize] != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn piece_fits_bounds(
        &mut self,
        pos: (u8, u8),
        brick: &[(i8, i8); 4],
    ) -> (bool, bool, bool, bool) {
        let mut x0 = true;
        let mut x1 = true;
        let mut y0 = true;
        let mut y1 = true;

        for (x, y) in brick
            .iter()
            .map(move |(x, y)| (x + pos.0 as i8, y + pos.1 as i8))
        {
            if x < 0 {
                x0 = false;
            }
            if x >= WIDTH as i8 {
                x1 = false;
            }
            if y < 0 {
                y0 = false;
            }
            if y >= HEIGHT as i8 {
                y1 = false;
            }
        }
        (x0, x1, y0, y1)
    }

    fn piece_iter<'a>(
        &self,
        pos: (u8, u8),
        brick: &'a [(i8, i8); 4],
    ) -> impl Iterator<Item = (i8, i8)> + 'a {
        brick
            .iter()
            .map(move |(x, y)| (x + pos.0 as i8, y + pos.1 as i8))
    }

    fn lines_rev_iter(&self) -> impl Iterator<Item = (u8, [u8; WIDTH as usize])> + '_ {
        (0..HEIGHT)
            .rev()
            .into_iter()
            .map(move |y| (y, self.cells[y as usize]))
    }

    pub fn line_numbers_iter(&self) -> impl Iterator<Item = u8> {
        (0..HEIGHT).map(move |y| (y))
    }

    pub fn board_iter(&self) -> impl Iterator<Item = (u8, u8, u8)> + '_ {
        (0..HEIGHT)
            .flat_map(move |y| (0..WIDTH).map(move |x| (x, y, self.cells[y as usize][x as usize])))
    }

    pub fn detect_lines(&mut self) -> bool {
        self.score += 4;

        let mut lines = Vec::new();
        for (_, row) in self.lines_rev_iter() {
            // last to first!
            if row.iter().any(|&cell| cell == 0) {
                lines.insert(0, row);
            }
        }

        let remaining_lines = HEIGHT - lines.len() as u8;
        if remaining_lines == 0 {
            return false;
        }
        self.lines += remaining_lines as u32;
        for _ in 0..remaining_lines {
            lines.insert(0, [0; WIDTH as usize]);
        }

        for (y, line) in lines.iter().enumerate() {
            for x in 0..WIDTH {
                self.cells[y as usize][x as usize] = line[x as usize];
            }
        }

        true
    }

    pub fn apply_piece(&mut self) {
        self.place_piece(self.position, &self.get_piece(), self.selected_piece + 1);
    }

    pub fn unapply_piece(&mut self) {
        self.place_piece(self.position, &self.get_piece(), 0);
    }
}

/*
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
*/

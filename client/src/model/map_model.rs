use serde_json::Value;
use roguelike_common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MMap {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub tiles: Vec<TileType>,
    pub contents: Vec<Vec<i32>>,
    pub status: Vec<i32>,
    pub fov: Vec<usize>,
    pub viewport: Vec<i32>,
    pub ppos: i32,
    pub target: i32,
}

pub const VP_W: i32 = 20;
pub const VP_H: i32 = 16;

impl MMap {
    pub fn new() -> Self {
         Self {
            width: 0,
            height: 0,
            depth: 0,
            tiles: Vec::new(),
            contents: Vec::new(),
            status: Vec::new(),
            fov: Vec::new(),
            viewport: Vec::new(),
            ppos: 0,
            target: 0,
        }
    }

    fn get_dim(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn set_map(&mut self, data: Value) {
        let game: (i32, i32, i32) = serde_json::from_value(data).unwrap();
        self.width = game.0;
        self.height = game.1;
        self.depth = game.2;
        self.tiles = vec![TileType::Floor; self.get_dim()];
        self.contents = vec![Vec::new(); self.get_dim()];
        self.status = vec![0; self.get_dim()];
    }

    pub fn set_fov(&mut self, data: Value) {
        for c in &self.fov {
            self.status[*c] &= !TARGETED;
            self.status[*c] &= !VISIBLE;
            self.status[*c] |= SEEN;
        }
        self.fov.clear();
        self.ppos = serde_json::from_value(data[0].clone()).unwrap();
        let fov: Fov = serde_json::from_value(data[1].clone()).unwrap();
        self.set_viewport();
        for (tile, indexes) in &fov {
            for idx in indexes.iter() {
                self.tiles[*idx] = *tile;
                self.status[*idx] |= VISIBLE;
                self.fov.push(*idx);
            }
        }
    }

    pub fn set_contents(&mut self, data: Value) {
        let contents: Vec<(usize, Vec<i32>)> = serde_json::from_value(data).unwrap();
        self.contents = vec![Vec::new(); self.get_dim()];
        for (idx, c) in &contents {
            self.contents[*idx] = c.to_vec();
        }
    }

    pub fn set_viewport(&mut self) {
        let ppos = self.ppos;
        let x = ppos % self.width;
        let y = ppos / self.width;
        let mut v = Vec::new();
        let h = (VP_H as f64 / 2.0).floor() as i32;
        let w = (VP_W as f64 / 2.0).floor() as i32;
        let mut d = 0;
        let mut e = 0;
        if y < h {
            d = (y - h).abs();
        }
        if y >= self.height - h {
            d = self.height - y - h - 1;
        }
        if x < w {
            e = w - x;
        }
        if x >= self.width - w {
            e = self.width - x - w;
        }
        for i in (h - d) * -1..=(h + d) {
            let left_point = ppos + i * self.width - w;
            let mut r = (left_point + e..left_point + VP_W + e).collect::<Vec<i32>>();
            v.append(&mut r);
            let dim = self.width * self.height;
            v.retain(|n| n >= &0 && n < &dim);
        };
        self.viewport = v;
    }

    pub fn reset_targeter(&mut self) {
        self.target = self.ppos;
    }

    pub fn set_single_target(&mut self, target: usize) {
        for idx in self.fov.iter() {
            self.status[*idx] &= !TARGETED;
        }
        self.status[target] |= TARGETED;
    }


    fn move_target(&mut self, x: i32, y: i32) -> Option<i32> {
        let pos = self.idx_xy(self.target);
        let new_pos = self.xy_idx(pos.x + x, pos.y + y);
        let p = new_pos as usize;
        if self.fov.contains(&p) {
            self.target = new_pos;
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn try_move(&mut self, key_code: u32) -> Option<i32> {
        match key_code {
            KEY_LEFT => self.move_target(-1, 0),
            KEY_RIGHT => self.move_target(1, 0),
            KEY_UP => self.move_target(0, -1),
            KEY_DOWN => self.move_target(0, 1),
            KEY_Y => self.move_target(-1, -1),
            KEY_U => self.move_target(1, -1),
            KEY_N => self.move_target(1, 1),
            KEY_B => self.move_target(-1, 1),
            _ => None
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> i32 {
        (y * self.width) + x
    }

    fn idx_xy(&self, idx: i32) -> Point {
        Point::new(idx % self.width, idx / self.width)
    }
}


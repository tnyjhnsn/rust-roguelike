use roguelike_common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub entities: Vec<String>,
    pub status: Vec<i32>,
    pub current_fov: Vec<usize>,
    pub viewport: Vec<i32>,
}

pub const VP_W: i32 = 20;
pub const VP_H: i32 = 16;

impl Map {
    pub fn new() -> Self {
         Map {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            entities: Vec::new(),
            status: Vec::new(),
            current_fov: Vec::new(),
            viewport: Vec::new(),
        }
    }

    pub fn set_viewport(&mut self, ppos: i32) {
        let x = ppos % self.width;
        let y = ppos / self.width;
        let mut v = Vec::new();
        let h = (VP_H as f64 / 2.0).floor() as i32;
        let w = (VP_W as f64 / 2.0).floor() as i32;
        let mut d = 0;
        let mut e = 0;
        if y < h || y >= self.height - h {
            d = (y - h).abs();
        }
        if y >= self.height - h {
            d = self.height - y - h;
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
}

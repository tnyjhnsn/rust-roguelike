pub const VP_W: i32 = 20;
pub const VP_H: i32 = 14;

#[derive(Debug, PartialEq, Clone)]
pub struct Viewport {
    pub game_width: i32,
    pub indexes: Vec<i32>,
}

impl Viewport {
    pub fn new(game_width: i32) -> Self {
        Viewport {
            game_width,
            indexes: Vec::new(),
        }
    }

    pub fn set_indexes(&mut self, ppos: i32) {
        let x = ppos % self.game_width;
        let y = ppos / self.game_width;
        let mut v = Vec::new();
        let h = (VP_H as f64 / 2.0).floor() as i32;
        let w = (VP_W as f64 / 2.0).floor() as i32;
        let mut d = 0;
        if y < h || y >= self.game_width - h {
            d = (y - h).abs();
        }
        if y >= self.game_width - h {
            d = self.game_width - y - h;
        }
        let mut e = 0;
        if x < w {
            e = w - x;
        }
        if x >= self.game_width - w {
            e = self.game_width - x - w;
        }
        for i in (h - d) * -1..=(h + d) {
            let left_point = ppos + i * self.game_width - w;
            let mut r = (left_point + e..left_point + VP_W + e).collect::<Vec<i32>>();
            v.append(&mut r);
            v.retain(|n| n >= &0 && n < &1600);
        };
        self.indexes = v;
    }
}

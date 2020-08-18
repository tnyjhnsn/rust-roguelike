use roguelike_common::*;
use serde_json::json;

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
}

impl Map {

    pub fn new_map() -> Self {
        let width: i32 = 40;
        let height: i32 = 40;
        let dim = (width * height) as usize;
        let tiles = vec![TileType::Floor; dim];

        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn create_temp_walls(&mut self) {
        let mut rng = rltk::RandomNumberGenerator::new();

        for x in 0..self.width {
            let mut idx = self.xy_idx(x, 0);
            self.tiles[idx] = TileType::Wall;
            idx = self.xy_idx(x, self.height - 1);
            self.tiles[idx] = TileType::Wall;
        }
        for y in 0..self.height {
            let mut idx = self.xy_idx(0, y);
            self.tiles[idx] = TileType::Wall;
            idx = self.xy_idx(self.width - 1, y);
            self.tiles[idx] = TileType::Wall;
        }

        for _i in 0..200 {
            let x = rng.roll_dice(1, self.width - 1);
            let y = rng.roll_dice(1, self.height - 1);
            let idx = self.xy_idx(x, y);
            if idx != self.xy_idx(20, 10) {
                self.tiles[idx] = TileType::Wall;
            }
        }
    }

    pub fn draw_game(&self) -> String {
        let map = (self.width, self.height);
        let gm = GameMsg {
            msg: String::from("GAME"),
            data: serde_json::to_value(map).unwrap(),
        };
        let s = serde_json::to_string(&gm).unwrap();
        //println!("{}", s);
        s
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn get_random_space(&self) -> (i32, i32) {
        let mut rng = rltk::RandomNumberGenerator::new();
        loop {
            let x = rng.roll_dice(1, self.width - 1);
            let y = rng.roll_dice(1, self.height - 1);
            let idx = self.xy_idx(x, y);
            if self.tiles[idx] == TileType::Floor {
                break (x, y)
            }
        }
    }
}

pub fn draw_fov(fov: Fov, ent: Entities) -> String {
    let mut d = Vec::new();
    let f = serde_json::to_value(fov).unwrap();
    let e = serde_json::to_value(ent).unwrap();
    d.push(f);
    d.push(e);
    let gm = GameMsg {
        msg: String::from("FOV"),
        data: json!(d),
    };
    let s = serde_json::to_string(&gm).unwrap();
    //println!("{}", s);
    s
}


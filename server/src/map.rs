use roguelike_common::*;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
}

impl Map {

    pub fn new_map() -> Self {
        let mut tiles = vec![TileType::Floor; 60*20];
        let width = 60;
        let height = 20;

        for x in 0..width {
            tiles[xy_idx(x, 0)] = TileType::Wall;
            tiles[xy_idx(x, height-1)] = TileType::Wall;
        }
        for y in 0..height {
            tiles[xy_idx(0, y)] = TileType::Wall;
            tiles[xy_idx(width-1, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..200 {
            let x = rng.roll_dice(1, 59);
            let y = rng.roll_dice(1, 19);
            let idx = xy_idx(x, y);
            if idx != xy_idx(20, 10) {
                tiles[idx] = TileType::Wall;
            }
        }
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn draw_map(&self) -> String {
        let map = (self.width, self.height);
        let gm = GameMsg {
            msg: String::from("GAME"),
            data: serde_json::to_value(map).unwrap(),
        };
        let s = serde_json::to_string(&gm).unwrap();
            println!("{}", s);
        s
    }
}

pub fn draw_fov(fov: Fov) -> String {
    let gm = GameMsg {
        msg: String::from("FOV"),
        data: serde_json::to_value(fov).unwrap(),
    };
    let s = serde_json::to_string(&gm).unwrap();
    //println!("{}", s);
    s
}

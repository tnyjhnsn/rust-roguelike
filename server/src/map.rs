use roguelike_common::*;

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 60*20];

    for x in 0..60 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 19)] = TileType::Wall;
    }
    for y in 0..20 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(59, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..200 {
        let x = rng.roll_dice(1, 59);
        let y = rng.roll_dice(1, 19);
        let idx = xy_idx(x, y);
        if idx != xy_idx(20, 10) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn get_position(e: EntityPositions) -> String {
    let gm = GameMsg {
        msg: String::from("POSITION"),
        data: serde_json::to_value(e).unwrap(),
    };
    let s = serde_json::to_string(&gm).unwrap();
    println!("{}", s);
    s
}

pub fn draw_map(tiles: Vec<TileType>) -> String {
    let map = Map { 
        width: 60,
        height: 20,
        tiles,
        entities: vec!(),
        status: vec!(),
    };
    let gm = GameMsg {
        msg: String::from("GAME"),
        data: serde_json::to_value(map).unwrap(),
    };
    serde_json::to_string(&gm).unwrap()
}

pub fn draw_fov(fov: Fov) -> String {
    let gm = GameMsg {
        msg: String::from("FOV"),
        data: serde_json::to_value(fov).unwrap(),
    };
    let s = serde_json::to_string(&gm).unwrap();
    println!("{}", s);
    s
}

use roguelike_common::*;

pub struct GameLog {
    pub logs: Logs,
    pub dirty: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![(0, String::from("Hello Rogue!"))],
            dirty: false,
        }
    }
    pub fn draw_gamelog(&self) -> String {
        let gm = GameMsg {
            msg: String::from("LOG"),
            data: serde_json::to_value(&self.logs).unwrap(),
        };
        let s = serde_json::to_string(&gm).unwrap();
        println!("MSG {}", s);
        s
    }
}


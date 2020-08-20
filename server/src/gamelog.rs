use roguelike_common::*;

pub struct GameLog {
    pub logs: Logs,
    pub has_log: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![(0, String::from("Hello Rogue!"))],
            has_log: true,
        }
    }

    pub fn add_log(&mut self, log: (u8, String)) {
        self.logs.push(log);
        self.has_log = true;
    }

    pub fn draw_gamelog(&mut self) -> Option<String> {
        if self.has_log == true {
            let gm = GameMsg {
                msg: String::from("LOG"),
                data: serde_json::to_value(&self.logs).unwrap(),
            };
            let s = serde_json::to_string(&gm).unwrap();
            println!("GAMELOG {}", s);
            self.logs.clear();
            self.has_log = false;
            Some(s)
        } else {
            None
        }
    }
}


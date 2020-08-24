use serde_json::json;
use roguelike_common::*;

pub struct GameLog {
    pub logs: Logs,
    pub has_log: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![(LogType::General, String::from("Hello Rogue!"))],
            has_log: true,
        }
    }

    pub fn add_log(&mut self, log: (LogType, String)) {
        self.logs.push(log);
        self.has_log = true;
    }

    pub fn draw_gamelog(&mut self) -> Option<String> {
        if self.has_log == true {
            let gm = GameMsg {
                msg: String::from("LOG"),
                data: json!(&self.logs),
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


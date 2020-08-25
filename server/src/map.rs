use specs::prelude::*;
use roguelike_common::*;
use serde_json::json;
use rand::Rng;
use std::collections::HashMap;

const DIJKSTRA_MAX: i32 = 1000;
const WIDTH: i32 = 40;
const HEIGHT: i32 = 40;

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub blocked: Vec<bool>,
    pub neighbours: Vec<Vec<usize>>,
    pub dijkstra_values: Vec<i32>,
    pub contents: Vec<Vec<Entity>>,
}

impl Map {

    pub fn new() -> Self {
        let width = WIDTH;
        let height = HEIGHT;
        let dim = (width * height) as usize;
        let tiles = vec![TileType::Floor; dim];
        let blocked = vec![false; dim];
        let neighbours = vec![Vec::new(); dim];
        let dijkstra_values = Vec::new();
        let contents = vec![Vec::new(); dim];

        let mut map = Map {
            width,
            height,
            tiles,
            blocked,
            neighbours,
            dijkstra_values,
            contents,
        };

        map.populate_neighbours();
        map
    }

    fn get_dim(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn clear_contents(&mut self) {
        for content in self.contents.iter_mut() {
            content.clear();
        }
    }

    pub fn populate_dijkstra_values(&mut self, dijkstra_map: &[usize], x: i32, y: i32) -> Position {
        self.dijkstra_values = vec![DIJKSTRA_MAX; self.get_dim()];
        self.dijkstra_values[dijkstra_map[0]] = 0;
        for i in dijkstra_map.iter() {
            let dv = self.dijkstra_values[*i]; 
            for n in self.neighbours[*i].iter() {
                if self.blocked[*n] == false && self.dijkstra_values[*n] == DIJKSTRA_MAX {
                    self.dijkstra_values[*n] = dv + 1;
                };
            };
        };

        let idx = self.xy_idx(x, y);
        let dv = self.dijkstra_values[idx];
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for n in self.neighbours[idx].iter() {
            if self.blocked[*n] == false {
                if self.dijkstra_values[*n] < dv {
                    v.push(n);
                }
            };
        };

        if v.len() > 0 {
            let n = rng.gen_range(0, v.len());
            return Position { x: *v[n] as i32 % WIDTH, y: *v[n] as i32 / WIDTH };
        }; 

        Position { x, y }
    }

    fn populate_neighbours(&mut self) {
        for i in 0..self.get_dim() - 1 {
            let row = i as i32 % self.width;
            let col = i as i32 / self.width;
            let mut neighbours = Vec::new();
            for r in [-1, 0, 1].iter().cloned() {
                let rr = row + r;
                if rr < 0 || rr >= self.height { continue; }
                for c in [-1, 0, 1].iter().cloned() {
                    let cc = col + c;
                    if (r == 0 && c == 0) || cc < 0 || cc >= self.width { continue; }
                    neighbours.push(self.xy_idx(rr, cc));
                }
            };
            self.neighbours[i] = neighbours;
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn create_temp_walls(&mut self) {
        let mut rng = rand::thread_rng();

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
            let x = rng.gen_range(1, self.width - 1);
            let y = rng.gen_range(1, self.height - 1);
            let idx = self.xy_idx(x, y);
            if idx != self.xy_idx(20, 20) {
                self.tiles[idx] = TileType::Wall;
            }
        }
    }

    pub fn draw_game(&self) -> String {
        let mut map = HashMap::new();
        map.entry(String::from("GAME")).or_insert((self.width, self.height));
        let gm = GameMsg {
            data: json!(map),
        };
        let s = serde_json::to_string(&gm).unwrap();
        //println!("{}", s);
        s
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn get_random_space(&self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(1, self.width - 1);
            let y = rng.gen_range(1, self.height - 1);
            let idx = self.xy_idx(x, y);
            if self.tiles[idx] == TileType::Floor {
                break (x, y)
            }
        }
    }
}


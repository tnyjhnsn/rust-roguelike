use specs::prelude::*;
use roguelike_common::*;
use serde_json::json;
use rand::Rng;
use std::collections::HashMap;
use super::*;

const DIJKSTRA_MAX: i32 = 1000;
const WIDTH: i32 = 30;
const HEIGHT: i32 = 61;

#[derive(Debug, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub blocked: Vec<bool>,
    pub neighbours: Vec<Vec<usize>>,
    pub dijkstra_values: Vec<i32>,
    pub contents: Vec<Vec<Entity>>,
    pub depth: i32,
}

impl Map {

    pub fn new(depth: i32) -> Self {
        let width = WIDTH;
        let height = HEIGHT;
        let dim = (width * height) as usize;
        let tiles = Vec::with_capacity(dim);
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
            depth,
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

    pub fn get_area_of_effect(&self, area: &mut Vec<i32>, radius: i32) {
        if radius == 0 {
            return;
        }
        let mut v = Vec::new();
        for idx in area.iter() {
            for n in &self.neighbours[*idx as usize] {
                v.push(*n as i32);
            }
        }
        area.append(&mut v);
        self.get_area_of_effect(area, radius - 1);
    }

    pub fn populate_dijkstra_values(&mut self, dijkstra_map: &Vec<usize>, x: i32, y: i32) -> Position {
        self.dijkstra_values = vec![DIJKSTRA_MAX; self.get_dim()];
        self.dijkstra_values[dijkstra_map[0]] = 0;
        for i in dijkstra_map.iter() {
            let dv = self.dijkstra_values[*i]; 
            for n in &self.neighbours[*i] {
                if self.blocked[*n] == false && self.dijkstra_values[*n] >= DIJKSTRA_MAX {
                    self.dijkstra_values[*n] = dv + 1;
                };
            };
        };

        let idx = self.xy_idx(x, y);
        let dv = self.dijkstra_values[idx];
        let v: Vec<&usize> = self.neighbours[idx].iter()
            .filter(|n| self.blocked[**n as usize] == false && self.dijkstra_values[**n as usize] < dv)
            .collect();

        let mut rng = rand::thread_rng();
        if v.len() > 0 {
            let n = rng.gen_range(0, v.len());
            return Position { x: *v[n] as i32 % WIDTH, y: *v[n] as i32 / WIDTH };
        }; 

        Position { x, y }
    }

    fn populate_neighbours(&mut self) {
        for i in 0..self.get_dim() {
            let (x, y) = self.idx_xy(i as i32);
            let mut neighbours = Vec::new();
            for c in [-1, 0, 1].iter().cloned() {
                let xx = x + c;
                if xx < 0 || xx >= self.width { continue; }
                for r in [-1, 0, 1].iter().cloned() {
                    let yy = y + r;
                    if (c == 0 && r == 0) || yy < 0 || yy >= self.height { continue; }
                    neighbours.push(self.xy_idx(xx, yy));
                }
            };
            self.neighbours[i] = neighbours;
        }
    }

    pub fn populate_blocked(&mut self) {
        for (idx, i) in DWARVEN_MINES_GATE.iter().enumerate() {
            self.blocked[idx] = match i {
                1|3 => true,
                _ => false,
            }
        }
    }

    pub fn create_temp_walls(&mut self) {
        for (idx, i) in DWARVEN_MINES_GATE.iter().enumerate() {
            match i {
                1 => self.tiles.push(TileType::Wall),
                3 => {
                    self.tiles.push(TileType::Floor);
                    self.blocked[idx] = true;
                }
                _ => self.tiles.push(TileType::Floor),
            }
        }
    }

    pub fn draw_map(&self) -> String {
        let mut map = HashMap::new();
        map.entry(String::from("MAP"))
            .or_insert((self.width, self.height, self.depth, &self.tiles));
        let gm = GameMsg {
            data: json!(map),
        };
        let s = serde_json::to_string(&gm).unwrap();
        s
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: i32) -> (i32, i32) {
        (idx % self.width, idx / self.width)
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

pub fn down_stairs(ecs: &mut World) -> Map {
    let mut dungeon = ecs.fetch_mut::<Dungeon>();
    let map = ecs.fetch::<Map>();
    dungeon.store_map(&map);
    Map::new(map.depth + 1)
}

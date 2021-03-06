use roguelike_common::*;
use serde_json::json;
use std::collections::{HashMap, HashSet};

const DIJKSTRA_MAX: i32 = 1000;

#[derive(Debug, Clone)]
pub struct Map {
    pub key: &'static str,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub neighbours: Vec<Vec<usize>>,
    pub dijkstra_values: Vec<i32>,
    pub difficulty: i32,
    pub view_blocked: HashSet<usize>,
}

impl Map {

    pub fn new(key: &'static str, width: i32, height: i32, new_tiles: &[i32],
        difficulty: i32) -> Self { 

        let dim = (width * height) as usize;
        crate::spatial::set_size(dim);
        let mut map = Map {
            key,
            width,
            height,
            tiles: Vec::with_capacity(dim),
            neighbours: vec![Vec::new(); dim],
            dijkstra_values: Vec::new(),
            difficulty,
            view_blocked: HashSet::new(),
        };

        map.populate_tiles(new_tiles);
        map.populate_neighbours();
        map
    }

    fn get_dim(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn clear_contents(&mut self) {
        crate::spatial::clear();
    }

    pub fn get_area_of_effect(&self, area: &mut Vec<usize>, radius: i32) {
        if radius == 0 {
            return;
        }
        let mut v = Vec::new();
        for idx in area.iter() {
            for n in &self.neighbours[*idx] {
                v.push(*n);
            }
        }
        area.append(&mut v);
        self.get_area_of_effect(area, radius - 1);
    }

    fn populate_dijkstra_values(&mut self, dijkstra_map: &Vec<usize>, target_idx: usize) {
        self.dijkstra_values = vec![DIJKSTRA_MAX; self.get_dim()];
        self.dijkstra_values[dijkstra_map[0]] = 0;
        for i in dijkstra_map.iter() {
            let dv = self.dijkstra_values[*i]; 
            for n in &self.neighbours[*i] {
                let mut blocked = crate::spatial::is_blocked(*n);
                if *n == target_idx { blocked = false; }
                if blocked == false && self.dijkstra_values[*n] >= DIJKSTRA_MAX {
                    self.dijkstra_values[*n] = dv + 1;
                };
            };
        };

    }
    
    pub fn dijkstra_exit<F>(&mut self, dijkstra_map: &Vec<usize>,
        x: i32, y: i32, f: F) -> Position
    where F: Fn(&Map, usize, i32) -> Vec<usize>
    {
        let idx = self.xy_idx(x, y);
        self.populate_dijkstra_values(dijkstra_map, idx);
        let dv = self.dijkstra_values[idx];
        let v = f(self, idx, dv);
        let mut rng = RandomNumberGenerator::new();
        if v.len() > 0 {
            let n = rng.range(0, v.len());
            return Position { x: v[n] as i32 % self.width, y: v[n] as i32 / self.width };
        }; 
        Position { x, y }
    }

    fn populate_neighbours(&mut self) {
        for i in 0..self.get_dim() {
            let pos = self.idx_xy(i as i32);
            let mut neighbours = Vec::new();
            for c in [-1, 0, 1].iter().cloned() {
                let xx = pos.x + c;
                if xx < 0 || xx >= self.width { continue; }
                for r in [-1, 0, 1].iter().cloned() {
                    let yy = pos.y + r;
                    if (c == 0 && r == 0) || yy < 0 || yy >= self.height { continue; }
                    neighbours.push(self.xy_idx(xx, yy));
                }
            };
            self.neighbours[i] = neighbours;
        }
    }

    pub fn populate_blocked(&mut self) {
        crate::spatial::populate_blocked_from_map(self);
    }

    pub fn populate_tiles(&mut self, tiles: &[i32]) {
        for i in tiles.iter() {
            match i {
                1 => self.tiles.push(TileType::Wall),
                2 => self.tiles.push(TileType::Blocked), 
                3 => self.tiles.push(TileType::Door), 
                49 => self.tiles.push(TileType::ExitMap),
                50 => self.tiles.push(TileType::Chasm),
                51 => self.tiles.push(TileType::Lava),
                _ => self.tiles.push(TileType::Floor),
            }
        }
    }

    pub fn draw_map(&self) -> String {
        let mut map = HashMap::new();
        map.entry(String::from("MAP"))
            .or_insert((self.key, self.width, self.height));
        let gm = GameMsg {
            data: json!(map),
        };
        let s = serde_json::to_string(&gm).unwrap();
        s
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: i32) -> Position {
        Position::new(idx % self.width, idx / self.width)
    }

    pub fn get_random_space(&self, rng: &mut RandomNumberGenerator) -> Position {
        loop {
            let x = rng.range(1, self.width - 1);
            let y = rng.range(1, self.height - 1);
            let idx = self.xy_idx(x, y);
            if self.tiles[idx] == TileType::Floor {
                break Position::new(x, y)
            }
        }
    }
}

pub fn lowest_exit(map: &Map, idx: usize, dv: i32) -> Vec<usize> {
    map.neighbours[idx].iter()
        .filter(|n| {
            let blocked = crate::spatial::is_blocked(**n);
            !blocked && map.dijkstra_values[**n] < dv 
        })
        .cloned()
        .collect()
}

pub fn highest_exit(map: &Map, idx: usize, dv: i32) -> Vec<usize> {
    map.neighbours[idx].iter()
        .filter(|n| {
            let blocked = crate::spatial::is_blocked(**n);
            !blocked && map.dijkstra_values[**n] > dv 
        })
        .cloned()
        .collect()
}


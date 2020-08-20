use specs::prelude::*;
use roguelike_common::*;
use super::player::*;
use super::gamelog::*;
use super::components::*;
use super::map::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = ( WriteExpect<'a, PlayerPosition>,
                        ReadStorage<'a, FieldOfView>, 
                        ReadExpect<'a, Map>, 
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteExpect<'a, GameLog>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut ppos, fov, map, monster, mut mpos, mut log, name) = data;

        for (fov, _monster, mpos, name) in (&fov, &monster, &mut mpos, &name).join() {
            if fov.visible_tiles.contains(&ppos.position) {
                println!("{} at {},{} shouts insults!", name.name, mpos.x, mpos.y);
                log.add_log((LogType::Monster, format!("{} shouts insults", name.name)));
                if ppos.dijkstra_map.is_empty() {
                    let mut dmap = DijkstraMap::new();
                    ppos.dijkstra_map = dmap.create(ppos.position.x, ppos.position.y, &map);
                }; 
                let new_pos = ppos.get_next_position(mpos.x, mpos.y);
                mpos.x = new_pos.x;
                mpos.y = new_pos.y;
            }
        }
    }
}

const DIJKSTRA_MAX: i32 = 1000;

struct DijkstraMap {
    range: i32,
    tiles: Vec<Position>,
    values: Vec<i32>,
    neighbours: Vec<Vec<Position>>,
}

impl DijkstraMap {
    fn new() -> Self {
        Self {
            range: 7,
            tiles: Vec::new(),
            values: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    fn create(&mut self, x: i32, y: i32, map: &Map) -> Vec<(Position, i32, Vec<Position>)> {
        for i in 0..=self.range {
            for a in [-1, 1].iter().cloned() {
                for j in 0..=self.range {
                    for b in [-1, 1].iter().cloned() {
                        if (i == 0 && a == -1) || (j == 0 && b == -1) {
                            continue;
                        }
                        let xp = x+a*i;
                        let yp = y+b*j;
                        if xp < 0 || xp >= map.width || yp < 0 || yp >= map.height {
                            continue;
                        }
                        let p = Position { x: xp, y: yp };  
                        self.neighbours.push(self.find_neighbours(&p, map.width, map.height));
                        self.tiles.push(p);
                        self.values.push(DIJKSTRA_MAX);
                    };
                };
            };
        };

        let zero_pos = self.tiles.iter()
            .position(|t| t.x == x && t.y == y)
            .unwrap();
        self.values[zero_pos] = 0;

        let mut dijkstra_map = Vec::new();

        for i in 0..self.tiles.len() - 1 {
            let tx = self.tiles[i].x;
            let ty = self.tiles[i].y;
            if map.tiles[map.xy_idx(tx, ty)] != TileType::Wall {
                let mut neighbour_dvs = Vec::new();
                let neighbours: Vec<Position> = self.neighbours[i]
                    .clone()
                    .into_iter()
                    .filter(|n| map.tiles[map.xy_idx(n.x, n.y)] != TileType::Wall)
                    .collect();
                for n in neighbours.iter() {
                    neighbour_dvs.push(self.get_dijkstra_value(&n));
                };
                let min = neighbour_dvs.iter().min();
                let m = match min {
                    Some(v) => *v,
                    None => DIJKSTRA_MAX,
                };
                if self.values[i] > m + 1 {
                    self.values[i] = m + 1;
                };
                dijkstra_map.push((self.tiles[i], self.values[i], neighbours));
            }
        };
        dijkstra_map
    }

    fn get_dijkstra_value(&self, p: &Position) -> i32 {
        let idx = self.tiles.iter().position(|t| t.x == p.x && t.y == p.y);
        match idx {
            Some(i) => self.values[i],
            None => DIJKSTRA_MAX,
        }
    }

    fn find_neighbours(&self, p: &Position, w: i32, h: i32) -> Vec<Position> {
        let mut neighbours = Vec::new();
        for r in [-1, 0, 1].iter().cloned() {
            for c in [-1, 0, 1].iter().cloned() {
                if r == 0 && c == 0 {
                    continue;
                }
                neighbours.push(Position { x: p.x + r, y: p.y + c });
            }
        };
        neighbours.retain(|n| n.x >= 0 && n.x < w && n.y >= 0 && n.y < h);
        neighbours
    }
}

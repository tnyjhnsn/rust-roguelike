use specs::prelude::*;
use super::{
    CombatStats,
    Player,
    Renderable,
    Name,
    Position,
    FieldOfView,
    Monster,
    Item,
    Potion,
    BlocksTile};
use rand::Rng;

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Player{})
        .with(Name { name: "You".to_string() })
        .with(Position { x, y })
        .with(Renderable { glyph: String::from("player-m") })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
        })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32, i: usize) {
    let mut rng = rand::thread_rng();
    let glyph;
    let name;
    let roll = rng.gen_range(1, 5);
    match roll {
        1 => { glyph = String::from("white-centipede"); name = "Carnivorous White Centipede".to_string(); }
        2 => { glyph = String::from("red-ant"); name = "Giant Red Ant".to_string(); }
        3 => { glyph = String::from("ghost"); name = "Scary Ghost".to_string(); }
        _ => { glyph = String::from("grey-mould"); name = "Grey Mould".to_string(); }
    }
    ecs
        .create_entity()
        .with(Monster{})
        .with(Name { name: format!("{} #{}", &name, i) })
        .with(Position { x, y })
        .with(Renderable { glyph })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
        })
        .with(BlocksTile{})
        .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Name { name: "Health Potion".to_string() })
        .with(Position{ x, y })
        .with(Renderable{ glyph: String::from("health-potion") })
        .with(Item{})
        .with(Potion{ heal: 8 })
        .build();
}


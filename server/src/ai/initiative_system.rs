use specs::prelude::*;
use crate:: {
    Initiative,
    Position,
    MyTurn,
    Attributes,
    GameState,
    RandomNumberGenerator,
    PlayerEntity,
    PlayerPosition,
};

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    type SystemData = (
        WriteStorage<'a, Initiative>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, MyTurn>,
        Entities<'a>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadStorage<'a, Attributes>,
        WriteExpect<'a, GameState>,
        ReadExpect<'a, PlayerEntity>,
        ReadExpect<'a, PlayerPosition>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut initiatives, positions, mut turns, entities, mut rng,
             attributes, mut game_state, player_entity, ppos) = data;

        if *game_state != GameState::Ticking { return; }

        turns.clear();

        for (entity, initiative, pos) in (&entities, &mut initiatives, &positions).join() {
            initiative.current -= 1;
            if initiative.current < 1 {
                let mut myturn = true;
                initiative.current = rng.roll_dice(1, 6, 6);
                if let Some(attr) = attributes.get(entity) {
                    initiative.current -= attr.quickness.bonus;
                }
                if entity == *player_entity {
                    *game_state = GameState::Waiting;
                } else {
                    let d = ppos.position.distance(*pos);
                    if d > 20.0 {
                        myturn = false;
                    }
                }
                if myturn {
                    turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
                }
            }
        }
    }
}

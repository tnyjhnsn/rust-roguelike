use specs::prelude::*;
use crate::{
    MyTurn,
    Confusion,
    GameState,
};

pub struct TurnStatusSystem {}

impl<'a> System<'a> for TurnStatusSystem {
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, Confusion>,
        Entities<'a>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut confusion, entities, game_state) = data;

        if *game_state != GameState::Ticking { return; }

        let mut not_my_turn = Vec::new();
        let mut not_confused = Vec::new();

        for (entity, _turn, confused) in (&entities, &mut turns, &mut confusion).join() {
            confused.turns -= 1;
            if confused.turns < 1 {
                not_confused.push(entity);
            } else {
                not_my_turn.push(entity);
            }
        }
        for e in not_my_turn {
            turns.remove(e);
        }
        for e in not_confused {
            confusion.remove(e);
        }
    }
}

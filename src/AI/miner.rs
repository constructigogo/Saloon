use bevy::math::DVec2;
use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{Destination, DestoType, GalaxyCoordinate, SimPosition, to_system};
use crate::space::anomalies::AnomalyMining;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToAnom;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Mine;

pub fn move_to_anom_system(
    mut ship: Query<(Entity, &GalaxyCoordinate, &SimPosition, &mut Destination)>,
    anoms: Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining)>,
    mut action: Query<(&Actor, &MoveToAnom, &mut ActionState)>,
) {
    for (Actor(actor), order, mut state) in action.iter_mut() {
        let (id, coord, pos, mut desto) = ship.get_mut(*actor).unwrap();
        match *state {
            ActionState::Requested => {
                let closest = get_closest_anom_pos(
                    coord,
                    pos,
                    &anoms
                );
                if  (pos.0.truncate() - closest.truncate()).length() > 0.00005{
                    *desto = Destination(DestoType::DPosition(closest.0.truncate()));
                    println!("action on {:?}, from {:?}, setting desto to {:?}",actor, pos.0.truncate(),closest.0.truncate());
                    *state = ActionState::Executing;
                }
            }
            ActionState::Executing => {
                match desto.0 {
                    DestoType::DPosition(id)=> {
                        if (pos.0.truncate() - id).length() < 0.00005{
                            //*state = ActionState::Success;
                        }
                    }
                    DestoType::TEntity(id) => {
                        if (pos.0 - id.0).length() < to_system(30.0){
                            //*state = ActionState::Success;
                        }
                    }
                    DestoType::None => {}
                }
                //let dist = pos.0 - des;
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn mine_scorer_system(
    mut query: Query<(&Actor, &mut Score), With<Mine>>,
){
    for (Actor(actor),mut score) in query.iter_mut() {
        score.set(1.0);
    }
}





fn get_closest_anom_pos(
    in_coord: &GalaxyCoordinate,
    at_pos: &SimPosition,
    anoms: &Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining)>,
) -> SimPosition {
    *(anoms
        .iter()
        .filter(|x| x.1.0 == in_coord.0)
        .min_by(|a, b| {
            let da = (a.2.0 - at_pos.0).length_squared();
            let db = (b.2.0 - at_pos.0).length_squared();
            da.partial_cmp(&db).unwrap()
        })).expect("none found").2
}
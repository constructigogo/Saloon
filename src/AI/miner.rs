use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use big_brain::prelude::*;

use crate::{Destination, DestoType, DVec3, GalaxyCoordinate, Inventory, SimPosition, to_system, WeaponTarget};
use crate::space::anomalies::{AnomalyActive, AnomalyMining};
use crate::space::asteroid::AsteroidTag;
use crate::space::galaxy::around_pos;
use crate::space::inventory::OnboardInventory;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToAnom;

#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct MiningInAnom(pub Entity);

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MineAnom;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct DepositOre;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Mine;

pub fn move_to_anom_system(
    mut command: Commands,
    mut ship: Query<(Entity, &GalaxyCoordinate, &SimPosition, &mut Destination)>,
    anoms: Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining), With<AnomalyActive>>,
    mut action: Query<(&Actor, &MoveToAnom, &mut ActionState)>,
) {
    for (Actor(actor), order, mut state) in action.iter_mut() {
        let (id, coord, pos, mut desto) = ship.get_mut(*actor).unwrap();
        match *state {
            ActionState::Requested => {
                let (closest, anom) = get_closest_anom_pos(
                    coord,
                    pos,
                    &anoms,
                );

                match anom {
                    None => {
                        println!("no anom found, failure");
                        *state = ActionState::Failure;
                    }
                    Some(anom_value) => {
                        command.entity(id).insert((MiningInAnom(anom_value)));
                        if (pos.0.truncate() - closest.truncate()).length() > 0.00005 {
                            *desto = Destination(
                                DestoType::DPosition(around_pos(closest, 15.0))
                            );
                            //println!("action on {:?}, from {:?}, setting desto to {:?}",actor, pos.0.truncate(),closest.0.truncate());
                            *state = ActionState::Executing;
                        }
                    }
                }

            }
            ActionState::Executing => {
                match desto.0 {
                    DestoType::DPosition(target_pos) => {
                        if (pos.0.truncate() - target_pos.0.truncate()).length() < to_system(30.0) {
                            //println!("Success");
                            *state = ActionState::Success;
                        }
                    }
                    DestoType::TEntity(id) => {
                        if (pos.0 - id.0).length() < to_system(30.0) {
                            //println!("Success");
                            *state = ActionState::Success;
                        }
                    }
                    DestoType::None => {}
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn mine_anom_system(
    mut ship: Query<(Entity, &GalaxyCoordinate, &SimPosition,&OnboardInventory, &MiningInAnom, &mut WeaponTarget, &mut Destination)>,
    anoms: Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining)>,
    asteroid: Query<(Entity, &SimPosition), With<AsteroidTag>>,
    inventories: Query<&Inventory>,
    mut action: Query<(&Actor, &MineAnom, &mut ActionState)>,
) {
    for (Actor(actor), order, mut state) in action.iter_mut() {
        let (id, coord, pos, inv, anom, mut target, mut desto) = ship.get_mut(*actor).unwrap();
        match *state {
            ActionState::Requested => {
                let result =
                    get_closest_asteroid_in_anom(
                        &pos,
                        anom.0,
                        &asteroid,
                        &anoms);

                match result.1 {
                    None => {
                        println!("could not find asteroid in anom {:?}",anom.0);
                        *state = ActionState::Failure;
                    }
                    Some(asteroid_id) => {
                        println!("found asteroid, moving to {:?}",asteroid_id);
                        target.0 = Some(asteroid_id);
                        desto.0 = DestoType::DPosition(around_pos(result.0, 15.0));
                        *state = ActionState::Executing;
                    }
                }
            }
            ActionState::Executing => {
                let inv_ref = inventories.get(inv.0).unwrap();
                match inv_ref.max_volume {
                    None => {}
                    Some(max_vol) => {
                        if inv_ref.cached_current_volume > 0.95*max_vol {
                            println!("cargo full");
                            *state = ActionState::Success;
                        }
                    }
                }
                match target.0 {
                    None=> {
                        *state = ActionState::Requested;
                    }
                    Some(_) => {

                    }
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn deposit_ore_action_system(){

}

pub fn mine_scorer_system(
    anoms: Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining)>,
    asteroids: Query<(Entity, &SimPosition), (With<AsteroidTag>)>,
    mut query: Query<(&Actor, &mut Score), With<Mine>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        score.set(1.0);
    }
}


fn get_closest_anom_pos(
    in_coord: &GalaxyCoordinate,
    at_pos: &SimPosition,
    anoms: &Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining), With<AnomalyActive>>,
) -> (SimPosition, Option<Entity>) {
    let res = (anoms
        .iter()
        .filter(|x| x.1.0 == in_coord.0)
        .min_by(|a, b| {
            let da = (a.2.0 - at_pos.0).length_squared();
            let db = (b.2.0 - at_pos.0).length_squared();
            da.partial_cmp(&db).unwrap()
        }));
    match res {
        Some(result) => {
            return (*result.2, Some(result.0));

        }
        None =>{
            return (SimPosition(DVec3::ZERO), None);
        }
    }

}

fn get_closest_asteroid_in_anom(
    at_pos: &SimPosition,
    anom_id: Entity,
    asteroids: &Query<(Entity, &SimPosition), With<AsteroidTag>>,
    anoms: &Query<(Entity, &GalaxyCoordinate, &SimPosition, &AnomalyMining)>,
) -> (SimPosition, Option<Entity>) {
    let res: (SimPosition, Option<Entity>);
    let get_anom = anoms.get(anom_id).unwrap();

    let mut dist = f64::MAX;
    let mut closest_pos = DVec3::ZERO;
    let mut closest_id: Option<Entity> = None;

    for id in get_anom.3.tracked.iter() {
        let asteroid_ref = asteroids.get(*id);
        match asteroid_ref {
            Ok(asteroid) => {
                let _current_dist = (at_pos.0 - asteroid.1.0).length();

                if _current_dist < dist {
                    dist = _current_dist;
                    closest_pos = asteroid.1.0;
                    closest_id = Some(*id);
                }
            }
            Err(_) => {}
        }
    }
    res = (SimPosition(closest_pos), closest_id);
    return res;
}
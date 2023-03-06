use std::cmp::max;
use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;

use crate::{Inventory, ItemType, RessourceWell, spawn_item, TransferItemOrder, WeaponInRange, WeaponTarget};
use crate::space::inventory::{is_type_in_inventory, is_type_in_inventory_mut, Item, OnboardInventory, WorldInventory};
use crate::space::weapon::WeaponStats;

#[derive(Component)]
pub struct ResourceGatherer;

pub fn resource_gathering_system(
    mut commands: Commands,
    world_inv: Res<WorldInventory>,
    mut ships: Query<(Entity, &OnboardInventory, &mut WeaponStats, &mut WeaponTarget, &WeaponInRange), With<ResourceGatherer>>,
    mut wells: Query<(&mut RessourceWell)>,
    mut inventories: Query<(&mut Inventory)>,
    mut items: Query<(&mut Item)>,
) {
    for (id, inv_id,mut weapon, mut target, in_range) in ships.iter_mut() {
        if weapon.reload.finished() {
            if in_range.0 {
                match target.0 {
                    None => {}
                    Some(target_id) => {
                        let well = wells.get_mut(target_id);
                        let inv_ref = inventories.get_mut(inv_id.0).expect("Entity should have inventory");

                        match well {
                            Ok(mut well_ref) => {
                                match inv_ref.max_volume {
                                    None => {
                                        panic!("uncapped volume not allowed to mine");
                                    }
                                    Some(value) => {
                                        let available = value - inv_ref.cached_current_volume;
                                        let res_yield = weapon.damage.min(available).min(well_ref.volume);//.max(well_ref.volume);

                                        let in_inv =
                                            is_type_in_inventory_mut(
                                                &well_ref._type,
                                                &inv_ref,
                                                &items,
                                            );

                                        match in_inv {
                                            None => {
                                                commands.spawn((
                                                    spawn_item(
                                                        id,
                                                        ItemType::ORE,
                                                        res_yield,
                                                    ),
                                                    TransferItemOrder {
                                                        from: world_inv.0,
                                                        to: inv_id.0,
                                                    }
                                                ));
                                                well_ref.volume -= res_yield;
                                            }
                                            Some(item_id) => {
                                                let mut item_ref = items.get_mut(item_id).expect("item should exist");
                                                item_ref.volume += res_yield;
                                                well_ref.volume -= res_yield;
                                            }
                                        }

                                        println!("mined {:?} ore, left {:?}",res_yield,well_ref.volume);
                                    }
                                }
                            }
                            Err(_) => {
                                target.0 = None;
                            }
                        }
                        weapon.reload.reset();
                    }
                }
            }
        }
    }
}
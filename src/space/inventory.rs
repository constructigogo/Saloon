use std::borrow::{Borrow, BorrowMut};
use std::io::empty;
use std::process::id;
use std::thread::spawn;

use bevy::ecs::query;
use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::utils::tracing::callsite::register;

#[derive(Component)]
pub struct OnboardInventory(pub Entity);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterInventoryTo(pub Entity);


#[derive(Component)]
pub struct Inventory {
    pub owner: Entity,
    pub location: Entity,
    pub container: Vec<Entity>,
    pub max_volume: Option<f32>,
    pub cached_current_volume: f32,
}

#[derive(Bundle)]
pub struct InventorySpawnBundle {
    pub inv: Inventory,
    pub register: RegisterInventoryTo,
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct UpdateCachedVolume;


#[derive(Component)]
pub struct Item {
    pub owner: Entity,
    pub _type: ItemType,
    pub volume: f32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ItemType {
    ORE,
}

#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct TransferItemOrder {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
}

#[derive(Resource)]
pub struct WorldInventory(pub Entity);

pub fn setup_world_inventory(
    mut commands: Commands
) {
    let world_inv = commands.spawn_empty().id();

    commands.entity(world_inv).insert(
        Inventory {
            owner: world_inv,
            location: world_inv,
            container: Vec::new(),
            max_volume: None,
            cached_current_volume: 0.0,
        },
    );
    commands.insert_resource(WorldInventory(world_inv));
}

pub fn spawn_inventory(target: Entity) -> InventorySpawnBundle {
    return InventorySpawnBundle {
        inv: Inventory {
            owner: target,
            location: target,
            container: vec![],
            max_volume: None,
            cached_current_volume: 0.0,
        },
        register: RegisterInventoryTo(target),
    };
}

pub fn spawn_item(target: Entity, __type: ItemType, _volume: f32) -> ItemBundle {
    return ItemBundle {
        item: Item {
            owner: target,
            _type: __type,
            volume: _volume,
        },
    };
}


pub fn register_inventory_to_ship_system(
    mut commands: Commands,
    registers: Query<(Entity, &RegisterInventoryTo)>,
) {
    for (id, register) in registers.iter() {
        //println!("init inv for {:?}", register.0);
        commands.entity(register.0)
            .insert(OnboardInventory(id));
        commands.entity(id).remove::<RegisterInventoryTo>();
    }
}

pub fn transfer_item(
    mut commands: Commands,
    mut inv_query: Query<(&mut Inventory)>,
    mut check_inv: Query<&mut Item>,
    mut query: Query<(Entity, &TransferItemOrder)>) {
    for (entity, order) in query.iter() {
        let [mut inv_from, mut inv_to] = inv_query.many_mut([order.from, order.to]);


        let item = check_inv.get(entity).unwrap();
        let _type = &item._type;
        let vol: f32 = item.volume;

        let in_inv = is_type_in_inventory_mut(
            _type,
            &inv_to,
            &check_inv,
        );

        let to_volume: f32 = volume_in_inventory_mut(&mut inv_to, &check_inv);

        let mut empty_space: Option<f32> = None;

        match inv_to.max_volume {
            None => {}
            Some(val) => { empty_space = Some(val - to_volume) }
        }

        if inv_to.max_volume.is_none() ||
            (inv_to.max_volume.is_some() && to_volume + vol <= inv_to.max_volume.unwrap()) {}

        match in_inv {
            None => {
                let [mut item_mut] = check_inv.many_mut([entity]);
                match empty_space {
                    None => {
                        inv_from.container.retain(|&x| x != entity);
                        inv_to.container.push(entity);
                    }
                    Some(val_left) => {
                        if val_left >= vol {
                            inv_from.container.retain(|&x| x != entity);
                            inv_to.container.push(entity);
                        } else {
                            item_mut.volume = val_left;
                            inv_from.container.retain(|&x| x != entity);
                            inv_to.container.push(entity);
                            let new_item_from = commands.spawn((
                                spawn_item(inv_from.owner, item_mut._type.clone(), vol - val_left),
                                TransferItemOrder { from: order.to, to: order.from }
                            )).id();
                        }
                    }
                }
            }
            Some(ent) => {
                let local_vol = vol;
                let [mut item_mut, mut item_in_inv] = check_inv.many_mut([entity, ent]);

                match empty_space {
                    None => {
                        item_in_inv.volume += local_vol;
                        commands.entity(entity).despawn();
                    }
                    Some(val_left) => {
                        if val_left >= vol {
                            item_in_inv.volume += vol;
                            commands.entity(entity).despawn();
                        } else {
                            let diff: f32 = vol - val_left;
                            item_mut.volume -= diff;
                            item_in_inv.volume += diff;
                        }
                    }
                }
            }
        }
        commands.entity(order.to).insert((UpdateCachedVolume));
        commands.entity(order.from).insert((UpdateCachedVolume));
        commands.entity(entity).remove::<TransferItemOrder>();
    }
}


pub fn update_cached_volume_system(
    mut commands: Commands,
    mut inventory: Query<(Entity, &mut Inventory), With<UpdateCachedVolume>>,
    mut check_inv: Query<&mut Item>,
) {
    for (id, mut inv) in inventory.iter_mut() {
        let result = volume_in_inventory_mut(&mut inv, &check_inv);
        //assert!(Some(inv.max_volume>Some(result));
        //println!("{:?}/{:?}", result, inv.max_volume);
        inv.cached_current_volume = result;
        commands.entity(id).remove::<UpdateCachedVolume>();
    }
}


fn volume_in_inventory_mut(inv: &mut Mut<Inventory>,
                           query: &Query<(&mut Item)>,
) -> f32 {
    let mut total: f32 = 0.0;
    let mut flagRemove: Vec<Entity> = Vec::new();
    for item in inv.container.iter() {
        let _itm = query.get(*item);
        match _itm {
            Ok(itm_id) => {
                total += itm_id.volume;
            }
            Err(_) => {
                flagRemove.push(*item);
            }
        }
    }

    for item in flagRemove {
        inv.container.retain(|&x| x != item);
    }

    //debug_assert!(inv.max_volume > Some(total));
    return total;
}

pub fn owner_has_inventory_in_station() {}

pub fn is_type_in_inventory(
    item_type: &ItemType,
    inv: &Inventory,
    query: &Query<(&Item)>,
) -> Option<Entity> {
    for item in inv.container.iter() {
        let _itm = query.get(*item);
        match _itm {
            Ok(itm_id) => {
                let _type = &itm_id._type;
                if *_type == *item_type {
                    return Some(*item);
                }
            }
            Err(_) => {}
        }
    }
    return None;
}

pub fn is_type_in_inventory_mut(
    item_type: &ItemType,
    inv: &Mut<Inventory>,
    query: &Query<(&mut Item)>,
) -> Option<Entity> {
    for item in inv.container.iter() {
        let _itm = query.get(*item);
        match _itm {
            Ok(itm_id) => {
                let _type = &itm_id._type;
                if *_type == *item_type {
                    return Some(*item);
                }
            }
            Err(_) => {}
        }
    }
    return None;
}


pub fn debug_items(
    keys: Res<Input<KeyCode>>,
    query: Query<(Entity, &Item)>,
    inv_q: Query<&Inventory>,
) {
    if keys.just_pressed(KeyCode::X) {
        for (ent, item) in query.iter() {
            println!("{:?} type {:?}, vol {:?},", ent, item._type, item.volume)
        }
        for item in inv_q.iter() {
            println!("{:?} {:?} {:?}", item.owner, item.location, item.container);
        }
    }
}
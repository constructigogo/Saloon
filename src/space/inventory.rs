use std::borrow::{Borrow, BorrowMut};
use std::io::empty;

use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub owner: Entity,
    pub location: Entity,
    pub container: Vec<Entity>,
    pub max_volume: Option<f32>,
}

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


pub fn spawn_item(target: Entity, __type: ItemType, _volume: f32) -> ItemBundle {
    return ItemBundle {
        item: Item {
            owner: target,
            _type: __type,
            volume: _volume,
        },
    };
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

        let in_inv = is_type_in_inventory(
            _type,
            &inv_to,
            &check_inv,
        );

        let to_volume: f32 = volume_in_inventory(&inv_to, &check_inv);

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

                println!("not found, entity is {:?}", inv_to.owner);
                match empty_space {
                    None => {
                        println!("no inv limit");
                        inv_from.container.retain(|&x| x != entity);
                        inv_to.container.push(entity);
                    }
                    Some(val_left) => {
                        println!("available space : {:?}", val_left);
                        if val_left >= vol {
                            println!("Enough space");
                            inv_from.container.retain(|&x| x != entity);
                            inv_to.container.push(entity);
                        } else {
                            println!("not enough space, need {:?}, have {:?}", vol, val_left);
                            item_mut.volume = val_left;
                            inv_from.container.retain(|&x| x != entity);
                            inv_to.container.push(entity);


                            //commands.entity(entity).despawn();
                            let new_item_from = commands.spawn((
                                spawn_item(inv_from.owner, item_mut._type.clone(), vol - val_left),
                                TransferItemOrder { from: order.to, to: order.from }
                            )).id();
                            //inv_to.container.push(new_item_to);
                        }
                    }
                }
            }
            Some(ent) => {
                println!("found in inv");
                let local_vol = vol;
                let [mut item_mut, mut item_in_inv] = check_inv.many_mut([entity, ent]);

                match empty_space {
                    None => {
                        item_in_inv.volume += local_vol;
                        commands.entity(entity).despawn();
                    }
                    Some(val_left) => {
                        println!("can add {:?}, need {:?}", val_left, vol);
                        if val_left >= vol {
                            //item_mut.volume -= vol;
                            item_in_inv.volume += vol;
                            commands.entity(entity).despawn();
                        } else {
                            let diff :f32 = vol-val_left;
                            item_mut.volume -= diff;
                            item_in_inv.volume += diff;
                        }
                    }
                }
            }
        }
        commands.entity(entity).remove::<TransferItemOrder>();
    }
}

fn volume_in_inventory(inv: &Mut<Inventory>,
                       query: &Query<(&mut Item)>,
) -> f32 {
    let mut total: f32 = 0.0;
    for item in inv.container.iter() {
        let _itm = query.get(*item).unwrap();
        total += _itm.volume;
    }
    //debug_assert!(inv.max_volume > Some(total));
    return total;
}


fn is_type_in_inventory(
    item_type: &ItemType,
    inv: &Mut<Inventory>,
    query: &Query<(&mut Item)>,
) -> Option<Entity> {
    for item in inv.container.iter() {
        let _itm = query.get(*item).unwrap();
        let _type = &_itm._type;
        if *_type == *item_type {
            return Some(*item);
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
            println!("{:?} {:?}", item.owner, item.container);
        }
    }
}
use std::borrow::{Borrow, BorrowMut};
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

#[derive(Debug, Eq, PartialEq)]
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


pub fn spawn_item(target: Entity) -> ItemBundle {
    return ItemBundle {
        item: Item {
            owner: target,
            _type: ItemType::ORE,
            volume: 10.0,
        },
    };
}

pub fn transfer_item(
    mut commands: Commands,
    mut inv_query: Query<(&mut Inventory)>,
    mut check_inv: Query<&mut Item>,
    mut query: Query<(Entity, &TransferItemOrder)>) {
    for (entity, order) in query.iter() {
        let item = check_inv.get(entity).unwrap();

        let [mut inv_from,mut inv_to] = inv_query.many_mut([order.from,order.to]);

        //let mut inv_from = inv_query.get_mut(order.from).unwrap();
        //let mut inv_to = inv_query.get_mut(order.to).unwrap();
        let in_inv = is_type_in_inventory(
            &item._type,
            &inv_to,
            &check_inv,
        );



        match in_inv {
            None => {
                println!("not found, ent is {:?}", entity);
                inv_from.container.retain(|&x| x != entity);
                inv_to.container.push(entity);
            }
            Some(ent) => {
                println!("found in inv");
                let vol : f32 = item.volume;
                if let Ok(mut item_in_inv) = check_inv.get_mut(ent) {
                    item_in_inv.volume += vol;
                    commands.entity(entity).despawn();
                }
            }
        }


        commands.entity(entity).remove::<TransferItemOrder>();
    }
}

fn is_type_in_inventory(
    item_type : &ItemType,
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
    query: Query<(&Item)>,
    inv_q: Query<&Inventory>,
) {
    if keys.just_pressed(KeyCode::X) {
        for item in query.iter() {
            println!("owner {:?}, item {:?}, vol {:?},", item.owner, item._type, item.volume)
        }
        for item in inv_q.iter() {
            //println!("{:?}",item.container);
        }
    }
}
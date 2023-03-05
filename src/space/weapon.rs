use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;
use crate::{SimPosition, to_system};

pub enum WeaponType{
    Mining,
    Laser,
    Projectile,
    Kinetic,
    Missile
}

pub enum WeaponConfig{
    RangeShort,
    RangeLong
}

pub enum WeaponSize{
    Small,
    Medium,
    Large,
    Capital,
}

#[derive(Component, Deref, DerefMut)]
pub struct WeaponTarget(pub Option<Entity>);

#[derive(Component)]
pub struct Weapon{
    pub _type : WeaponType,
    pub config : WeaponConfig,
    pub size : WeaponSize,
    pub tier : i32,
    pub bank : i32
}

#[derive(Component, Deref, DerefMut)]
pub struct WeaponInRange(bool);

pub fn weapon_range_checker_system(
    targetpos : Query<&SimPosition>,
    mut weapon : Query<(&SimPosition, &mut WeaponInRange, &WeaponTarget)>
){
    for (pos,mut inrange, target) in weapon.iter_mut() {
        match target.0 {
            Some(ent) => {
                let target_pos = targetpos.get(ent) ;
                match target_pos {
                    Ok(t_pos) => {
                        inrange.0 = (t_pos.0 - pos.0).length_squared() < to_system(15.0*15.0)
                    }
                    Err(_) => {}
                }
            }
            None => {}
        }

    }
}
use std::time::Duration;

use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;

use crate::{SimPosition, to_system};
use crate::space::weapon::mining::ResourceGatherer;

pub mod mining;

pub enum WeaponType {
    Mining,
    Laser,
    Projectile,
    Kinetic,
    Missile,
}

pub enum WeaponConfig {
    RangeShort,
    RangeLong,
}

pub enum WeaponSize {
    Small,
    Medium,
    Large,
    Capital,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub _weapon: Weapon,
    pub target: WeaponTarget,
    pub in_range: WeaponInRange,
}

#[derive(Component, Deref, DerefMut)]
pub struct WeaponTarget(pub Option<Entity>);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Weapon {
    pub _type: WeaponType,
    pub config: WeaponConfig,
    pub size: WeaponSize,
    pub tier: i32,
    pub bank: i32,
}

#[derive(Component)]
pub struct WeaponStats {
    pub damage: f32,
    pub range: f64,
    pub reload: Timer,
    pub bank: i32,
}


#[derive(Component, Deref, DerefMut)]
pub struct WeaponInRange(pub bool);

pub fn weapon_range_checker_system(
    targetpos: Query<&SimPosition>,
    mut weapon: Query<(&SimPosition, &mut WeaponInRange, &WeaponStats, &mut WeaponTarget)>,
) {
    for (pos, mut inrange, stats, mut target) in weapon.iter_mut() {
        match target.0 {
            Some(ent) => {
                let target_pos = targetpos.get(ent);
                match target_pos {
                    Ok(t_pos) => {
                        inrange.0 = (t_pos.0 - pos.0).length_squared() < to_system(15.0 * 15.0)
                    }
                    Err(_) => { target.0 = None }
                }
            }
            None => {}
        }
    }
}

pub fn weapon_cooldown_ticking_system(
    time: Res<Time>,
    mut weapons: Query<&mut WeaponStats>,
) {
    for mut weapon in weapons.iter_mut() {
        if !weapon.reload.finished() {
            //println!("running {:?}",weapon.reload.elapsed());
            weapon.reload.tick(time.delta());
        }
    }
}

pub fn weapon_init_system(
    mut commands: Commands,
    mut weapons: Query<(Entity, &Weapon), Or<(Added<Weapon>, Changed<Weapon>)>>,
) {
    for (id, weapon) in weapons.iter() {
        println!("init weapon for {:?}", id);
        let _damage: f32;
        let _damage_mult: f32;
        let _range: f64;
        let _range_mult: f64;
        let _reload: Timer;
        let _reload_mult: f32;

        match weapon._type {
            WeaponType::Mining => {
                commands.entity(id).insert((ResourceGatherer));
                _damage = 50.0;
            }
            _ => {
                commands.entity(id).remove::<ResourceGatherer>();
                _damage = 5.0;
            }
        }

        match weapon.config {
            WeaponConfig::RangeShort => {
                _range_mult = 0.6;
                _damage_mult = 1.2;
            }
            WeaponConfig::RangeLong => {
                _range_mult = 1.4;
                _damage_mult = 0.8;
            }
        }

        let result = WeaponStats {
            damage: _damage,
            range: to_system(35.0),
            reload: Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ),
            bank: weapon.bank,
        };

        commands.entity(id).insert((result));
    }
}
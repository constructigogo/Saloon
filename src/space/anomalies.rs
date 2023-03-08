use std::process::id;
use std::time::Duration;

use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;

use crate::{DVec3, GalaxyCoordinate, ItemType, RessourceWell, SimPosition, SolarSystem, spawn_asteroid, to_system};
use crate::space::asteroid::AsteroidTag;
use crate::space::galaxy::{DisplayableGalaxyEntityBundle, GalaxyEntityBundle};



#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterTo(pub Entity);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AnomalyInit;

#[derive(Component)]
pub struct Anomaly{
    pub level : i32,
}

#[derive(Component)]
pub struct AnomalyMining{
    pub tracked : Vec<Entity>,
}

#[derive(Component)]
pub struct AnomalyCombat;


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AnomalyActive;

#[derive(Component)]
pub struct AnomalyRespawnTimer(pub Timer);

#[derive(Bundle)]
pub struct AnomalyBundle {
    pub anom : Anomaly,
    pub timer : AnomalyRespawnTimer,
    pub displayable: DisplayableGalaxyEntityBundle,
    pub init : AnomalyInit
}


pub fn spawn_anom(at: SimPosition, galaxy: Entity) -> AnomalyBundle {
    return AnomalyBundle {
        anom : Anomaly{ level: 1 },
        timer: AnomalyRespawnTimer(Timer::new(
            Duration::from_secs(5),
            TimerMode::Repeating,
        )),
        displayable: DisplayableGalaxyEntityBundle {
            display: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.25, 0.85, 0.55, 0.2),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                ..default()
            },
            galaxy: GalaxyEntityBundle {
                galaxy_coord: GalaxyCoordinate(galaxy),
                simulation_position: at,
            },
        },
        init: AnomalyInit
    };
}


pub fn init_anom(
    mut command: Commands,
    mut anom: Query<(Entity,&GalaxyCoordinate, &SimPosition, &Anomaly, &mut AnomalyMining), With<AnomalyInit>>,
) {
    for (id,galaxy,pos,anom,mut mining) in anom.iter_mut() {
        for _ in 0..5*anom.level {
            let roid = command.spawn((
                spawn_asteroid(pos.clone(), galaxy.0),
            )).id();
            mining.tracked.push(roid);
        }
        command.entity(id).remove::<AnomalyInit>();
        command.entity(id).insert(AnomalyActive);
    }
}

pub fn mining_anomaly_lifecycle_system(
    mut commands : Commands,
    anomalies : Query<(Entity, &AnomalyMining), (Without<AnomalyInit>,With<AnomalyActive>)>,
    asteroids : Query<(Entity),(With<AsteroidTag>)>
){
    for (id,anom) in anomalies.iter() {
        let mut reset = true;
        for asteroid in anom.tracked.iter() {
            let as_ref = asteroids.get(*asteroid);
            match as_ref {
                Ok(_) => {
                    reset = false;
                }
                Err(_) => {}
            }
        }
        if reset {
            //commands.entity(id).insert(AnomalyInit);
            //println!("anom {:?} is empty, respawning ...", id);
            commands.entity(id).remove::<AnomalyActive>();
        }
    }
}

pub fn anomaly_respawn_timer_system(
    mut commands :Commands,
    time : Res<Time>,
    mut anoms : Query<(Entity, &mut AnomalyRespawnTimer), Without<AnomalyActive>>
){
    for (id,mut anom) in anoms.iter_mut() {
        anom.0.tick(time.delta());

        if anom.0.finished() {
            commands.entity(id).insert(AnomalyInit);
            anom.0.reset();
        }

    }
}

pub fn register_anom(
    mut command: Commands,
    anom: Query<(Entity, &RegisterTo),With<Anomaly>>,
    mut system: Query<&mut SolarSystem>,
) {
    for (id, order) in anom.iter() {
        let mut sys = system.get_mut(order.0).unwrap();
        sys.anomalies.push(id);
        command.entity(id).remove::<RegisterTo>();
    }
}
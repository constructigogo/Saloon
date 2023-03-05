use std::time::Duration;

use bevy::{ecs::{entity::Entities, query}, prelude::*};

use crate::{DVec3, GalaxyCoordinate, ItemType, RessourceWell, SimPosition, SolarSystem, spawn_asteroid, to_system};
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
            TimerMode::Once,
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
                RessourceWell {
                    _type: ItemType::ORE,
                    volume: 500.0,
                }
            )).id();
            mining.tracked.push(roid);
        }
        command.entity(id).remove::<AnomalyInit>();
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
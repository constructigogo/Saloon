use std::time::Duration;
use bevy::{ecs::{entity::Entities, query}, prelude::*};
use crate::{GalaxyCoordinate, SimPosition};
use crate::space::galaxy::{DisplayableGalaxyEntityBundle, GalaxyEntityBundle};


#[derive(Component)]
pub struct AnomalyMining;

#[derive(Component)]
pub struct AnomalyCombat;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AnomalyActive;

#[derive(Component)]
pub struct AnomalyRespawnTimer(pub Timer);

#[derive(Bundle)]
pub struct AnomalyBundle {
    pub timer : AnomalyRespawnTimer,
    pub displayable: DisplayableGalaxyEntityBundle
}


pub fn spawn_anom(at : SimPosition, galaxy : Entity) -> AnomalyBundle {
    return AnomalyBundle {
        timer: AnomalyRespawnTimer(Timer::new(
            Duration::from_secs(5),
            TimerMode::Once,
        )),
        displayable: DisplayableGalaxyEntityBundle {
            display: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.25, 0.85, 0.55,0.2),
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
                simulation_position: at
            },
        }
    }
}

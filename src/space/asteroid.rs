use bevy::{ecs::{entity::Entities, query}, prelude::*};

use crate::{GalaxyCoordinate, ItemType, SimPosition};
use crate::space::galaxy::{around_pos, DisplayableGalaxyEntityBundle, GalaxyEntityBundle};

#[derive(Component)]
pub struct AsteroidTag;

#[derive(Component)]
pub struct RessourceWell {
    pub _type: ItemType,
    pub volume: f32,
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub displayable: DisplayableGalaxyEntityBundle,
    pub well: RessourceWell,
    pub tag: AsteroidTag,
}

pub fn asteroid_life_cycle_system(
    mut commands: Commands,
    mut asteroids: Query<(Entity, &RessourceWell), (With<AsteroidTag>)>,
) {
    for (id, mut asteroid) in asteroids.iter_mut() {
        if asteroid.volume <= 0.0 {
            commands.entity(id).despawn();
        }
    }
}

pub fn spawn_asteroid(at: SimPosition, galaxy: Entity) -> AsteroidBundle {
    return AsteroidBundle {
        displayable: DisplayableGalaxyEntityBundle {
            display: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.31, 0.22, 0.15),
                    custom_size: Some(Vec2::new(8.0, 8.0)),
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
                simulation_position: around_pos(at, 65.0),
            },
        },
        well: RessourceWell {
            _type: ItemType::ORE,
            volume: 10000.0,
        },
        tag: AsteroidTag,
    };
}
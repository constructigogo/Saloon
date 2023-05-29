use bevy::prelude::*;
use crate::{GalaxyCoordinate, SimPosition};

#[derive(Component)]
pub struct AnchorableTag;

#[derive(Bundle)]
pub struct AnchorableBundle {
    display: SpriteBundle,
    sim_pos : SimPosition,
    galaxy_pos :GalaxyCoordinate,
    tag : AnchorableTag,
}

pub fn spawn_station_at(at : SimPosition, galaxy : Entity ) -> AnchorableBundle{
    return AnchorableBundle{
        display: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.85, 0.15),
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        sim_pos: at.clone(),
        galaxy_pos: GalaxyCoordinate(galaxy),
        tag: AnchorableTag,
    }
}
use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use crate::camera::CameraZoom;
use crate::{SimPosition, SolarSystem};
use crate::space::galaxy::Rendered;

pub fn project_to_camera(camera_zoom : Res<CameraZoom>,
    mut query : Query<(&mut Transform, &SimPosition), (With<Rendered>, Without<SolarSystem>)>){

    //println!("projecting {:?} objects ", query.iter().len());

    for (mut trans, sPos) in query.iter_mut() {
        let calc = Vec3{
            x: (sPos.0.x/ camera_zoom.0) as f32,
            y: (sPos.0.y/ camera_zoom.0) as f32,
            z: 0.0
        };
        trans.translation = calc
    }
}
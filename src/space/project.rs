use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

use crate::{SimPosition, SolarSystem};
use crate::camera::{CameraID, CameraZoom};
use crate::space::galaxy::Rendered;

pub fn project_to_camera(camera_zoom: Res<CameraZoom>,
                         camera_id: Res<CameraID>,
                         mut camera_query: Query<&Camera>,
                         mut query: Query<(&mut Transform, &SimPosition), (With<Rendered>, Without<SolarSystem>)>) {

    //println!("projecting {:?} objects ", query.iter().len());
    let got = camera_query.get_mut(camera_id.0);
    match got{
        Ok(cam) => {
            for (mut trans, sPos) in query.iter_mut() {
                let calc = Vec3 {
                    x: ((sPos.0.x / camera_zoom.0) as f32).clamp(
                        -(((cam.physical_target_size().unwrap().x -4)/2) as f32),
                        ((cam.physical_target_size().unwrap().x -4)/2) as f32),
                    y: ((sPos.0.y / camera_zoom.0) as f32).clamp(
                        -(((cam.physical_target_size().unwrap().y -4)/2) as f32),
                        ((cam.physical_target_size().unwrap().y -4)/2) as f32),
                    z: 0.0,
                };
                trans.translation = calc
            }
        }
        Err(_) => {}
    }

}
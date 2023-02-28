use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

use crate::{SimPosition, SolarSystem};
use crate::camera::{CameraID, CameraZoom};
use crate::space::galaxy::Rendered;

pub fn project_to_camera(camera_zoom: Res<CameraZoom>,
                         camera_id: Res<CameraID>,
                         camera_query: Query<(&Camera, &Transform)>,
                         mut query: Query<(&mut Transform, &SimPosition), (With<Rendered>, Without<SolarSystem>, Without<Camera>)>) {

    //println!("projecting {:?} objects ", query.iter().len());
    let got: Result<(&Camera, &Transform), QueryEntityError> = camera_query.get(camera_id.0);
    match got {
        Ok(cam) => {
            let camera: &Camera = cam.0;
            let transf: &Transform = cam.1;
            for (mut trans, sPos) in query.iter_mut() {
                let calc = Vec3 {
                    x: ((sPos.0.x / camera_zoom.0) as f32).clamp(
                        (transf.translation.x -((camera.physical_viewport_size().unwrap().x - 48) / 2) as f32),
                        (transf.translation.x +((camera.physical_viewport_size().unwrap().x - 48) / 2) as f32)),
                    y: ((sPos.0.y / camera_zoom.0) as f32).clamp(
                        (transf.translation.y -((camera.physical_viewport_size().unwrap().y - 48) / 2) as f32),
                        (transf.translation.y +((camera.physical_viewport_size().unwrap().y - 48) / 2) as f32)),
                    z: 0.0,
                };
                trans.translation = calc
            }
        }
        Err(_) => {}
    }
}
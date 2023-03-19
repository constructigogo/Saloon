use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::query::QueryEntityError;
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

use crate::{GalaxyCoordinate, SimPosition, SolarSystem};
use crate::camera::{CameraID, CameraZoom};

pub fn project_to_camera(camera_zoom: Res<CameraZoom>,
                         camera_id: Res<CameraID>,
                         camera_query: Query<(&Camera, &Transform, &SimPosition)>,
                         mut system_query: Query<(&mut Transform, &SimPosition), (Without<GalaxyCoordinate>, Without<Camera>)>,
                         mut query: Query<(&GalaxyCoordinate, &mut Transform, &SimPosition), (Without<Camera>)>,
) {

    //println!("projecting {:?} objects ", query.iter().len());
    let got: Result<(&Camera, &Transform, &SimPosition), QueryEntityError> = camera_query.get(camera_id.0);
    match got {
        Ok(cam) => {
            let camera: &Camera = cam.0;
            let transf: &Transform = cam.1;
            let pos: &DVec3 = &cam.2.0;

            for (mut trans, sPos) in system_query.iter_mut() {
                let calc = Vec3 {
                    x: (((sPos.0.x - pos.x) / (camera_zoom.0.exp() * 0.000001)) as f32),
                    y: (((sPos.0.y - pos.y) / (camera_zoom.0.exp() * 0.000001)) as f32),
                    z: 0.0,
                };
                trans.translation = calc
            }

            for (coord, mut trans, sPos) in query.iter_mut() {
                let (delta_coord,delta_pos) = system_query.get(coord.0).unwrap();
                let calc = Vec3 {
                    x: ((((delta_pos.0.x + sPos.0.x)-pos.x) / (camera_zoom.0.exp() * 0.000001)) as f32)
                        /*
                        .clamp(
                        (transf.translation.x - ((camera.physical_viewport_size().unwrap().x - 48) / 2) as f32),
                        (transf.translation.x + ((camera.physical_viewport_size().unwrap().x - 48) / 2) as f32)),
                        */,

                    y: ((((delta_pos.0.y + sPos.0.y)-pos.y) / (camera_zoom.0.exp() * 0.000001)) as f32)
                         /*
                        .clamp(
                        (transf.translation.y - ((camera.physical_viewport_size().unwrap().y - 48) / 2) as f32),
                        (transf.translation.y + ((camera.physical_viewport_size().unwrap().y - 48) / 2) as f32)),
                        */,
                    z: 1.0,
                };
                trans.translation = calc
            }
        }
        Err(_) => {}
    }
}
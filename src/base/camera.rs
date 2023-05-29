use std::cmp::{max, min};
use std::process::id;
use std::slice::Windows;

use bevy::{input::{ButtonState, keyboard::KeyboardInput, mouse::MouseMotion}, prelude::*};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::DVec2;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::PickingCameraBundle;

use crate::{Anomaly, DVec3, GalaxyCoordinate, GalaxyGateTag, m_to_system, SimPosition, SolarSystem, SystemMap, UndockLoc};

use super::settings::{GameplaySettings, InputSettings};

#[derive(Resource)]
pub struct CameraID(pub Entity);

#[derive(Resource)]
pub struct CameraZoom(pub f64);

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(camera_input);
    }
}

fn setup(mut commands: Commands) {
    let id = commands.spawn((
        Camera2dBundle::default(),
        PickingCameraBundle::default(),
        SimPosition(DVec3::ZERO),
    )).id();

    commands.insert_resource(CameraID(id));
    commands.insert_resource(CameraZoom(23.0));
}


pub fn camera_input(time: Res<Time>,
                    windows: Query<&Window, With<PrimaryWindow>>,
                    camera_id: Res<CameraID>,
                    system_query: Query<(&SimPosition), With<SolarSystem>>,
                    mut camera_zoom: ResMut<CameraZoom>,
                    mut camera_query: Query<&mut SimPosition, (With<Camera>, Without<SolarSystem>)>,
                    poi_query: Query<(&GalaxyCoordinate, &Transform, &SimPosition), (Or<(With<Anomaly>, With<GalaxyGateTag>, With<UndockLoc>)>, Without<Camera>, Without<SolarSystem>)>,
                    keys: Res<Input<ScanCode>>,
                    settings: Res<GameplaySettings>,
                    input_settings: Res<InputSettings>,
                    mut motion_evr: EventReader<MouseMotion>,
                    mut scroll_evr: EventReader<MouseWheel>,
                    buttons: Res<Input<MouseButton>>) {
    //get the camera first before checking inputs
    let got = camera_query.get_mut(camera_id.0);
    match got {
        Ok(mut tr) => {
            //let window = windows.get_primary().unwrap();
            if let Ok(window) = windows.get_single() {

                for ev in scroll_evr.iter() {
                    let incr: f64;
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            incr = ((ev.y*settings.camera_zoom_sensitivity) / 5.0) as f64;
                            //println!("camera center : {:?}",camera_zoom.0);
                        }
                        MouseScrollUnit::Pixel => {
                            incr = 0.0;
                            println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                        }
                    }
                    if let Some(_position) = window.cursor_position() {
                        if camera_zoom.0 != 0.0 && camera_zoom.0 != 23.0 {
                            let mut cam_delta: DVec3;
                            let at = (_position - Vec2::new(window.width() / 2.0, window.height() / 2.0));
                            cam_delta = DVec3::new(at.x as f64, at.y as f64, 0.0) * m_to_system(camera_zoom.0.exp());

                            let at = (_position - Vec2::new(window.width() / 2.0, window.height() / 2.0));
                            let closest = poi_query.iter()
                                .min_by(|x, y|
                                    (x.1.translation.truncate() - at).length()
                                        .partial_cmp(&(y.1.translation.truncate() - at).length())
                                        .unwrap()
                                );
                            match closest {
                                None => {
                                    cam_delta = DVec3::new(at.x as f64, at.y as f64, 0.0) * m_to_system(camera_zoom.0.exp());
                                }
                                Some((a, b, c)) => {
                                    let sys_pos = system_query.get(a.0).unwrap();
                                    //println!("pos : {:?}", c.0);
                                    if (b.translation.truncate() - at).length() < 24.0 {
                                        cam_delta = (c.0+sys_pos.0) - tr.0;
                                    } else {
                                        cam_delta = DVec3::new(at.x as f64, at.y as f64, 0.0) * m_to_system(camera_zoom.0.exp());
                                    }
                                }
                            }


                            let ratio = ((camera_zoom.0 + incr).exp()) / camera_zoom.0.exp();

                            let lerped: DVec3 = DVec3::lerp(tr.0, tr.0 + cam_delta, 1.0 - ratio);
                            tr.0 = lerped;
                        }
                    } else {
                        // cursor is not inside the window
                    }

                    camera_zoom.0 = f64::max((camera_zoom.0 + incr), 0.0).min(23.0);
                }

            }
            else {
                return;
            };
            let mut dir = DVec3::ZERO;
            //mouse drag
            /*
            if buttons.pressed(MouseButton::Left) {
                for ev in motion_evr.iter() {
                    dir += Vec3 {
                        x: -ev.delta.x,
                        y: ev.delta.y,
                        z: 0.0,
                    }
                }
            }
            */

            //keyboard
            match input_settings.camera_up {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += DVec3::Y * m_to_system(camera_zoom.0.exp());
                    }
                }
                None => {}
            }
            match input_settings.camera_down {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += DVec3::NEG_Y * m_to_system(camera_zoom.0.exp());
                    }
                }
                None => {}
            }
            match input_settings.camera_right {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += DVec3::X * m_to_system(camera_zoom.0.exp());
                    }
                }
                None => {}
            }
            match input_settings.camera_left {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += DVec3::NEG_X * m_to_system(camera_zoom.0.exp());
                    }
                }
                None => {}
            }
            tr.0 += dir * 512.0 * time.delta_seconds_f64()
        }
        Err(_) => {}
    }
}

/*
pub fn camera_system_view(ev: EventReader<RenderSystemEvent>,
                          mut zoom: ResMut<CameraZoom>,
) {
    if !ev.is_empty() {
        zoom.0 = 23.0;
    }
}

pub fn camera_galaxy_view(ev: EventReader<RenderGalaxyEvent>,
                          camera_id: Res<CameraID>,
                          mut zoom: ResMut<CameraZoom>,
                          mut camera_query: Query<&mut SimPosition, With<Camera>>,
) {
    if !ev.is_empty() {
        zoom.0 = 1.0;
        camera_query.get_mut(camera_id.0).unwrap().0 = DVec3::ZERO;
    }
}
 */
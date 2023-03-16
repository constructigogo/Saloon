use std::cmp::max;

use bevy::{input::{ButtonState, keyboard::KeyboardInput, mouse::MouseMotion}, prelude::*};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::DVec2;
use bevy_mod_picking::PickingCameraBundle;

use crate::{DVec3, m_to_system, SimPosition};
use crate::space::galaxy::RenderSystemEvent;

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
        app.add_system(camera_system_view);
    }
}

fn setup(mut commands: Commands) {
    let id = commands.spawn((
        Camera2dBundle::default(),
        PickingCameraBundle::default(),
        SimPosition(DVec3::ZERO),
    )).id();

    commands.insert_resource(CameraID(id));
    commands.insert_resource(CameraZoom(1.0));
}


pub fn camera_input(time: Res<Time>,
                    windows: Res<Windows>,
                    mut camera_zoom: ResMut<CameraZoom>,
                    mut camera_query: Query<&mut SimPosition, With<Camera>>,
                    keys: Res<Input<ScanCode>>,
                    camera_id: Res<CameraID>,
                    settings: Res<GameplaySettings>,
                    input_settings: Res<InputSettings>,
                    mut motion_evr: EventReader<MouseMotion>,
                    mut scroll_evr: EventReader<MouseWheel>,
                    buttons: Res<Input<MouseButton>>) {

    //get the camera first before checking inputs
    let got = camera_query.get_mut(camera_id.0);
    match got {
        Ok(mut tr) => {
            let window = windows.get_primary().unwrap();
            for ev in scroll_evr.iter() {
                let incr: f64;
                match ev.unit {
                    MouseScrollUnit::Line => {
                        incr = (ev.y / 5.0) as f64;
                        //println!("camera center : {:?}",)
                    }
                    MouseScrollUnit::Pixel => {
                        incr = 0.0;
                        println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    }
                }
                if let Some(_position) = window.cursor_position() {
                    if camera_zoom.0 != 0.0 && camera_zoom.0 != 23.0 {
                        let at = (_position-Vec2::new(window.width()/2.0, window.height()/2.0));
                        let cam_delta: DVec3 = DVec3::new(at.x as f64, at.y as f64, 0.0) * m_to_system(camera_zoom.0.exp()) ;

                        let ratio = ((camera_zoom.0+incr).exp())/camera_zoom.0.exp();

                        let lerped : DVec3 = DVec3::lerp(tr.0, tr.0+ cam_delta, 1.0-ratio);
                        println!("mouse pos : {:?}, {:?}",at,ratio);
                        tr.0 = lerped;

                    }
                } else {
                    // cursor is not inside the window
                }

                camera_zoom.0 = f64::max((camera_zoom.0 + incr), 0.0).min(23.0);

            }

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

pub fn camera_system_view(ev: EventReader<RenderSystemEvent>,
                          mut zoom: ResMut<CameraZoom>,
) {
    if !ev.is_empty() {
        zoom.0 = 23.0;
    }
}
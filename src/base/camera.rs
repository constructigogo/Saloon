use std::cmp::max;

use bevy::{input::{ButtonState, keyboard::KeyboardInput, mouse::MouseMotion}, prelude::*};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_mod_picking::PickingCameraBundle;

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
        PickingCameraBundle::default()
    )).id();

    commands.insert_resource(CameraID(id));
    commands.insert_resource(CameraZoom(1.0));
}


pub fn camera_input(time: Res<Time>,
                mut camera_zoom: ResMut<CameraZoom>,
                mut camera_query: Query<&mut Transform, With<Camera>>,
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
            for ev in scroll_evr.iter() {
                match ev.unit {
                    MouseScrollUnit::Line => {
                        let incr=(ev.y / 5.0) as f64;
                        camera_zoom.0 = f64::max((camera_zoom.0 + incr), 0.0).min(23.0);
                        //println!("camera center : {:?}",)
                    }
                    MouseScrollUnit::Pixel => {
                        println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    }
                }
            }

            let mut dir = Vec3::ZERO;
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
                        dir += Vec3::Y;
                    }
                }
                None => {}
            }
            match input_settings.camera_down {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += Vec3::NEG_Y;
                    }
                }
                None => {}
            }
            match input_settings.camera_right {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += Vec3::X;
                    }
                }
                None => {}
            }
            match input_settings.camera_left {
                Some(code) => {
                    if keys.pressed(code) {
                        dir += Vec3::NEG_X;
                    }
                }
                None => {}
            }
            tr.translation += dir * 512.0 * time.delta_seconds()
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
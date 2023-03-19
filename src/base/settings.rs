use std::default;

use bevy::prelude::*;

pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameplaySettings::default());
        app.insert_resource(InputSettings::default());
    }
}


#[derive(Resource)]
pub struct GameplaySettings {
    pub camera_keyboard_sensivity: f32,
    pub camera_zoom_sensivity: f32,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            camera_keyboard_sensivity: 0.5,
            camera_zoom_sensivity: 1.6,
        }
    }
}

#[derive(Resource)]
pub struct InputSettings {
    pub camera_up: Option<ScanCode>,
    pub camera_down: Option<ScanCode>,
    pub camera_left: Option<ScanCode>,
    pub camera_right: Option<ScanCode>,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            camera_up: Some(ScanCode(17)),
            camera_down: Some(ScanCode(31)),
            camera_left: Some(ScanCode(30)),
            camera_right: Some(ScanCode(32)),
        }
    }
}

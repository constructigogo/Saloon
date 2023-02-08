use bevy::app::App;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use space::SpaceGamePlugins;
use space::ship::pilot::*;
use crate::base::*;
use crate::base::timer::*;
use crate::space::ship::*;




pub mod base;
pub mod space;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BaseLogicPlugins)
        .add_plugins(SpaceGamePlugins)

        .add_plugin(TimerPlugin)
        .add_startup_system(setup)
        //.add_system(frame_update)
        .add_system(follow_mouse)
        .run();
}

#[derive(Component)]
struct BrickTag;

#[derive(Bundle)]
struct BrickBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    tag : BrickTag,
    sprite_bundle: SpriteBundle
}



fn follow_mouse(mut query: Query<&mut Transform>,
    mut motion_evr: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>){
    if mouse_button_input.pressed(MouseButton::Left) {
        for mut transf in  &mut query {
            for ev in motion_evr.iter(){
                transf.translation += Vec3::new(ev.delta.x,-ev.delta.y,0.0);
            }
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    /* 
    // Cube
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        ..default()
    },
    Velocity(Vec2 { x: 0.0, y: -9.81 })

    ));
    */
    for _ in 0..2000 {
        commands.spawn((
            spawn_new_pilot(),
            FlagUndocking,
            )
        );
    }

}
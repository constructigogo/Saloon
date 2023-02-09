use std::default;

use bevy::app::App;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use bevy_mod_picking::*;
use rand::{thread_rng, Rng};
use space::galaxy::{SystemMap, SolarSystem, GalaxyCoordinate};
use space::ship::pilot::*;
use space::SpaceGamePlugins;

use crate::base::*;
use crate::base::timer::*;
use crate::DestoType::TEntity;
use crate::space::ship::*;

pub mod base;
pub mod space;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(SpaceGamePlugins)
        .add_plugin(TimerPlugin)
        .add_startup_system(setup)
        //.add_system(frame_update)
        //.add_system(follow_mouse)
        .run();
}

#[derive(Component)]
struct TestTag;


fn follow_mouse(mut query: Query<&mut Transform>,
                mut motion_evr: EventReader<MouseMotion>,
                mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for mut transf in &mut query {
            for ev in motion_evr.iter() {
                transf.translation += Vec3::new(ev.delta.x, -ev.delta.y, 0.0);
            }
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cluster: ResMut<SystemMap>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        PickingCameraBundle::default()
    ));
    let mut rng = thread_rng();
    for i in 0..3 {
        let id = commands.spawn((SolarSystem{
            anomalies: Vec::new(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.95, 0.25, 0.25),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: -500.0 + (500.0 * i as f32),
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        },
    )).id();

        cluster.0.push(id);

        for s in 0..10 {
            commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: rng.gen_range(-200.0..200.0),
                        y: rng.gen_range(-150.0..150.0),
                        z: 0.0,
                    },
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                ..default()
            },
            GalaxyCoordinate(id),
            ));
        }
    }

    

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
    /* 
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.80, 0.25, 0.10),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: -600.0,
                    y: 300.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        TestTag,
    ));
    
    for _ in 0..1 {
        commands.spawn((
            spawn_new_pilot(),
            FlagUndocking,
        )
        );
    }
    */
}


fn test_move(
    mut queryShips : Query<&mut Destination, Without<TestTag>>,
    queryTargets: Query<&Transform, With<TestTag>>){
    let mut t : Option<&Transform> = None;
    let mut min_dist : f32=f32::MAX;
    for tr in queryTargets.iter() {
        let dist = tr.translation.length_squared();
        if dist < min_dist {
            min_dist = dist;
            t = Some(tr)
        }
    }

    //println!("{:?}",t);

    for mut dest in &mut queryShips {
        match t {
            Some(tr) => {
                dest.0 = TEntity(*tr);
            }
            None => {}
        }

    }
}

use bevy::app::App;
use bevy::input::mouse::MouseMotion;
use bevy::math::DVec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use big_brain::prelude::*;
use rand::thread_rng;

use space::galaxy::{GalaxyCoordinate, SolarSystem, SystemMap};
use space::ship::pilot::*;
use space::SpaceGamePlugins;

use crate::AI::AIPlugins;
use crate::AI::miner::{DepositOre, Mine, MineAnom, MoveToAnom};
use crate::base::*;
use crate::base::timer::*;
use crate::DestoType::TEntity;
use crate::map::{generate_map_pathfinding_system, line_debug};
use crate::space::anomalies::*;
use crate::space::asteroid::*;
use crate::space::galaxy::*;
use crate::space::inventory::*;
use crate::space::ship::*;
use crate::space::station::*;
use crate::space::weapon::*;

pub mod base;
pub mod space;
pub mod AI;


fn main() {
    let mut app = App::new();

    app
        //.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(BigBrainPlugin)
        //.add_plugin(EditorPlugin)
        .add_plugins(DefaultPickingPlugins)
        //.add_plugin(DebugEventsPickingPlugin)
        .add_plugins(BaseLogicPlugins)
        .add_plugins(SpaceGamePlugins)
        .add_plugins(AIPlugins)
        .add_plugin(TimerPlugin)
        //.add_system(frame_update)
        .add_startup_system(generate_map_pathfinding_system)
        .add_system(line_debug)
    ;
    //.add_system(follow_mouse)

    //bevy_mod_debugdump::print_schedule(&mut app);
    //bevy_mod_debugdump::print_render_graph(&mut app);
    app.run();
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
    let mut rng = thread_rng();
    for i in 0..1 {
        let id = commands.spawn(
            (
                SolarSystem {
                    anomalies: Vec::new(),
                    gates: Vec::new(),
                },
                UndockLoc,
                SimPosition(DVec3 {
                    x: (-500.0 + (500.0 * i as f64)) * 0.00001,
                    y: 0.0,
                    z: 0.0,
                }, ),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform {
                        translation: Vec3 {
                            x: -500.0 + (500.0 * i as f32),
                            y: 0.0,
                            z: 0.0,
                        },
                        scale: Vec3 { x: 64.0, y: 64.0, z: 1.0 },
                        ..default()
                    },
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
                PickableBundle::default(),
                Rendered,
            )).remove::<Selection>().id();

        cluster.0.push(id);

        /*
        let station = commands.spawn((
            spawn_station_at(SimPosition(DVec3::ZERO), id),
            UndockLoc,
        )).id();

        commands.spawn(spawn_inventory(station));
         */

        /*
        let mid = commands.spawn((
            spawn_station_at(SimPosition(DVec3 {
                x: 150000.0,
                y: 0.0,
                z: 0.0,
            }), id),
            UndockLoc,
        )).id();

        commands.spawn(spawn_inventory(mid));
         */
        let far = commands.spawn((
            spawn_station_at(SimPosition(DVec3 {
                x: au_to_system(25.0),
                y: 0.0,
                z: 0.0,
            }), id),
            UndockLoc,
        )).id();
        commands.spawn(spawn_inventory(far));


        let anom = commands.spawn((
            spawn_anom(SimPosition(DVec3 {
                x: m_to_system(400.0),
                y: m_to_system(400.0),
                z: 0.0,
            }), id),
            AnomalyMining { tracked: Vec::new() },
            RegisterTo(id),
        )).id();

        let anom2 = commands.spawn((
            spawn_anom(SimPosition(DVec3 {
                x: m_to_system(-400.0),
                y: m_to_system(-400.0),
                z: 0.0,
            }), id),
            AnomalyMining { tracked: Vec::new() },
            RegisterTo(id),
        )).id();

        let anom_inv = commands.spawn((
            Inventory {
                owner: anom,
                location: anom,
                container: Vec::new(),
                max_volume: None,
                cached_current_volume: 0.0,
            },
            UpdateCachedVolume
        )).id();


        let first = commands.spawn((
            spawn_new_pilot(),
            UndockingFrom(far),
        )).id();

        let mut vec_inv: Vec<Entity> = Vec::new();

        let first_inv = commands.spawn((
            Inventory {
                owner: first,
                location: first,
                container: vec_inv,
                max_volume: Some(15.0),
                cached_current_volume: 0.0,
            },
            UpdateCachedVolume
        )).id();


        for _ in 0..1 {
            let mine_in_anom = Steps::build()
                .label("MineInAnom")
                .step(MoveToAnom)
                .step(MineAnom)
                .step(DepositOre);


            let ship = commands.spawn((
                spawn_new_pilot(),
                UndockingFrom(far),
                Thinker::build()
                    .label("mine")
                    .picker(FirstToScore { threshold: 0.8 })
                    .when(
                        Mine,
                        mine_in_anom,
                    ),
                WeaponBundle {
                    _weapon: Weapon {
                        _type: WeaponType::Mining,
                        config: WeaponConfig::RangeShort,
                        size: WeaponSize::Small,
                        tier: 1,
                        bank: 1,
                    },
                    target: WeaponTarget(None),
                    in_range: WeaponInRange(false),
                }
            )).id();

            let inv = commands.spawn((
                Inventory {
                    owner: ship,
                    location: ship,
                    container: Vec::new(),
                    max_volume: Some(253.6577),
                    cached_current_volume: 0.0,
                },
                UpdateCachedVolume,
                RegisterInventoryTo(ship)
            )).id();
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
    mut query_ships: Query<&mut Destination, Without<TestTag>>,
    query_targets: Query<&SimPosition, With<TestTag>>) {
    let mut t: Option<&SimPosition> = None;
    let mut min_dist: f64 = f64::MAX;
    for tr in query_targets.iter() {
        let dist = tr.0.length_squared();
        if dist < min_dist {
            min_dist = dist;
            t = Some(tr)
        }
    }

    //println!("{:?}",t);

    for mut dest in &mut query_ships {
        match t {
            Some(tr) => {
                dest.0 = TEntity(*tr);
            }
            None => {}
        }
    }
}

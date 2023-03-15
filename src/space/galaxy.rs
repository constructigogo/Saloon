pub mod map;

use std::f64::consts::PI;
use std::ops::Range;
use std::thread::current;

use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::system::lifetimeless::SCommands;
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

pub fn m_to_system(from: f64) -> f64 {
    return from * 0.000001;
}

pub fn au_to_system(from : f64) -> f64 {
    return from * 150000.0;
}

pub fn around_pos(pos: SimPosition, radius: f64) -> SimPosition {
    let rng = thread_rng().gen::<f64>() * 2.0 * PI;
    let rad = thread_rng().gen_range::<f64, Range<f64>>(0.0..radius);
    let n_pos = pos.0 + ((DVec3::new(f64::cos(rng), f64::sin(rng), 0.0)) * m_to_system(rad));
    return SimPosition(n_pos);
}

/// Since we need every ship to be able to live in a different system/map
/// we need to simulate them independently of the rendering, all in local space
/// but without interfering with each others (ships in system A should not see ships in system B)
/// TODO WE NEED THAT ASAP
#[derive(Resource, Default)]
pub struct SystemMap(pub Vec<Entity>);

/// Index of the reference system
#[derive(Component, Deref)]
pub struct GalaxyCoordinate(pub Entity);

#[derive(Resource, Deref)]
pub struct GalaxyScale(pub f64);

/// Since coordinates are float we need to avoid going into large coordinates value
#[derive(Component)]
pub struct SolarSystem {
    pub anomalies: Vec<Entity>,
    pub gates: Vec<Entity>,
    //pub size: f32, //probably useless we'll see
}

/// Position for simulation
#[derive(Component, Default, Copy, Clone, Deref, DerefMut, Reflect)]
pub struct SimPosition(pub DVec3);

#[derive(Bundle)]
pub struct GalaxyGateBundle {
    pub tag: GalaxyGateTag,
    pub desto: GateDestination,
}

#[derive(Component, Deref)]
pub struct GateDestination(pub Entity);


#[derive(Component)]
pub struct GalaxyGateTag;


#[derive(Bundle)]
pub struct DisplayableGalaxyEntityBundle {
    pub display: SpriteBundle,
    pub galaxy: GalaxyEntityBundle,
}

#[derive(Bundle)]
pub struct GalaxyEntityBundle {
    pub galaxy_coord: GalaxyCoordinate,
    pub simulation_position: SimPosition,
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Rendered;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RenderFlag;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ViewState {
    SYSTEM,
    GALAXY,
    EMPTY,
}

#[derive(Resource)]
pub struct CurrentSystemDisplay(pub Option<Entity>);

pub struct HideSystemEvent;

pub struct HideGalaxyEvent;

pub struct RenderGalaxyEvent;

pub fn exit_system_view(
    keys: Res<Input<KeyCode>>,
    mut system: ResMut<CurrentSystemDisplay>,
    mut ev: EventWriter<RenderGalaxyEvent>,
    mut ev_hide: EventWriter<HideSystemEvent>,
    mut state: ResMut<State<ViewState>>) {

    //println!("state {:?}",state.current());
    if *state.current() != ViewState::GALAXY {
        if keys.any_just_pressed([KeyCode::Numpad0, KeyCode::Escape]) {
            ev_hide.send(HideSystemEvent);
            ev.send(RenderGalaxyEvent);
            state.set(ViewState::GALAXY);
            system.0 = None;
        }
    }
}


pub fn click_enter_system_view(
    mut events: EventReader<PickingEvent>,
    query_clicked: Query<(Entity), (With<SolarSystem>)>,
    mut ev: EventWriter<RenderSystemEvent>,
    mut ev_hide: EventWriter<HideGalaxyEvent>,
    mut state: ResMut<State<ViewState>>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(_) => {}
            PickingEvent::Hover(_) => {}
            PickingEvent::Clicked(e) => {
                if let Ok(isok) = query_clicked.get(*e) {
                    ev_hide.send(HideGalaxyEvent);
                    ev.send(RenderSystemEvent(isok));
                    state.set(ViewState::SYSTEM);
                }
            }
        }
    }
}


pub struct RenderSystemEvent(Entity);

pub fn flag_render_solar_system(
    mut commands: Commands,
    mut current: ResMut<CurrentSystemDisplay>,
    query_future: Query<(Entity, &GalaxyCoordinate), (Without<Rendered>, With<Transform>)>,
    mut ev_render: EventReader<RenderSystemEvent>,
    mut state: ResMut<State<ViewState>>,
) {
    if !ev_render.is_empty() {
        let sys = ev_render.iter().next();
        match sys {
            Some(val) => {
                //flag for render entites in system val
                for (entity, galaxy) in &query_future {
                    if galaxy.0 == val.0 {
                        commands.entity(entity).insert(Rendered);
                    }
                }
                println!("render map {:?}", val.0);
                current.0 = Some(val.0);
            }
            None => {}
        }
    }
}

pub fn hide_system_view(mut commands: Commands,
                        mut query: Query<(Entity, &mut Visibility), (With<Rendered>)>,
                        mut state: ResMut<State<ViewState>>,
                        mut ev_hide: EventReader<HideSystemEvent>) {
    if !ev_hide.is_empty() {
        //let sys = ev_hide.iter().next();
        for ev in ev_hide.iter() {
            for (entity, mut vis) in &mut query {
                vis.is_visible = false;
                commands.entity(entity).remove::<Rendered>();
            }
            println!("hide_system_view");
        }

        //state.set(ViewState::EMPTY);
    }
}

pub fn hide_galaxy_view(mut commands: Commands,
                        mut query: Query<(Entity, &mut Visibility), (With<SolarSystem>)>,
                        mut state: ResMut<State<ViewState>>,
                        mut ev_hide: EventReader<HideGalaxyEvent>) {
    if !ev_hide.is_empty() {
        let sys = ev_hide.iter().next();
        for (entity, mut vis) in &mut query {
            vis.is_visible = false;
            commands.entity(entity).remove::<Rendered>();
        }
        println!("hide_galaxy_view");
        //state.set(ViewState::EMPTY);
    }
}

pub fn generate_galaxy_view(mut commands: Commands,
                            mut query_clicked: Query<(Entity, &mut Visibility), (With<SolarSystem>)>,
                            mut state: ResMut<State<ViewState>>,
                            mut ev_render: EventReader<RenderGalaxyEvent>) {
    if !ev_render.is_empty() {
        for _ in ev_render.iter(){
            for (entity, mut vis) in &mut query_clicked {
                vis.is_visible = true;
                commands.entity(entity).insert(Rendered);
            }
            //state.set(ViewState::GALAXY);
            println!("render galaxy");
        }

    }
}

pub fn generate_system_view(mut commands: Commands,
                            mut query: Query<(Entity, &mut Visibility), Added<Rendered>>,
                            mut state: ResMut<State<ViewState>>) {
    for (entity, mut vis) in &mut query {
        vis.is_visible = true;
        //commands.entity(entity).insert(Rendered).remove::<RenderFlag>();
    }
}

pub fn add_to_system_view(
    mut commands: Commands,
    current: Res<CurrentSystemDisplay>,
    query_future: Query<(Entity, &GalaxyCoordinate), (Without<Rendered>, Without<RenderFlag>, Added<Transform>)>,
) {
    match current.0 {
        None => {}
        Some(system) => {
            for (id, galaxy) in query_future.iter() {
                if galaxy.0 == system {
                    commands.entity(id).insert((Rendered));
                }
            }
        }
    }
}

/*
#[derive(Bundle)]
pub struct PointOfInterestBundle{
    galaxy_coord : GalaxyCoordinate,
    local_transform : Transform,
}
*/
use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::math::DVec3;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

/// Since we need every ship to be able to live in a different system/map
/// we need to simulate them independently of the rendering, all in local space
/// but without interfering with each others (ships in system A should not see ships in system B)
/// TODO WE NEED THAT ASAP
#[derive(Resource, Default)]
pub struct SystemMap(pub Vec<Entity>);

/// Index of the reference system
#[derive(Component, Deref)]
pub struct GalaxyCoordinate(pub Entity);

/// Since coordinates are float we need to avoid going into large coordinates value
#[derive(Component)]
pub struct SolarSystem {
    pub anomalies: Vec<Entity>,
    pub gates: Vec<Entity>,
    //pub size: f32, //probably useless we'll see
}

/// Position for simulation
#[derive(Component, Default, Copy, Clone, Deref, DerefMut)]
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


#[derive(Component)]
pub struct AnomalyMining;

#[derive(Component)]
pub struct AnomalyCombat;


#[derive(Bundle)]
pub struct GalaxyEntityBundle {
    pub galaxy_coord: GalaxyCoordinate,
    pub simulation_position : SimPosition,
    pub local_transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
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

pub struct HideSystemEvent;

pub struct HideGalaxyEvent;

pub struct RenderGalaxyEvent;

pub fn exit_system_view(
    keys: Res<Input<KeyCode>>,
    mut ev: EventWriter<RenderGalaxyEvent>,
    mut ev_hide: EventWriter<HideSystemEvent>,
    mut state: ResMut<State<ViewState>>) {

    //println!("state {:?}",state.current());
    if *state.current() != ViewState::GALAXY {
        if keys.any_just_pressed([KeyCode::Numpad0, KeyCode::Escape]) {
            ev_hide.send(HideSystemEvent);
            ev.send(RenderGalaxyEvent);
            state.set(ViewState::GALAXY);

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

pub fn flag_render_solar_system(mut commands: Commands,
                                query_future: Query<(Entity, &GalaxyCoordinate), Without<Rendered>>,
                                mut ev_render: EventReader<RenderSystemEvent>,
                                mut state: ResMut<State<ViewState>>) {
    if !ev_render.is_empty() {
        let sys = ev_render.iter().next();
        match sys {
            Some(val) => {
                //flag for render entites in system val
                for (entity, galaxy) in &query_future {
                    if galaxy.0 == val.0 {
                        commands.entity(entity).insert(RenderFlag);
                    }
                }
                println!("render map {:?}", val.0);
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
        for (entity, mut vis) in &mut query {
            vis.is_visible = false;
            commands.entity(entity).remove::<Rendered>();
        }
        //state.set(ViewState::EMPTY);
    }
}

pub fn hide_galaxy_view(mut commands: Commands,
                        mut query: Query<(Entity, &mut Visibility), (With<SolarSystem>)>,
                        mut state: ResMut<State<ViewState>>,
                        mut ev_hide: EventReader<HideGalaxyEvent>) {
    if !ev_hide.is_empty() {
        //let sys = ev_hide.iter().next();
        for (entity, mut vis) in &mut query {
            vis.is_visible = false;
            commands.entity(entity).remove::<Rendered>();
        }
        //state.set(ViewState::EMPTY);
    }
}

pub fn generate_galaxy_view(mut commands: Commands,
                            mut query_clicked: Query<(Entity, &mut Visibility), (With<SolarSystem>)>,
                            mut state: ResMut<State<ViewState>>,
                            ev_render: EventReader<RenderGalaxyEvent>) {
    if !ev_render.is_empty() {
        for (entity, mut vis) in &mut query_clicked {
            vis.is_visible = true;
            commands.entity(entity).insert(Rendered);
        }
        //state.set(ViewState::GALAXY);
        println!("render galaxy");
    }
}

pub fn generate_system_view(mut commands: Commands,
                            mut query: Query<(Entity, &mut Visibility), Added<RenderFlag>>,
                            mut state: ResMut<State<ViewState>>) {
    for (entity, mut vis) in &mut query {
        vis.is_visible = true;
        commands.entity(entity).insert(Rendered).remove::<RenderFlag>();
    }
}


/*
#[derive(Bundle)]
pub struct PointOfInterestBundle{
    galaxy_coord : GalaxyCoordinate,
    local_transform : Transform,
}
*/
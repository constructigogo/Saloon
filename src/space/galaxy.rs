use bevy::{prelude::*, ecs::{query, entity::Entities}};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
/// Since we need every ship to be able to live in a different system/map
/// we need to simulate them independently of the rendering, all in local space
/// but without interfering with each others (ships in system A should not see ships in system B)
/// TODO WE NEED THAT ASAP

#[derive(Resource,Default)]
pub struct SystemMap(pub Vec<Entity>);

///Index of the reference system
#[derive(Component,Deref)]
pub struct GalaxyCoordinate(pub Entity);

#[derive(Component)]
pub struct SolarSystem {
    pub anomalies : Vec<Entity>,
    pub gates : Vec<Entity>
}



#[derive(Bundle)]
pub struct GalaxyGateBundle{
    pub tag : GalaxyGateTag,
    pub desto : GateDestination,
}

#[derive(Component,Deref)]
pub struct GateDestination(pub Entity);


#[derive(Component)]
pub struct GalaxyGateTag;


#[derive(Component)]
pub struct AnomalyMining;

#[derive(Component)]
pub struct AnomalyCombat;


#[derive(Bundle)]
pub struct GalaxyEntityBundle{
    pub galaxyCoord : GalaxyCoordinate,
    pub localTransform : Transform,
    pub globalTransform : GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Rendered;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RenderFlag;

#[derive(Resource,PartialEq, Eq)]
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
    state: Res<ViewState>){
    if *state.into_inner() != ViewState::GALAXY { 
        if keys.just_pressed(KeyCode::Numpad0) {
            ev.send(RenderGalaxyEvent);
            ev_hide.send(HideSystemEvent);
        }
    }
}


pub fn click_enter_system_view(
    mut events: EventReader<PickingEvent>,
    queryClicked : Query<(Entity), (With<SolarSystem>)>,
    mut ev: EventWriter<RenderSystemEvent>,
    mut evHide: EventWriter<HideGalaxyEvent>){
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => {},
            PickingEvent::Hover(e) => {},
            PickingEvent::Clicked(e) => {
                if let Ok(isok) = queryClicked.get(*e) {
                    evHide.send(HideGalaxyEvent);
                    ev.send(RenderSystemEvent(isok));
                }
            },
        }
    }
}


pub struct RenderSystemEvent(Entity);

pub fn flag_render_solar_system(mut commands: Commands,
    queryFuture : Query<(Entity, &GalaxyCoordinate), Without<Rendered>>,
    mut ev_render: EventReader<RenderSystemEvent>){
    
    if !ev_render.is_empty() {
        let sys = ev_render.iter().next();
        match sys {
            Some(val) => {
                //flag for render entites in system val
                for (entity,galaxy) in &queryFuture{
                    if galaxy.0 == val.0 {
                        commands.entity(entity).insert(RenderFlag);
                    }
                }
                println!("render map {:?}",val.0);
            },
            None => {},
        }
    }
}

pub fn hide_system_view(mut commands: Commands,
    mut query : Query<(Entity,& mut Visibility), (With<Rendered>)>,
    mut state: ResMut<ViewState>,
    mut ev_hide: EventReader<HideSystemEvent>){

    if !ev_hide.is_empty() {
        let sys = ev_hide.iter().next();
        for (entity,mut vis) in & mut query {
            vis.is_visible =false;
            commands.entity(entity).remove::<Rendered>();
        }
        *state=ViewState::EMPTY;
    }
    
}

pub fn hide_galaxy_view(mut commands: Commands,
    mut query : Query<(Entity,& mut Visibility), (With<SolarSystem>)>,
    mut state: ResMut<ViewState>,
    mut ev_hide: EventReader<HideGalaxyEvent>){

    if !ev_hide.is_empty() {
        let sys = ev_hide.iter().next();
        for (entity,mut vis) in & mut query {
            vis.is_visible =false;
            commands.entity(entity).remove::<Rendered>();
        }
        *state=ViewState::EMPTY;
    }
    
}

pub fn generate_galaxy_view(mut commands: Commands,
    mut queryClicked : Query<(Entity,& mut Visibility), (With<SolarSystem>)>,
    mut state: ResMut<ViewState>,
    ev_render: EventReader<RenderGalaxyEvent>){

    if !ev_render.is_empty(){
        for (entity,mut vis) in & mut queryClicked {
            vis.is_visible =true;
            commands.entity(entity).insert(Rendered);
        }
        *state=ViewState::GALAXY;
    }
}

pub fn generate_system_view(mut commands: Commands,
    mut query: Query<(Entity,& mut Visibility) , Added<RenderFlag>>,
    mut state: ResMut<ViewState>){
    for (entity,mut vis) in & mut query {
        vis.is_visible =true;
        commands.entity(entity).insert(Rendered).remove::<RenderFlag>();
    }
    *state=ViewState::SYSTEM;
}


/*
#[derive(Bundle)]
pub struct PointOfInterestBundle{
    galaxyCoord : GalaxyCoordinate,
    localTransform : Transform,
}
*/
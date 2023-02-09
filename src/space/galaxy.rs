use bevy::{prelude::*, ecs::{query, entity::Entities}};

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
}

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


pub struct RenderSystemEvent(Entity);


pub fn flag_render_solar_system(mut commands: Commands,
    mut queryCurrent: Query<(Entity,&mut Visibility), With<Rendered>>,
    queryFuture : Query<(Entity, &GalaxyCoordinate), Without<Rendered>>,
    keys: Res<Input<KeyCode>>,
    cluster: Res<SystemMap>,
    mut ev_render: EventReader<RenderSystemEvent>){
    
    if !ev_render.is_empty() {
        let sys = ev_render.iter().next();
        match sys {
            Some(val) => {
                for (entity, mut vis) in &mut queryCurrent {
                    commands.entity(entity).remove::<Rendered>();
                    vis.is_visible = false;
                    
                }
                for (entity,galaxy) in &queryFuture{
                    if galaxy.0 == val.0 {
                        commands.entity(entity).insert(RenderFlag);
                    }
                }
                println!("render map 1");
            },
            None => {},
        }
    }
    
    if keys.any_just_pressed([KeyCode::Numpad1,KeyCode::Numpad2,KeyCode::Numpad3]) {
        for (entity, mut vis) in &mut queryCurrent {
            commands.entity(entity).remove::<Rendered>();
            vis.is_visible = false;
        }
        println!("cleared view");
        if keys.just_pressed(KeyCode::Numpad1) {
            for (entity,galaxy) in &queryFuture{
                if galaxy.0 == cluster.0[0] {
                    commands.entity(entity).insert(RenderFlag);
                }
            }
            println!("render map 1");
        }
        if keys.just_pressed(KeyCode::Numpad2) {
            for (entity,galaxy) in &queryFuture{
                if galaxy.0 == cluster.0[1] {
                    commands.entity(entity).insert(RenderFlag);
                }
            }
            println!("render map 2");
        }
    };
}

pub fn generate_view(mut commands: Commands,
    mut query: Query<(Entity,& mut Visibility) , Added<RenderFlag>>,){
    for (entity,mut vis) in & mut query {
        vis.is_visible =true;
        commands.entity(entity).insert(Rendered).remove::<RenderFlag>();
    }
}


/*
#[derive(Bundle)]
pub struct PointOfInterestBundle{
    galaxyCoord : GalaxyCoordinate,
    localTransform : Transform,
}
*/
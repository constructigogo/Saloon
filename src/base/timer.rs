use bevy::app::App;
use bevy::prelude::*;

#[derive(Resource)]
pub struct OneSecondTimer(pub Timer);

#[derive(Resource)]
pub struct FiveSecondTimer(pub Timer);

pub struct TimerPlugin;


fn tick_timers(time : Res<Time>,
    mut timer1 : ResMut<OneSecondTimer>,
    mut timer5 : ResMut<FiveSecondTimer>) {
    
    timer1.0.tick(time.delta());
    timer5.0.tick(time.delta());

}




impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
            .insert_resource(OneSecondTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(FiveSecondTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_system(tick_timers);
        }
}
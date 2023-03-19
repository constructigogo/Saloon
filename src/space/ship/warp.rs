use std::cmp::min;
use std::process::id;

use bevy::prelude::*;

use crate::{Destination, DVec3, m_to_system, Mass, SimPosition, ThrusterEngine};
use crate::space::galaxy::au_to_system;
use crate::space::ship;
use crate::warp::WarpPhase::*;

const AU: f64 = 1_500_000_000.0;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InitWarp;

pub enum WarpPhase {
    Accelerating,
    Cruise,
    Decelerating,
}

#[derive(Component)]
pub struct Warping {
    speed: f64,
    // AU/s
    from_pos: DVec3,
    ramping: f64,
    ramping_max: f64,
    cruise_time: f64,
    current_phase: WarpPhase,
}

impl Warping {
    fn build(speed: f64, from: DVec3) -> Warping {
        return Warping {
            speed,
            from_pos: from,
            ramping: 0.0,
            ramping_max: AU.ln() / speed,
            cruise_time: 0.0,
            current_phase: Accelerating,
        };
    }
}


pub fn check_for_warp_system(
    mut commands: Commands,
    mut query: Query<(Entity, &SimPosition, &Destination), (Or<(Added<Destination>, Changed<Destination>)>, Without<Warping>)>,
) {
    for (id, pos, desto) in query.iter() {
        match desto.0 {
            ship::DestoType::DPosition(target) => {
                let dist_left: f64 = (pos.0 - target.0).length();
                if dist_left > m_to_system(10000.0) {
                    commands.entity(id).insert((InitWarp));
                }
            }
            ship::DestoType::TEntity(_) => {}
            ship::DestoType::None => {}
        }
    }
}


pub fn init_warp_system(
    mut commands: Commands,
    mut query: Query<(Entity, &SimPosition, &Mass, &ThrusterEngine), With<InitWarp>>,
) {
    for (id, pos, mass, thruster) in query.iter() {
        commands.entity(id).insert((
            Warping::build(3.0, pos.0)
        )).remove::<InitWarp>();
    }
}

pub fn warp_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SimPosition, &Destination, &mut Warping)>,
) {
    for (id, mut pos, desto, mut warping) in query.iter_mut() {
        let dt = 0.02;
        match desto.0 {
            ship::DestoType::DPosition(target) => {
                let dir: DVec3 = (target.0 - pos.0).normalize();
                let dist: f64 = (target.0 - warping.from_pos).length();
                //println!("{:?}", dist);
                let k = warping.speed;
                let a = AU;
                let v_warp = k * a;
                let j = f64::min(k / 3.0, 2.0);


                let t_accel = (v_warp / k).ln() / k;
                let t_decel = (v_warp / 100.0).ln() / j;

                let accel_dist = AU;
                let decel_dist = (k * AU) / j;
                let cruis_dist = (dist / au_to_system(1.0)) * AU - accel_dist - accel_dist;

                let min_dist = m_to_system(accel_dist + accel_dist);

                let t_cruise = cruis_dist / v_warp;
                let t_total = t_cruise + t_accel + t_accel;

                //println!("t : {:?}, 1:{:?}, 2:{:?}, 3:{:?}", warping.ramping, t_accel, t_cruise, t_accel);

                if dist > min_dist {
                    match &warping.current_phase {
                        Accelerating => {
                            //println!("accel");

                            let t = warping.ramping.min(warping.ramping_max);


                            let d = (k * t).exp();
                            let v = k * d;

                            let traveled_r = m_to_system(d) / dist;

                            pos.0 = DVec3::lerp(warping.from_pos, target.0, traveled_r);

                            //println!("dt {:?}", traveled_r);

                            if t >= t_accel {
                                warping.current_phase = Cruise;
                            } else {
                                //pos.0 += dir * m_to_system(v * time.delta_seconds_f64());
                                warping.ramping += time.delta_seconds_f64();
                            }
                        }
                        Cruise => {
                            //println!("cruise");
                            let t = warping.cruise_time.min(t_cruise);
                            let at = warping.ramping.min(warping.ramping_max);
                            let d = (k * at).exp();
                            let v = k * d;

                            let traveled_r = m_to_system(d) / dist;
                            let dist_left = ((dist - (2.0 * m_to_system(AU))) / dist);
                            let percent = t / t_cruise;

                            let traveled_cruise: f64 = percent * dist_left;
                            let total = traveled_r + traveled_cruise;

                            pos.0 = DVec3::lerp(warping.from_pos, target.0, total);

                            //println!("dt {:?}: percent {:?}",dist_left,percent);


                            if t >= t_cruise {
                                warping.current_phase = Decelerating;
                            } else {
                                //pos.0 += dir * m_to_system(v * time.delta_seconds_f64());
                                warping.cruise_time += time.delta_seconds_f64();
                            }
                        }
                        Decelerating => {
                            //println!("decel");
                            warping.ramping -= time.delta_seconds_f64();

                            let dist_left = m_to_system(AU);
                            let percent = (dist - dist_left) / dist;

                            let t = warping.ramping.max(0.0);


                            let d = (k * (t)).exp();

                            let traveled_r = m_to_system(AU - d) / dist;

                            pos.0 = DVec3::lerp(warping.from_pos, target.0, percent + traveled_r);


                            if t <= 0.0 {
                                //println!("dist {:?}", (target.0-pos.0).length()/0.000001);
                                commands.entity(id).remove::<Warping>();
                            } else {
                                //pos.0 += dir * m_to_system(v * time.delta_seconds_f64());
                            }
                        }
                    }


                    //warping.ramping = warping.ramping.clamp(0.0,warping.ramping_max);
                    //println!("t : {:?}, t_a : {:?},t_d : {:?}, v : {:?}, v_w : {:?}", t, t_accel, t_decel, v, v_warp);
                } else {
                    //commands.entity(id).remove::<Warping>();
                }
            }
            ship::DestoType::TEntity(_) => {}
            ship::DestoType::None => {}
        }
    }
}
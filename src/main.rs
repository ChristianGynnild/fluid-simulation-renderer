mod renderer;
use renderer::render;
mod simulation;
use simulation::IX;
use std::fs;

pub const N:i32 = 128;

fn main(){
    let mut dens_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut dens = vec![0.;(N as usize+2)*(N as usize+2)];

    let mut u_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut v_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    
    let mut u = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut v = vec![0.;(N as usize+2)*(N as usize+2)];

    let speed_amout = 1.;
    let visc = 0.001;
    let diff = 0.01;
    let dt = 0.01;

    dens = simulation::init_density(N, dens);
    dens_prev = simulation::init_density(N, dens_prev);

    (u, v) = simulation::init_velocity(N, speed_amout, u,v);
    (u_prev, v_prev) = simulation::init_velocity(N, speed_amout, u_prev,v_prev);

    fs::remove_dir_all("images");
    std::fs::create_dir_all("images");
    let mut filepath;

    let mut i:i32 = 0;

    while true{        
        filepath = format!("images/output{}.png", i);
        println!("Rendering frame {}", i);
        render(&dens_prev, &filepath);
    
        (u, v, u_prev, v_prev) = simulation::vel_step(N, u, v, u_prev, v_prev, visc, dt);
        (dens, dens_prev) = simulation::dens_step(N, dens, dens_prev, &u, &v, diff, dt);

        i = i+1;
    }
}
mod renderer;
use renderer::render;
mod simulation;
use simulation::IX;

pub const WIDTH:i32 = 200;
pub const HEIGHT:i32 = 200;

pub const N:i32 = 64;

fn main(){
    let mut dens_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut dens = vec![0.;(N as usize+2)*(N as usize+2)];

    let mut u_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut v_prev = vec![0.;(N as usize+2)*(N as usize+2)];
    
    let mut u = vec![0.;(N as usize+2)*(N as usize+2)];
    let mut v = vec![0.;(N as usize+2)*(N as usize+2)];

    let visc = 0.001;
    let diff = 0.0001;
    let dt = 0.01;

    dens = simulation::init_density(N, dens);
    dens_prev = simulation::init_density(N, dens_prev);

    (u, v) = simulation::init_velocity(N, u,v);
    (u_prev, v_prev) = simulation::init_velocity(N, u_prev,v_prev);


    std::fs::create_dir_all("images");
    let mut filepath;

    for i in 0..100{        
        filepath = format!("images/output{}.png", i);
        println!("Rendering frame {}", i);
        render(&dens_prev, &filepath);
    
        (u, v, u_prev, v_prev) = simulation::vel_step(N, u, v, u_prev, v_prev, visc, dt);
        (dens, dens_prev) = simulation::dens_step(N, dens, dens_prev, &u, &v, diff, dt);
    }
}
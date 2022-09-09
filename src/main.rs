mod renderer;
use renderer::render;
mod simulation;
use simulation::IX;

pub const WIDTH:usize = 200;
pub const HEIGHT:usize = 200;


fn main(){
    let mut density0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut density = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    let mut velocityX0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut velocityY0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    
    let mut velocityX = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut velocityY = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            if x > WIDTH/2{
                density0[IX(x,y)] = 1.;
            }
        }
    }

    for x in 1..(WIDTH+1){
        for y in 1..(WIDTH+1){
            let (velocity_init_x, velocity_init_y):(f32, f32) = ((x as f32-(WIDTH/2) as f32), (y as f32-(HEIGHT/2) as f32));
            (velocityX0[IX(x,y)], velocityY0[IX(x,y)]) = (-velocity_init_y, velocity_init_x);
        }
    }

    density0 = simulation::set_boundary(density0, 0);
    (velocityX, velocityY, velocityX0, velocityY0) = (velocityX0, velocityY0, velocityX, velocityY);


    std::fs::create_dir_all("images");

    let mut filepath = format!("images/output{}.png", 0);
    println!("Rendering frame {}", 0);
    render(&density0, &filepath);

    for i in 1..100{    
        density = simulation::diffuse(density, &density0, 0, 0.01, 0.01);
        (density, density0) = (density0, density);
        density = simulation::advect(density, &density0, 0, &velocityX, &velocityY, 1., 0.01);
        (density, density0) = (density0, density);

        
        filepath = format!("images/output{}.png", i);
        println!("Rendering frame {}", i);
        render(&density0, &filepath);
    }


    
}
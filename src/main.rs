mod renderer;
use renderer::render;
mod simulation;
use simulation::index;

pub const WIDTH:usize = 8;
pub const HEIGHT:usize = 8;


fn main(){
    let mut density0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut density = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    let mut velocityX0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut velocityY0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    
    let mut velocityX = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut velocityY = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            if x > 4{
                density[index(x,y)] = 1.;
            }
        }
    }

    density0 = density.to_vec();

    
    density = simulation::diffuse(density, density0, 0, 0.005, 0.01);

    render(&density);

    
}
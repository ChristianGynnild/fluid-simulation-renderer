mod renderer;
use renderer::render;
mod simulation;
use simulation::index;

pub const WIDTH:usize = 2000;
pub const HEIGHT:usize = 2000;



fn main(){
    let mut density0 = vec![0.;(HEIGHT+2)*(WIDTH+2)];
    let mut density = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    for x in 0..(WIDTH+2){
        for y in 0..(HEIGHT+2){
            density0[index(x,y)] = x as f32/(WIDTH+2) as f32 * y as f32/(WIDTH+2) as f32;
        }
    }

    //density = simulation::diffuse(density, density0, 1, 1., 0.00001);

    render(&density0);
}
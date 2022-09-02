mod renderer;
use renderer::render;

pub const WIDTH:usize = 2000;
pub const HEIGHT:usize = 2000;

use std::mem;

fn index(x:usize,y:usize) -> usize{
    return x+y*(WIDTH+2)
}


fn main(){
    let mut density = vec![0.;(HEIGHT+2)*(WIDTH+2)];

    for x in 0..(WIDTH+2){
        for y in 0..(HEIGHT+2){
            density[index(x,y)] = x as f32/(WIDTH+2) as f32;
        }
    }


    render(&density);
}
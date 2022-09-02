const WIDTH:usize=super::WIDTH;
const HEIGHT:usize=super::HEIGHT;


pub fn index(x:usize,y:usize) -> usize{
    return x+y*(WIDTH+2);
}

pub fn diffuse(mut attribute:Vec<f32>, attribute0:Vec<f32>, boundary:i8, diffusion_speed:f32, delta_time:f32)-> Vec<f32>{
    let simulation_time_step = WIDTH as f32*HEIGHT as f32*diffusion_speed*delta_time;

    let mut average_surrounding_values:f32;

    for i in 0..20{
        for x in 1..(WIDTH+1){
            for y in 1..(HEIGHT+1){
                average_surrounding_values = (
                    attribute[index(x+1,y  )]+
                    attribute[index(x-1,y  )]+
                    attribute[index(x  ,y+1)]+
                    attribute[index(x  ,y-1)]
                )/4.;
                attribute[index(x,y)] = (attribute0[index(x,y)]+average_surrounding_values*simulation_time_step)/(1.+simulation_time_step);
            }
        }
        // set_boundary()
    }

    return attribute;
}

pub fn advect(){

}

pub fn remove_divergence(){

}
const WIDTH:usize=super::WIDTH;
const HEIGHT:usize=super::HEIGHT;


fn lerp(a:f32, b:f32, interpolation_value:f32)->f32{
    return a*(1.-interpolation_value)+interpolation_value*b
}

pub fn index(x:usize,y:usize) -> usize{
    return x+y*(WIDTH+2);
}

pub fn diffuse(mut attribute:Vec<f32>, attribute0:Vec<f32>, boundary:i8, diffusion_speed:f32, delta_time:f32)-> Vec<f32>{
    let time_step = WIDTH as f32*HEIGHT as f32*diffusion_speed*delta_time;

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
                attribute[index(x,y)] = (attribute0[index(x,y)]+average_surrounding_values*time_step)/(1.+time_step);
            }
        }
        // set_boundary()
    }

    return attribute;
}

pub fn advect(mut attribute:Vec<f32>, attribute0:Vec<f32>, boundary:i8, velocity_x:Vec<f32>, velocity_y:Vec<f32>, advection_speed:f32, delta_time:f32){
    let time_step = WIDTH as f32*advection_speed*delta_time;
    let mut position_x;
    let mut position_y;

    for x in 1..(WIDTH+1){
        for y in 2..(HEIGHT+1){
            position_x = x as f32-velocity_x[index(x,y)]*time_step;
            position_y = y as f32-velocity_y[index(x,y)]*time_step;

            if (position_x<0.5) {position_x=0.5}; 
            if (position_y<0.5) {position_y=0.5}; 

            if (position_x>WIDTH as f32+0.5) {position_x=WIDTH as f32 + 0.5}; 
            if (position_y>HEIGHT as f32+0.5) {position_y=HEIGHT as f32 + 0.5}; 
                
            let floored_position_x = position_x as usize;
            let floored_position_y = position_y as usize;

            attribute[index(x,y)] = lerp(
                lerp(
                    attribute0[index(floored_position_x, floored_position_y)], 
                    attribute0[index(floored_position_x+1, floored_position_y)],
                    floored_position_x as f32 - position_x
                ),
                lerp(
                    attribute0[index(floored_position_x, floored_position_y+1)], 
                    attribute0[index(floored_position_x+1, floored_position_y+1)],
                    floored_position_x as f32 - position_x
                ),
                floored_position_y as f32 - position_y
            )
        }
    }

    //set_boundary()
}

pub fn remove_divergence(mut velocity_x:Vec<f32>, mut velocity_y:Vec<f32>){
    
    let mut divergence = vec![0.;(WIDTH+2)*(HEIGHT+2)];
    let mut p = vec![0.;(WIDTH+2)*(HEIGHT+2)];

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            divergence[index(x,y)] = 
                (
                    (velocity_x[index(x+1,y)]-velocity_x[index(x-1,y)])-
                    (velocity_y[index(x,y+1)]-velocity_y[index(x,y-1)])
                ) * 0.5;
        }
    }

    //set boundary(div)

    for i in 0..20{
        for x in 1..(WIDTH+1){
            for y in 1..(HEIGHT+1){
                p[index(x, y)] = (p[index(x+1,y)]+p[index(x-1,y)]+p[index(x,y+1)]+p[index(x,y-1)]-divergence[index(x,y)])/4.;
            }
        }
        //set_boundary(p)
    }

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            velocity_x[index(x, y)] -= 0.5*(p[index(x+1,y)]-p[index(x-1,y)]);
            velocity_y[index(x, y)] -= 0.5*(p[index(x,y+1)]-p[index(x,y-1)]);
        }
    }

}
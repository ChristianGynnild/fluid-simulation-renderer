const WIDTH:usize=super::WIDTH;
const HEIGHT:usize=super::HEIGHT;


fn lerp(a:f32, b:f32, interpolation_value:f32)->f32{
    return a*(1.-interpolation_value)+interpolation_value*b
}

pub fn index(x:usize,y:usize) -> usize{
    return x+y*(WIDTH+2);
}

pub fn diffuse(mut attribute:Vec<f32>, attribute0:&Vec<f32>, dimension:i8, diffusion_speed:f32, delta_time:f32)-> Vec<f32>{
    let time_step = WIDTH as f32*HEIGHT as f32*diffusion_speed*delta_time;

    let mut average_surrounding_values:f32;

    for i in 0..100{
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
        attribute = set_boundary(attribute, dimension);
    }

    return attribute;
}

pub fn advect(mut attribute:Vec<f32>, attribute0:&Vec<f32>, dimension:i8, velocity_x:&Vec<f32>, velocity_y:&Vec<f32>, advection_speed:f32, delta_time:f32) -> Vec<f32>{
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

    attribute = set_boundary(attribute, dimension);

    return attribute;
}

pub fn remove_divergence(mut velocity_x:Vec<f32>, mut velocity_y:Vec<f32>) -> (Vec<f32>, Vec<f32>){
    
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

    divergence = set_boundary(divergence, 0);

    for i in 0..20{
        for x in 1..(WIDTH+1){
            for y in 1..(HEIGHT+1){
                p[index(x, y)] = (p[index(x+1,y)]+p[index(x-1,y)]+p[index(x,y+1)]+p[index(x,y-1)]-divergence[index(x,y)])/4.;
            }
        }
        p = set_boundary(p, 0);
    }

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            velocity_x[index(x, y)] -= 0.5*(p[index(x+1,y)]-p[index(x-1,y)]);
            velocity_y[index(x, y)] -= 0.5*(p[index(x,y+1)]-p[index(x,y-1)]);
        }
    }

    return (velocity_x, velocity_y)
}


pub fn set_boundary(mut attribute:Vec<f32>, dimension:i8) -> Vec<f32>
{
    for x in 1..(WIDTH+1){
        attribute[index(x, 0)] = match dimension{
            2 => -attribute[index(x, 1)],
            _ =>  attribute[index(x, 1)]
        };

        attribute[index(x, HEIGHT+1)] = match dimension{
            2 => -attribute[index(x, HEIGHT)],
            _ =>  attribute[index(x, HEIGHT)]
        };
    }

    for y in 1..(HEIGHT+1){
        attribute[index(0, y)] = match dimension{
            1 => -attribute[index(1, y)],
            _ =>  attribute[index(1, y)]
        };

        attribute[index(WIDTH+1, y)] = match dimension{
            1 => -attribute[index(WIDTH, y)],
            _ =>  attribute[index(WIDTH, y)]
        };
    }

    attribute[index(0,0)] = (attribute[index(1,0)]+attribute[index(0,1)])/2.;
    attribute[index(WIDTH+1,0)] = (attribute[index(WIDTH,0)]+attribute[index(WIDTH+1,1)])/2.;
    attribute[index(0,HEIGHT+1)] = (attribute[index(0,HEIGHT)]+attribute[index(1,HEIGHT+1)])/2.;
    attribute[index(WIDTH+1,HEIGHT+1)] = (attribute[index(WIDTH,HEIGHT+1)]+attribute[index(WIDTH+1,HEIGHT)])/2.;

    return attribute;
}
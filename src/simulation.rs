const WIDTH:usize=super::WIDTH;
const HEIGHT:usize=super::HEIGHT;


fn lerp(a:f32, b:f32, interpolation_value:f32)->f32{
    return a*(1.-interpolation_value)+interpolation_value*b
}

pub fn IX(x:i32,y:i32) -> usize{
    return x as usize+y as usize*(WIDTH+2);
}

fn add_source(N:i32, mut x:Vec<f32>, mut s:&Vec<f32>, dt:f32) -> Vec<f32>
{ 
    let size=(N+2)*(N+2);
    for i in (0 as usize)..size as usize{
        x[i] += dt*s[i];
    }

    return x;
}

fn diffuse(N:i32, b:i32, mut x:Vec<f32>, x0:&Vec<f32>, diff:f32, dt:f32) -> Vec<f32>
{
    let a=dt*diff*N as f32*N as f32;
    for k in 0..20{
        for i in 1..N{
            for j in 1..N{
                x[IX(i,j)] = (x0[IX(i,j)] + a*(x[IX(i-1,j)]+x[IX(i+1,j)]+
                x[IX(i,j-1)]+x[IX(i,j+1)]))/(1.+4.*a);
            }
        }
    set_bnd( N, b, x );
    }

    return x;
}

pub fn advect(mut attribute:Vec<f32>, attribute0:&Vec<f32>, dimension:i8, velocity_x:&Vec<f32>, velocity_y:&Vec<f32>, advection_speed:f32, delta_time:f32) -> Vec<f32>{
    let time_step = WIDTH as f32*advection_speed*delta_time;
    let mut position_x;
    let mut position_y;

    for x in 1..(WIDTH+1){
        for y in 2..(HEIGHT+1){
            position_x = x as f32-velocity_x[IX(x,y)]*time_step;
            position_y = y as f32-velocity_y[IX(x,y)]*time_step;

            if (position_x<0.5) {position_x=0.5}; 
            if (position_y<0.5) {position_y=0.5}; 

            if (position_x>WIDTH as f32+0.5) {position_x=WIDTH as f32 + 0.5}; 
            if (position_y>HEIGHT as f32+0.5) {position_y=HEIGHT as f32 + 0.5}; 
                
            let floored_position_x = position_x as usize;
            let floored_position_y = position_y as usize;

            attribute[IX(x,y)] = lerp(
                lerp(
                    attribute0[IX(floored_position_x, floored_position_y)], 
                    attribute0[IX(floored_position_x+1, floored_position_y)],
                    floored_position_x as f32 - position_x
                ),
                lerp(
                    attribute0[IX(floored_position_x, floored_position_y+1)], 
                    attribute0[IX(floored_position_x+1, floored_position_y+1)],
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
            divergence[IX(x,y)] = 
                (
                    (velocity_x[IX(x+1,y)]-velocity_x[IX(x-1,y)])-
                    (velocity_y[IX(x,y+1)]-velocity_y[IX(x,y-1)])
                ) * 0.5;
        }
    }

    divergence = set_boundary(divergence, 0);

    for i in 0..20{
        for x in 1..(WIDTH+1){
            for y in 1..(HEIGHT+1){
                p[IX(x, y)] = (p[IX(x+1,y)]+p[IX(x-1,y)]+p[IX(x,y+1)]+p[IX(x,y-1)]-divergence[IX(x,y)])/4.;
            }
        }
        p = set_boundary(p, 0);
    }

    for x in 1..(WIDTH+1){
        for y in 1..(HEIGHT+1){
            velocity_x[IX(x, y)] -= 0.5*(p[IX(x+1,y)]-p[IX(x-1,y)]);
            velocity_y[IX(x, y)] -= 0.5*(p[IX(x,y+1)]-p[IX(x,y-1)]);
        }
    }

    return (velocity_x, velocity_y)
}


pub fn set_boundary(mut attribute:Vec<f32>, dimension:i8) -> Vec<f32>
{
    for x in 1..(WIDTH+1){
        attribute[IX(x, 0)] = match dimension{
            2 => -attribute[IX(x, 1)],
            _ =>  attribute[IX(x, 1)]
        };

        attribute[IX(x, HEIGHT+1)] = match dimension{
            2 => -attribute[IX(x, HEIGHT)],
            _ =>  attribute[IX(x, HEIGHT)]
        };
    }

    for y in 1..(HEIGHT+1){
        attribute[IX(0, y)] = match dimension{
            1 => -attribute[IX(1, y)],
            _ =>  attribute[IX(1, y)]
        };

        attribute[IX(WIDTH+1, y)] = match dimension{
            1 => -attribute[IX(WIDTH, y)],
            _ =>  attribute[IX(WIDTH, y)]
        };
    }

    attribute[IX(0,0)] = (attribute[IX(1,0)]+attribute[IX(0,1)])/2.;
    attribute[IX(WIDTH+1,0)] = (attribute[IX(WIDTH,0)]+attribute[IX(WIDTH+1,1)])/2.;
    attribute[IX(0,HEIGHT+1)] = (attribute[IX(0,HEIGHT)]+attribute[IX(1,HEIGHT+1)])/2.;
    attribute[IX(WIDTH+1,HEIGHT+1)] = (attribute[IX(WIDTH,HEIGHT+1)]+attribute[IX(WIDTH+1,HEIGHT)])/2.;

    return attribute;
}
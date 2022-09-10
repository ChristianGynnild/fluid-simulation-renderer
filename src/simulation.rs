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

fn advect(N:i32, b:i32, mut d:Vec<f32>, d0:&Vec<f32>, u:&Vec<f32>, v:&Vec<f32>, dt:f32) -> Vec<f32>
{
    let (x, y, i0, j0, i1, j1);
    let (x, y, s0, t0, s1, t1, dt0);

    let dt0 = dt*N as f32;
    for i in 1..N{
        for j in 1..N{
            x = i as f32-dt0*u[IX(i,j)]; y = j as f32 -dt0*v[IX(i,j)];
            if (x<0.5){x=0.5}; if (x>N as f32+0.5){x=N as f32 + 0.5}; i0=x as i32; i1=i0+ 1;
            if (y<0.5){y=0.5}; if (y>N as f32+0.5){y=N as f32 + 0.5}; j0=y as i32; j1=j0+1;
            s1 = x-i0 as f32; s0 = 1.-s1; t1 = y-j0 as f32; t0 = 1.-t1;
            d[IX(i,j)] = s0*(t0*d0[IX(i0,j0)]+t1*d0[IX(i0,j1)])+
            s1*(t0*d0[IX(i1,j0)]+t1*d0[IX(i1,j1)]);
        }
    }
    set_bnd( N, b, d );

    return d;
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
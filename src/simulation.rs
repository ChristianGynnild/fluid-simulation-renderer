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


fn dens_step(N:i32, mut x:Vec<f32>, mut x0:Vec<f32>, u:&Vec<f32>, v:&Vec<f32>, diff:f32, dt:f32) -> (Vec<f32>, Vec<f32>)
{
    x = add_source( N, x, &x0, dt );
    (x,x0) = (x0, x); 
    x = diffuse(N, 0, x, &x0, diff, dt );
    (x,x0) = (x0, x);
    x = advect( N, 0, x, &x0, u, v, dt );

    return (x, x0);
}

fn vel_step(N:i32, mut u:Vec<f32>, mut v:Vec<f32>, mut u0:Vec<f32>, mut v0:Vec<f32>, visc:f32, dt:f32) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>)
{
    u = add_source(N, u, &u0, dt );
    v = add_source(N, v, &v0, dt);
    (u, u0) = (u0, u); 
    (v, v0) = (v0, v);
    u = diffuse(N, 1, u, &u0, visc, dt);
    v = diffuse(N, 2, v, &v0, visc, dt);
    (u, v, u0, v0) = project(N, u, v, u0, v0);
    (u, u0) = (u0, u); 
    (v, v0) = (v0, v);
    u = advect(N, 1, u, &u0, &u0, &v0, dt );
    v = advect ( N, 2, v, &v0, &u0, &v0, dt);
    (u, v, u0, v0) = project(N, u, v, u0, v0);
}


fn project(N:i32, mut u:Vec<f32>, mut v:Vec<f32>, mut p:Vec<f32>, mut div:Vec<f32>) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>)
{
    let h:f32 = 1.0/N as f32;

    for i in 1..N{
        for j in 1..N{
            div[IX(i,j)] = -0.5*h*(u[IX(i+1,j)]-u[IX(i-1,j)]+
            v[IX(i,j+1)]-v[IX(i,j-1)]);
            p[IX(i,j)] = 0.;
        }
    }
    set_bnd(N, 0, div);
    set_bnd(N, 0, p);
    for k in 0..20{
        for i in 1..N{
            for j in 1..N{
                p[IX(i,j)] = (div[IX( i,j)]+p[IX(i-1,j)]+p[IX(i+1,j)]+
                p[IX(i,j-1)]+p[IX(i,j+1)])/4.;
            }
        }
        set_bnd(N, 0, p);
    }
    for i in 1..N{
        for j in 1..N{
            u[IX(i,j)] -= 0.5*(p[IX(i+1,j)]-p[IX(i-1,j)])/h;
            v[IX(i,j)] -= 0.5*(p[IX(i,j+1)]-p[IX(i,j-1)])/h;
        }
    }
    set_bnd( N, 1, u);
    set_bnd( N, 2, v);

    return (u, v, p, div);
}

fn set_bnd(N:i32, b:i32, mut x:Vec<f32>) -> Vec<f32>
{
    for i in 1..N{
        x[IX(0 ,i)] = match b==1 {true => -x[IX(1,i)], false => x[IX(1,i)]};
        x[IX(N+1,i)] = match b==1 {true => -x[IX(N,i)], false => x[IX(N,i)]};
        x[IX(i,0 )] = match b==2 {true => -x[IX(i,1)], false => x[IX(i,1)]};
        x[IX(i,N+1)] = match b==2 {true => -x[IX(i,N)], false => x[IX(i,N)]};
    }
    x[IX(0 ,0 )] = 0.5*(x[IX(1,0 )]+x[IX(0 ,1)]);
    x[IX(0 ,N+1)] = 0.5*(x[IX(1,N+1)]+x[IX(0 ,N )]);
    x[IX(N+1,0 )] = 0.5*(x[IX(N,0 )]+x[IX(N+1,1)]);
    x[IX(N+1,N+1)] = 0.5*(x[IX(N,N+1)]+x[IX(N+1,N )]);

    return x;
}
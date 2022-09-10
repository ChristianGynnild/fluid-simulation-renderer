use image::RgbImage;

const N:usize = super::N as usize;

fn save_image(vector:Vec<u8>, filepath:&str){
    RgbImage::from_raw(N as u32, N as u32, vector).unwrap().save(filepath).unwrap();
}


pub fn render(density:&Vec<f32>, filepath:&str){
    let mut image:Vec<u8> = vec![0;N*N*3];
    let color:[u8;3] = [7,252,166];
    let mut density_value:f32;
    let mut index:usize;

    for x in 0..N{
        for y in 0..N{
            density_value = density[(y+1)*(N+2)+(x+1)];
            index=(N-1-y)*N*3+x*3;

            image[index+0] = (density_value*color[0] as f32) as u8;
            image[index+1] = (density_value*color[1] as f32) as u8;
            image[index+2] = (density_value*color[2] as f32) as u8;
        }
    }
    
    save_image(image, &filepath);
}
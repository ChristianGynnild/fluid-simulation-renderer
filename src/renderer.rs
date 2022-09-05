use image::RgbImage;

const WIDTH:usize = super::WIDTH;
const HEIGHT:usize = super::HEIGHT;

fn save_image(vector:Vec<u8>, filepath:&str){
    RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, vector).unwrap().save(filepath).unwrap();
}


pub fn render(density:&Vec<f32>, filepath:&str){
    let mut image:Vec<u8> = vec![0;WIDTH*HEIGHT*3];
    let color:[u8;3] = [7,252,166];
    let mut density_value:f32;
    let mut index:usize;

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            density_value = density[(y+1)*(WIDTH+2)+(x+1)];
            index=(HEIGHT-1-y)*WIDTH*3+x*3;

            image[index+0] = (density_value*color[0] as f32) as u8;
            image[index+1] = (density_value*color[1] as f32) as u8;
            image[index+2] = (density_value*color[2] as f32) as u8;
        }
    }
    
    save_image(image, &filepath);
}
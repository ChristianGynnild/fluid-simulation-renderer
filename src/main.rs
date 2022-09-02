use image::RgbImage;


const width:usize = 32;
const height:usize = 32;

fn save_image(vector:Vec<u8>){
    RgbImage::from_raw(width as u32, height as u32, vector);
}

fn main(){    
    let density  = [[[0];height];width];
    let density0 = [[[0];height];width];

    let velocity  = [[[0,0];height];width];
    let velocity0 = [[[0,0];height];width];
}
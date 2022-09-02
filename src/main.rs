use image::RgbImage;


const width:u32 = 32;
const height:u32 = 32;

fn save_image(vector:Vec<u8>){
    RgbImage::from_raw(width, height, vector);
}

fn main(){    

}
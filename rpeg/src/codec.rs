#![allow(unused)]
#[allow(non_snake_case)]
#[allow(unused_imports)]
use csc411_image::{RgbImage, Read, Write};
use bitpack::bitpack::{newu,news,getu,gets};

pub fn compress(filename: Option<&str>) {
    let image = RgbImage::read(filename).unwrap();

    let height = image.height & !1_u32;
    let width = image.width & !1_u32;

    let mut packed = vec![];

    println!("Compressed image format 2\n{} {}", width, height);

    for row in (0..height as usize).step_by(2){
        for col in (0..width as usize).step_by(2){
            let u_width = width as usize;
            let top_left = image.pixels[row * u_width + col].clone();
            let top_right = image.pixels[row * u_width + (col + 1)].clone();
            let bottom_left = image.pixels[(row + 1) * u_width + col].clone();
            let bottom_right = image.pixels[(row + 1) * u_width + (col + 1)].clone();
            let word = c_process(top_left, top_right, bottom_left, bottom_right, image.denominator);
            packed.push(word.to_be_bytes());
        }
    }
    //println!("{:?}", packed);
    csc411_rpegio::output_rpeg_data(&packed, width, height)

}

pub fn c_process(top_left: csc411_image::Rgb, top_right: csc411_image::Rgb, bottom_left: csc411_image::Rgb, bottom_right: csc411_image::Rgb, denom: u16) -> u32{

    let float_block = convert_to_float(top_left, top_right, bottom_left, bottom_right, denom);
    let color_space_block = convert_to_cs(float_block);
    let word = pack_bits(color_space_block) as u32;
    //print!("{:?}", (word as u32).to_be_bytes());
    //csc411_rpegio::output_rpeg_data(&[(word as u32).to_be_bytes()], width, height);
    word

}

pub fn convert_to_float(top_left: csc411_image::Rgb, top_right: csc411_image::Rgb, bottom_left: csc411_image::Rgb, bottom_right: csc411_image::Rgb, denom: u16) -> Vec<Vec<f64>>{
    let f_tl = vec![top_left.red as f64/ denom as f64, top_left.green as f64/ denom as f64, top_left.blue as f64/ denom as f64];
    let f_tr = vec![top_right.red as f64/ denom as f64, top_right.green as f64/ denom as f64, top_right.blue as f64/ denom as f64];
    let f_bl = vec![bottom_left.red as f64/ denom as f64, bottom_left.green as f64/ denom as f64, bottom_left.blue as f64/ denom as f64];
    let f_br = vec![bottom_right.red as f64/ denom as f64, bottom_right.green as f64/ denom as f64, bottom_right.blue as f64/ denom as f64];

    vec![f_tl, f_tr, f_bl, f_br]
}

pub fn convert_to_cs(float_block: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let cs_tl = vec![ 0.299 * float_block[0][0] + 0.587 * float_block[0][1] + 0.114 * float_block[0][2],
                      -0.168736 * float_block[0][0] - 0.331264 * float_block[0][1] + 0.5 * float_block[0][2],
                      0.5 * float_block[0][0] - 0.418688 * float_block[0][1] - 0.081312 * float_block[0][2]];
    let cs_tr = vec![ 0.299 * float_block[1][0] + 0.587 * float_block[1][1] + 0.114 * float_block[1][2],
                      -0.168736 * float_block[1][0] - 0.331264 * float_block[1][1] + 0.5 * float_block[1][2],
                      0.5 * float_block[1][0] - 0.418688 * float_block[1][1] - 0.081312 * float_block[1][2]];
    let cs_bl = vec![ 0.299 * float_block[2][0] + 0.587 * float_block[2][1] + 0.114 * float_block[2][2],
                      -0.168736 * float_block[2][0] - 0.331264 * float_block[2][1] + 0.5 * float_block[2][2],
                      0.5 * float_block[2][0] - 0.418688 * float_block[2][1] - 0.081312 * float_block[2][2]];
    let cs_br = vec![ 0.299 * float_block[3][0] + 0.587 * float_block[3][1] + 0.114 * float_block[3][2],
                      -0.168736 * float_block[3][0] - 0.331264 * float_block[3][1] + 0.5 * float_block[3][2],
                      0.5 * float_block[3][0] - 0.418688 * float_block[3][1] - 0.081312 * float_block[3][2]];

    vec![cs_tl, cs_tr, cs_bl, cs_br]

}

pub fn pack_bits(color_space_block: Vec<Vec<f64>>) -> u64 {
    let avg_pb = ((color_space_block[0][1] + color_space_block[1][1] + color_space_block[2][1] + color_space_block[3][1]) / 4.0) as f32;
    let avg_pr = ((color_space_block[0][2] + color_space_block[1][2] + color_space_block[2][2] + color_space_block[3][2]) / 4.0) as f32;

    let pb_chroma_index = csc411_arith::index_of_chroma(avg_pb);
    let pr_chroma_index = csc411_arith::index_of_chroma(avg_pr);

    let cos_co = do_dct(color_space_block);

    let bcd = convert_to_bits(&cos_co);

    let a = (cos_co[0] * 511.0).round() as u32;
    let b = bcd[0];
    let c = bcd[1];
    let d = bcd[2];

    let mut word: u64 = 0;
    word = newu(word, 4, 0, pr_chroma_index.try_into().unwrap()).unwrap();
    word = newu(word, 4, 4, pb_chroma_index.try_into().unwrap()).unwrap();
    word = news(word, 5, 8, d.into()).unwrap();
    word = news(word, 5, 13, c.into()).unwrap();
    word = news(word, 5, 18, b.into()).unwrap();
    word = newu(word, 9, 23, a.into()).unwrap();

    word
}

pub fn do_dct(color_space_block: Vec<Vec<f64>>) -> Vec<f64>{

    let y1 = color_space_block[0][0];
    let y2 = color_space_block[1][0];
    let y3 = color_space_block[2][0];
    let y4 = color_space_block[3][0];

    let a = (y4 + y3 + y2 + y1)/4.0;
    let b = (y4 + y3 - y2 - y1)/4.0;
    let c = (y4 - y3 + y2 - y1)/4.0;
    let d = (y4 - y3 - y2 + y1)/4.0;

    vec![a,b,c,d]
}

pub fn convert_to_bits(cos_co: &Vec<f64>) -> Vec<i32>{

    let b = ((cos_co[1] * 50.0).round() as i32).clamp(-15,15);
    let c = ((cos_co[2] * 50.0).round() as i32).clamp(-15,15);
    let d = ((cos_co[3] * 50.0).round() as i32).clamp(-15,15);

    vec![b,c,d]

}

pub fn decompress(filename: Option<&str>) {

    let (raw_bytes, width, height) = csc411_rpegio::read_in_rpeg_data(filename).unwrap();

    let u_width = width as usize;
    let u_height = height as usize;

    let mut col = 0_usize;
    let mut row = 0_usize;

    let mut image_pixels = vec![csc411_image::Rgb { red: 0, green: 0, blue: 0 }; (width * height) as usize];

    for i in 0..raw_bytes.len() {
        let word = u32::from_be_bytes(raw_bytes[i]);
        let rgb_block = d_process(word);
        //println!("{:?}", rgb_block);

        if col >= (width) as usize{
            col = 0;
        }

        let tl = csc411_image::Rgb { red: rgb_block[0][1] as u16, green: rgb_block[0][1] as u16, blue: rgb_block[0][2] as u16 };
        image_pixels[get_index(col,row, u_width, u_height).unwrap()] = tl;
        let tr = csc411_image::Rgb { red: rgb_block[1][0] as u16, green: rgb_block[1][1] as u16, blue: rgb_block[1][2] as u16 };
        image_pixels[get_index(col + 1,row, u_width, u_height).unwrap()] = tr;
        let bl = csc411_image::Rgb { red: rgb_block[2][0] as u16, green: rgb_block[2][1] as u16, blue: rgb_block[2][2] as u16 };
        image_pixels[get_index(col,row + 1, u_width, u_height).unwrap()] = bl;
        let br = csc411_image::Rgb { red: rgb_block[3][0] as u16, green: rgb_block[3][1] as u16, blue: rgb_block[3][2] as u16 };
        image_pixels[get_index(col + 1,row + 1, u_width, u_height).unwrap()] = br;

        col += 2;
        if col >= width as usize && row < (height - 2) as usize{
            row += 2
        }
    }

    // let mut image_pixels: Vec<csc411_image::Rgb> = image.iter_row_major().map(|(_,_,T)| {T.clone()}).collect();
    //println!("{:?}", image_pixels);

    let uncompressed = csc411_image::RgbImage {
        pixels: image_pixels.clone(),
        width: width as u32,
        height: height as u32,
        denominator: 255,
    };

    //uncompressed.write(None);

}

pub fn d_process(word: u32) -> Vec<[f64;3]>{

    let dct = from_bits(word);
    let y_block = convert_to_y(dct);
    let rgb_block = convert_to_rgb(y_block);

    println!("{:?}", rgb_block);
    rgb_block

}

pub fn from_bits(word: u32) -> Vec<f64> {
    let a = (getu(word.into(), 9, 23) as f64 / 511.0).round();
    let b = ((gets(word.into(), 5, 18) as f64) / 50.0).clamp(-0.3, 0.3);
    let c = ((gets(word.into(), 5, 13) as f64) / 50.0).clamp(-0.3, 0.3);
    let d = ((gets(word.into(), 5, 8) as f64) / 50.0).clamp(-0.3, 0.3);
    let pb_chroma_index = getu(word.into(), 4, 4);
    let pr_chroma_index = getu(word.into(), 4, 0);

    let pb = csc411_arith::chroma_of_index(pb_chroma_index as usize) as f64;
    let pr = csc411_arith::chroma_of_index(pr_chroma_index as usize) as f64;

    //println!("{:?}",vec![a,b,c,d,pb,pr]);
    vec![a,b,c,d,pb,pr]
}

pub fn convert_to_y(dct: Vec<f64>) -> Vec<f64> {

    let a = dct[0];
    let b = dct[1];
    let c = dct[2];
    let d = dct[3];
    let pb = dct[4];
    let pr = dct[5];

    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;

    vec![y1,y2,y3,y4,pb,pr]

}

pub fn convert_to_rgb(y_block: Vec<f64>) -> Vec<[f64; 3]>{

    let pb = y_block[4];
    let pr = y_block[5];
    let mut rgb_block = vec![];

    for i in 0..y_block.len() - 2 {
        let r = 1.0 * y_block[i] + 0.0 * pb + 1.402 * pr;
        let g = 1.0 * y_block[i] - 0.344136 * pb - 0.714136 * pr;
        let b = 1.0 * y_block[i] + 1.772 * pb + 0.0 * pr;

        rgb_block.push([r,g,b])
    }

    //println!("{:?}", rgb_block);

    rgb_block

}

fn get_index(c: usize, r: usize, width: usize, height: usize) -> Option<usize> {
    if c < width && r < height {
        Some(r * width + c)
    } else {
        None
    }
}
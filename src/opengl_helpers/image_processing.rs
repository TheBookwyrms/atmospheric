use std::fs;

use crate::opengl_helpers::enums::{ImageFormat, PPMType};

//use numeracy::matrices::Matrix;
use numeracy::matrices::Matrix;

use zune_jpeg;
use zune_png;


pub struct PPM {
    pub type_:PPMType,
    pub width:usize,
    pub height:usize,
    pub max_colour_val:u16,
    ///// shape of data is [3*width, height]
    /// a lie, because the real shape should be [3*width, height]
    pub data:Vec<u8>,
}


#[derive(Debug, Clone)]
pub struct Image {
    //pub raw:Vec<u8>,
    //pub pixels:Vec<u8>,
    pub width:i32,
    pub height:i32,
    pub nchannels:usize,
    //pub data:Matrix<u8>,
    pub data:Vec<u8>,
    //pub data:&'a [u8],
    pub format:ImageFormat,
}
impl Image {

    fn get_data_from_bytes(file_bytes:&[u8], format:ImageFormat) -> (Vec<u8>, usize, usize, usize) {

        //let file_text: Vec<char> = file_bytes.iter().map(|a| *a as char).collect();
        let file_text: String = file_bytes.iter().map(|a| *a as char).collect();
        //let file_text: String = file_text.into_iter().collect();
        let file_text:Vec<&str> = file_text.split("\n").collect();

        let (pixels, width, height, nchannels) = match format {
            ImageFormat::JPEG => {
                let mut decoder = zune_jpeg::JpegDecoder::new(file_bytes.clone());
                let pixels = decoder.decode().unwrap();
                let (width, height) = decoder.dimensions().unwrap();
                let nchannels = decoder.get_output_colorspace().unwrap().num_components();
                (pixels, width, height, nchannels)
            },
            ImageFormat::PNG  => {
                let mut decoder = zune_png::PngDecoder::new(file_bytes.clone());
                let pixels = decoder.decode().unwrap().u8().unwrap();
                let (width, height) = decoder.get_dimensions().unwrap();
                let nchannels = decoder.get_colorspace().unwrap().num_components();
                (pixels, width, height, nchannels)
            },
            ImageFormat::PPMP3 => {
                assert_eq!("P3", file_text[0]);

                let wh = file_text[1].split(" ").collect::<Vec<&str>>();
                let width = wh[0].parse::<usize>().unwrap();
                let height = wh[1].parse::<usize>().unwrap();

                let mut ppm_data :Vec<u8> = vec![];

                for line in file_text[3..file_text.len()-1].to_vec() {
                    let rgb = line.split(" ").collect::<Vec<&str>>();
                    if rgb.len() != 0 {
                        for i in rgb {
                            //println!("{}", i);
                            ppm_data.push(i.parse::<u8>().unwrap());
                        }
                    }
                }

                (ppm_data, width, height, 3)
            }
        };

        (pixels, width, height, nchannels)
    }

    fn get_data_from_bytes_vec(file_bytes:Vec<u8>, format:ImageFormat) -> (Vec<u8>, usize, usize, usize) {

        //let file_text: Vec<char> = file_bytes.iter().map(|a| *a as char).collect();
        let file_text: String = file_bytes.iter().map(|a| *a as char).collect();
        //let file_text: String = file_text.into_iter().collect();
        let file_text:Vec<&str> = file_text.split("\n").collect();

        let (pixels, width, height, nchannels) = match format {
            ImageFormat::JPEG => {
                let mut decoder = zune_jpeg::JpegDecoder::new(file_bytes.clone());
                let pixels = decoder.decode().unwrap();
                let (width, height) = decoder.dimensions().unwrap();
                let nchannels = decoder.get_output_colorspace().unwrap().num_components();
                (pixels, width, height, nchannels)
            },
            ImageFormat::PNG  => {
                let mut decoder = zune_png::PngDecoder::new(file_bytes.clone());
                let pixels = decoder.decode().unwrap().u8().unwrap();
                let (width, height) = decoder.get_dimensions().unwrap();
                let nchannels = decoder.get_colorspace().unwrap().num_components();
                (pixels, width, height, nchannels)
            },
            ImageFormat::PPMP3 => {
                assert_eq!("P3", file_text[0]);

                let wh = file_text[1].split(" ").collect::<Vec<&str>>();
                let width = wh[0].parse::<usize>().unwrap();
                let height = wh[1].parse::<usize>().unwrap();

                let mut ppm_data :Vec<u8> = vec![];

                for line in file_text[3..file_text.len()-1].to_vec() {
                    let rgb = line.split(" ").collect::<Vec<&str>>();
                    if rgb.len() != 0 {
                        for i in rgb {
                            //println!("{}", i);
                            ppm_data.push(i.parse::<u8>().unwrap());
                        }
                    }
                }

                (ppm_data, width, height, 3)
            }
        };

        (pixels, width, height, nchannels)
    }

    fn flip_vertically(pixels:Vec<u8>, width:usize, height:usize, nchannels:usize) -> Vec<u8> {
        let mut flipped_vec = Vec::with_capacity(pixels.len());
        let w = width*nchannels;
        let h = height;
        for i in 0..h {
            let row_slice = &pixels[(h-i-1)*w..(h-i)*w];
            flipped_vec.extend_from_slice(row_slice);
        }
        flipped_vec
    }

    pub fn decode_from_bytes(file_bytes:&[u8], format:ImageFormat, flip:bool) -> Image {

        let (pixels, width, height, nchannels) = Self::get_data_from_bytes(file_bytes, format);

        let data = if flip {
            Self::flip_vertically(pixels, width, height, nchannels)
        } else {
            pixels
        };

        Image {
            //raw: file_bytes,
            //pixels,
            width:width.try_into().unwrap(),
            height:height.try_into().unwrap(),
            nchannels,
            data:data,
            format
        }
    }

    pub fn decode_from_bytes_vec(file_bytes:Vec<u8>, format:ImageFormat, flip:bool) -> Image {

        let (pixels, width, height, nchannels) = Self::get_data_from_bytes_vec(file_bytes, format);

        let data = if flip {
            Self::flip_vertically(pixels, width, height, nchannels)
        } else {
            pixels
        };

        Image {
            //raw: file_bytes,
            //pixels,
            width:width.try_into().unwrap(),
            height:height.try_into().unwrap(),
            nchannels,
            data:data,
            format
        }
    }

    pub fn decode_from_path(path:&str, format:ImageFormat, flip:bool) -> Image {
        let file_bytes = fs::read(path).unwrap();

        let (pixels, width, height, nchannels) = Self::get_data_from_bytes_vec(file_bytes, format);

        let data = if flip {
            Self::flip_vertically(pixels, width, height, nchannels)
        } else {
            pixels
        };


        
        //let pixels_matrix = Matrix {shape:[width*nchannels, height], array:pixels.clone()};
        //let data = if flip {
        //    pixels_matrix.flip_vertically()
        //} else {
        //    pixels_matrix
        //};

        Image {
            //raw: file_bytes,
            //pixels,
            width:width.try_into().unwrap(),
            height:height.try_into().unwrap(),
            nchannels,
            data:data,
            format
        }
    }

    pub fn decode_from_ppm(ppm:PPM, flip:bool) -> Image {

        let data = if flip {
            Self::flip_vertically(ppm.data, ppm.width, ppm.height, 3)
        } else {
            ppm.data
        };
        //let data = if flip {
        //    ppm.data.flip_vertically()
        //} else {
        //    ppm.data
        //};

        Image {
            //raw: file_bytes,
            //pixels,
            width:ppm.width.try_into().unwrap(),
            height:ppm.height.try_into().unwrap(),
            nchannels:3,
            data:data,
            format:ppm.type_.into(),
        }
    }
}
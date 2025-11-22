use std::fs;

use crate::enums::ImageFormat;

use numeracy::matrices::Matrix;
use numeracy::enums::MatrixDataTypes;

use zune_jpeg;
use zune_png;


pub struct Image {
    pub raw:Vec<u8>,
    pub pixels:Vec<u8>,
    pub width:i32,
    pub height:i32,
    pub nchannels:usize,
    pub data:Matrix<u8>,
    pub format:ImageFormat,
}
impl Image {
    pub fn decode(path:&str, format:ImageFormat, flip:bool) -> Image {
        let file_bytes = fs::read(path).unwrap();
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
        };

        
        let data_matrix = Matrix {shape:vec![width*nchannels, height], array:pixels.clone(), dtype:MatrixDataTypes::U8};
        let data = if flip {
            data_matrix.flip_vertically().unwrap()
        } else {
            data_matrix
        };

        Image {
            raw: file_bytes,
            pixels,
            width:width.try_into().unwrap(),
            height:height.try_into().unwrap(),
            nchannels,
            data,
            format
        }
    }
}
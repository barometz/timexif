// Timexif is copyright © 2016, Dominic van Berkel <dev+timexif@baudvine.net>
// Distributed under the MIT open source license; see the file LICENSE.txt for
// details.

//! The filters module takes a sequence of `Image` and removes all images that
//! do or do not match certain criteria. Primarily, this is intended for
//! removing images that are too dark.

use std::fs::File;
use std::io::BufReader;

use jpeg_decoder as jpeg;

use ::Image;

/// True when an image is at least this bright on average, false otherwise.
pub fn filter_dark(image: &Image, min_brightness: u8) -> bool {
    match mean_brightness(image) {
        Ok(b)  => b >= min_brightness,
        Err(_) => false
    }
}

fn mean_brightness(image: &Image) -> Result<u8, jpeg::Error> {
    // The file should really be known to exist at this point. 
    let file = File::open(&image.path).unwrap();
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let mut data = try!(decoder.decode());

    let info = decoder.info().unwrap();
    // Convert CMYK (naively?) to RGB so there's only one measure of
    // brightness
    if info.pixel_format == jpeg::PixelFormat::CMYK32 {
        data = cmyk_to_rgb(&mut data);
    }

    let mut total: u64 = 0;
    for n in &data {
        total += *n as u64;
    }

    let mean = total / data.len() as u64;
    
    Ok(mean as u8)
}

/// cmyk_to_rgb borrowed from jpeg-decoder example, licensed under MIT /
/// Apache-2.0 by Ulf Nilsson (https://github.com/kaksmet/jpeg-decoder/)
fn cmyk_to_rgb(input: &[u8]) -> Vec<u8> {
    let size = input.len() - input.len() / 4;
    let mut output = Vec::with_capacity(size);
    
    for pixel in input.chunks(4) {
        let c = pixel[0] as f32 / 255.0;
        let m = pixel[1] as f32 / 255.0;
        let y = pixel[2] as f32 / 255.0;
        let k = pixel[3] as f32 / 255.0;

        // CMYK -> CMY
        let c = c * (1.0 - k) + k;
        let m = m * (1.0 - k) + k;
        let y = y * (1.0 - k) + k;

        // CMY -> RGB
        let r = (1.0 - c) * 255.0;
        let g = (1.0 - m) * 255.0;
        let b = (1.0 - y) * 255.0;

        output.push(r as u8);
        output.push(g as u8);
        output.push(b as u8);
    }

    output
}

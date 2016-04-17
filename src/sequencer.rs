// Timexif is copyright © 2016, Dominic van Berkel <dev+timexif@baudvine.net>
// Distributed under the MIT open source license; see the file LICENSE.txt for
// details.

//! The sequencer module walks through the provided path, finds all
//! EXIF-tagged images inside it and returns a list of matching Images.

extern crate rexif;
extern crate walkdir;

use std::path::Path;

use self::walkdir::WalkDir;

use ::Image;

pub fn get_images<P: AsRef<Path>>(dir: P) -> Vec<Image> {
    WalkDir::new(dir).into_iter()
        // First, all DirEntries that are not errors
        .filter_map(|d| d.ok())
        // Convert to String so we can pass it around more easily
        .filter_map(|d| d.path().to_str().map(String::from))
        .filter_map(to_image)
        .collect::<Vec<Image>>()
}

fn get_exif_datetime(path: &String) -> Option<String> {
    let tagvalue = match rexif::parse_file(path) {
        Ok(record) => record.entries.iter()
            // See if any tags are DateTimeOriginal
            .find(|&e| e.tag == rexif::ExifTag::DateTimeOriginal)
            .map(|e| e.value.clone()),
        Err(_) => return None
    };
    
    match tagvalue {
        Some(tagvalue) => match tagvalue {
            rexif::TagValue::Ascii(dt) => Some(dt),
            _ => None
        },
        _ => None
    }
}

fn to_image(path: String) -> Option<Image> {
    get_exif_datetime(&path)
        .and_then(|dt| Image::from_exif_datetime(&path, &dt).ok())
}

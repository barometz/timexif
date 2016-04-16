// Timexif is copyright © 2016, Dominic van Berkel <dev+timexif@baudvine.net>
// Distributed under the MIT open source license; see the file LICENSE.txt for
// details.

//! The sequencer module turns a collection or glob of paths to images into
//! a sequence of `Image` with known time.

use std::path::{Path, PathBuf};
use rexif::ExifData;
use walkdir::{IterFilterEntry, DirEntry, WalkDir};

use ::Image;

pub fn get_images<P: AsRef<Path>>(dir: P) -> Vec<Image> {
    WalkDir::new(dir).into_iter()
        .filter_map(|r| r.ok())
        .filter(contains_exif)
        .filter(has_timestamp)
        .map(to_image)
        .collect::<Vec<Image>>()
}

fn contains_exif(path: &DirEntry) -> bool {
    unimplemented!();
}

fn has_timestamp(path: &DirEntry) -> bool {
    unimplemented!();
}

fn to_image(path: DirEntry) -> Image {
    unimplemented!();
}

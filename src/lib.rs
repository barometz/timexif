// Timexif is copyright © 2016, Dominic van Berkel <dev+timexif@baudvine.net>
// Distributed under the MIT open source license; see the file LICENSE.txt for
// details.

//! Lib crate for timexif, an EXIF-timestamp-based time lapse creation tool.

extern crate chrono;

use std::path::PathBuf;
use chrono::NaiveDateTime;

pub mod filters;
pub mod framing;
pub mod sequencer;

/// An image, with a path to the file and a timestamp indicating when the
/// image was created. Generally, the timestamp will be produced from EXIF
/// data, which doesn't include a timezone, which is why this uses `chrono`'s
/// `NaiveDateTime`
#[derive(Debug)]
pub struct Image {
    pub path: PathBuf,
    pub timestamp: NaiveDateTime,
}

impl Image {
    /// Marginally more convenient constructor for Image, saving you a
    /// conversion on the path.
    pub fn new(path: &str, timestamp: NaiveDateTime)
               -> Image {
        Image { path: PathBuf::from(path), timestamp: timestamp, }
    }

    /// Constructor that converts EXIF datetimes
    pub fn from_exif_datetime(path: &str, timestamp: &str)
                              -> Result<Image, chrono::ParseError> {
        // Standard EXIF datetime format
        let fmt = "%Y:%m:%d %H:%M:%S";
        NaiveDateTime::parse_from_str(timestamp, fmt)
            .and_then(|ts| Ok(Image::new(path, ts)))
    }
}

#[cfg(test)]
mod tests {
    use ::Image;
    use chrono::*;
    
    #[test]
    fn image_new_from_exif() {
        let p = "DSC_1001.jpg";
        let ts = "2003:08:11 16:45:32";
        let img = Image::from_exif_datetime(p, ts).unwrap();
        
        assert_eq!(img.path.to_str(), Some(p));
        assert_eq!(img.timestamp.year(), 2003);
        assert_eq!(img.timestamp.minute(), 45);
    }
}

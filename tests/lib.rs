extern crate chrono;

extern crate timexif;

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

const SIMPLE: &'static str = "tests/sample-imgs/simple/";
const BRIGHTNESS: &'static str = "tests/sample-imgs/brightness";

/// There are four images, two of which have valid timestamps.
#[test]
fn correct_number_of_images() {
    let imgs = timexif::sequencer::get_images(SIMPLE);
    assert_eq!(imgs.len(), 2);
}

#[test]
fn correct_timestamps() {
    let dt = |y, m, d, h, min, s|
    NaiveDateTime::new(NaiveDate::from_ymd(y, m, d), NaiveTime::from_hms(h, min, s));

    let mut expected = HashMap::new();

    expected.insert(format!("{}{}", SIMPLE, "large from dslr.JPG"),
                    dt(2014, 7, 12, 11, 24, 56));
    expected.insert(format!("{}{}", SIMPLE, "phone photo.jpg"),
                    dt(2016, 4, 16, 12, 27, 14));

    let imgs = timexif::sequencer::get_images(SIMPLE);
    for img in imgs {
        let path = img.path.to_str().unwrap();
        assert!(expected.contains_key(path));
        let dt = expected.get(path).unwrap();
        assert_eq!(dt, &img.timestamp);
    }
}

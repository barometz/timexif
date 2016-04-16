extern crate chrono;

extern crate timexif;

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// There are four images, two of which have valid timestamps.
#[test]
fn correct_number_of_images() {
    let imgs = timexif::sequencer::get_images("tests/sample-imgs");
    assert_eq!(imgs.len(), 2);
}

#[test]
fn correct_timestamps() {
    let dt = |y, m, d, h, min, s|
    NaiveDateTime::new(NaiveDate::from_ymd(y, m, d), NaiveTime::from_hms(h, min, s));

    let mut expected = HashMap::new();

    expected.insert("tests/sample-imgs/large from dslr.JPG",
                    dt(2014, 7, 12, 11, 24, 56));
    expected.insert("tests/sample-imgs/phone photo.jpg",
                    dt(2016, 4, 16, 12, 27, 14));

    let imgs = timexif::sequencer::get_images("tests/sample-imgs");
    for img in imgs {
        let path = img.path.to_str().unwrap();
        assert!(expected.contains_key(path));
        let dt = expected.get(path).unwrap();
        assert_eq!(dt, &img.timestamp);
    }
}

extern crate chrono;

extern crate timexif;

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use timexif::*;

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

    let imgs = sequencer::get_images(SIMPLE);
    for img in imgs {
        let path = img.path.to_str().unwrap();
        assert!(expected.contains_key(path));
        let dt = expected.get(path).unwrap();
        assert_eq!(dt, &img.timestamp);
    }
}

/// All of the images are at least zero bright - that is, none are unfairly
/// filtered out.
#[test]
fn brightness_all_ge_zero() {
    let imgs = sequencer::get_images(BRIGHTNESS);
    let filtered = imgs.iter()
        .filter(|i| filters::filter_dark(i, 0))
        .collect::<Vec<&Image>>();
    assert_eq!(imgs.len(), filtered.len());
}

/// None of the images are measured to be maximally bright (so don't put a
/// fully white jpeg in there)
#[test]
fn brightness_none_ge_max() {
    let imgs = sequencer::get_images(BRIGHTNESS);
    let filtered = imgs.iter()
        .filter(|i| filters::filter_dark(i, 255))
        .collect::<Vec<&Image>>();
    assert_eq!(0, filtered.len());
}

/// The right number of images is filtered out with some threshold. I suspect
/// I may need to do away with the plant images and generate some proper test
/// images instead.
#[test]
fn brightness_threshold() {
    let imgs = sequencer::get_images(BRIGHTNESS);
    let filtered = imgs.iter()
        .filter(|i| filters::filter_dark(i, 90))
        .collect::<Vec<&Image>>();
    assert_eq!(4, filtered.len());

    let filtered = imgs.iter()
        .filter(|i| filters::filter_dark(i, 20))
        .collect::<Vec<&Image>>();
    assert_eq!(5, filtered.len());
}

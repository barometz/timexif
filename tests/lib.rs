extern crate timexif;

#[test]
fn do_the_thing() {
    let imgs = timexif::sequencer::get_images("tests/sample-imgs");
    assert_eq!(imgs.len(), 2);
}

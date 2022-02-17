use adder; // needed because each files are separate crate
mod common;

// no need to annotate #[cfg(test)]
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two_v2(2));
}
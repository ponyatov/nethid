//! # `nethid` tests

#![cfg(test)]

use crate::*;

#[test]
/// dummy test /always ok/
fn any() {
    assert_eq!(1, 1);
}

#[test]
fn sdl_context() {
    let scr = Screen::new(String::from(""));
    assert_eq!(scr.argv, "");
    assert_eq!(scr.w, W);
    assert_eq!(scr.h, H);
}

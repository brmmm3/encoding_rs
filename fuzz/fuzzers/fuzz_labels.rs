#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate encoding_rs;
use encoding_rs2::*;

fuzz_target!(|data: &[u8]| {
    Encoding::for_label(data);
});

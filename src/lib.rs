
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rust_password;

#[wasm_bindgen]
pub fn get_next(length: usize, num_digits: usize, num_symbols: usize, no_upper: bool, allow_repeat: bool) -> String {
    match rust_password::generate(length, num_digits, num_symbols, no_upper, allow_repeat) {
        Ok(v) => v,
        Err(_ex) => "Exception occured".to_string()
    }
}

#[wasm_bindgen]
pub fn get_default() -> String {
    match rust_password::generate(52, 10, 5, false, false) {
        Ok(v) => v,
        Err(_ex) => "Exception occured".to_string()
    }
}
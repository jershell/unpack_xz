extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decompress_xz(data: &[u8]) -> Vec<u8> {
    let mut buffer = std::io::BufReader::new(data);
    let mut out: Vec<u8> = Vec::new();
    lzma_rs::xz_decompress(&mut buffer, &mut out).unwrap();
    out
}
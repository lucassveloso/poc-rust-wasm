extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn filter_even_numbers(vec: Vec<u32>) -> Vec<u32> {
  return vec.into_iter().filter(|x| x%2 == 0).collect::<Vec<u32>>()
}

#[wasm_bindgen]
pub extern fn fibonacci(n: u32) -> u32 {
  if n <= 1 { return 1 }
  fibonacci(n - 1) + fibonacci(n - 2)
}
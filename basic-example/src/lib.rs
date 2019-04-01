extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../domUtils")]
extern { 
  fn appendStringToBody(str: String);
}

#[wasm_bindgen]
pub fn greeting(name: String) {
  let result = format!("Hello {}!", name);
  appendStringToBody(result);
}
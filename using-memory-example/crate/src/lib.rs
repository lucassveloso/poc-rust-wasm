use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let h1: web_sys::Node = document.create_element("h1")?.into();
    h1.set_text_content(Some("Hi! These colored squares were created using shared data in memory. This is awesome!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&h1)?;

    Ok(())
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

#[wasm_bindgen]
pub struct Image {
  pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
  pub fn new() -> Image {
    let red = Color { red: 255, green: 0, blue: 0 };
    let green = Color { red: 0, green: 255, blue: 0 };
    let blue = Color { red: 0, green: 0, blue: 255 };
    let pixels = vec![red, green, blue];
    Image {
      pixels
    }
  }

  pub fn pixels_ptr(&self) -> *const Color {
    self.pixels.as_ptr()
  }
}
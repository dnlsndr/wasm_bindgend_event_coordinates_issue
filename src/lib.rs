use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_event_coordinates(event: web_sys::PointerEvent) {
    let cx = event.client_x();
    let sx = event.screen_x();
    let x = event.x();
    let ox = event.offset_x();
    console_log!("Log from rust: {}, {}, {}, {}", cx, sx, x, ox);
}

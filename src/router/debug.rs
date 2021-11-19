
use std::fmt::{Debug, Display};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn logv<T: Debug>(value: T) {
    log(&format!("Value: {:?}", &value));
}


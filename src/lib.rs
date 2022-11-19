use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test2() -> u8 {
    10
}

// #[wasm_bindgen]
// //#[wasm_bindgen(getter_with_clone)]
// pub struct Rudolph {
//     speed: u32,
//     lumens: u32,
// }

// #[wasm_bindgen]
// pub enum Common {
//     Santa(Santa),
//     Rudolph(Rudolph),
// }

#[wasm_bindgen]
pub fn return_boxed_js_value_slice() -> Box<[JsValue]> {
    vec![
        JsValue::NULL,
        JsValue::UNDEFINED,
        JsValue::from("hello"),
        // Rudolph {
        //     speed: 10,
        //     lumens: 10,
        // }
        // .into(),
    ]
    .into_boxed_slice()
}

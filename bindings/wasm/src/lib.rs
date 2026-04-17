use wasm_bindgen::prelude::*;

/// Looks up the ISO 3166-1 alpha-2 country code for an IPv4 or IPv6 address.
///
/// Accepts either a single address string or an array of address strings.
/// Returns a string (or `null`) for a single input, or an array of
/// string-or-null for an array input. Throws `TypeError` on any other input.
#[wasm_bindgen]
pub fn country_code(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        return Ok(match ::iptocc::country_code(&s) {
            Some(cc) => JsValue::from_str(cc),
            None => JsValue::NULL,
        });
    }

    if !js_sys::Array::is_array(&input) {
        return Err(JsError::new("country_code expects a string or an array of strings"));
    }

    let arr = js_sys::Array::from(&input);
    let len = arr.length();
    let out = js_sys::Array::new_with_length(len);
    for i in 0..len {
        let item = arr.get(i);
        let cell = match item.as_string().and_then(|s| ::iptocc::country_code(&s)) {
            Some(cc) => JsValue::from_str(cc),
            None => JsValue::NULL,
        };
        out.set(i, cell);
    }
    Ok(out.into())
}

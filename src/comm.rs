use wasm_bindgen::prelude::*;

// default subtitle
static mut PREV: String = String::new();

// sleep function
#[wasm_bindgen(module = "/src/reset.js")]
extern "C" {
    fn reset(prev: String, time: u32);
}

// change subtitle to given message
fn send(msg: String, error: bool) {

    // setup element
    let element: web_sys::Element =  web_sys::window().expect("").document()
        .expect("").get_element_by_id("subtitle").unwrap();

    // set default subtitle
    unsafe { if PREV.is_empty() { PREV = element.inner_html(); } }

    // set message and color
    element.set_inner_html(&msg);
    element.set_attribute("style",
        if error { "color: var(--error)" }
        else     { "color: var(--info) " }
    ).unwrap();

    // return to title and style
    unsafe { reset(PREV.clone(), 1000); }
}

// send info
pub fn info(msg: String) {
    send(msg, false);
}

// send error
pub fn error(msg: String) {
    send(msg, true);
}

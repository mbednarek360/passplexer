use wasm_bindgen::prelude::*;

// default subtitle
static mut PREV: String = String::new();

// sleep function
#[wasm_bindgen(module = "/src/reset.js")]
extern "C" {
    fn reset(prev: String, time: u32);
}

// change subtitle to given message
fn send(doc: &web_sys::Document, msg: String, error: bool) {
    let element: web_sys::Element = doc.get_element_by_id("subtitle").unwrap();

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
pub fn info(doc: &web_sys::Document, msg: String) {
    send(doc, msg, false);
}

// send error
pub fn error(doc: &web_sys::Document, msg: String) {
    send(doc, msg, true);
}

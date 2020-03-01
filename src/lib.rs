use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};
mod pass;
pub mod comm;

// javascript interop
#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    fn copy_clipboard(text: String);
}

// run on load
#[wasm_bindgen]
pub fn main() {

    // ease debugging
    console_error_panic_hook::set_once();

    // setup dom
    let document: web_sys::Document = web_sys::window().expect("").document().expect("");

    // attach event listener on button press
    EventListener::new(&document.get_element_by_id("gen_button").unwrap(), "click", move |_event| {

        // send pass generated password to js
        copy_clipboard(pass::pass_gen(
            document.get_element_by_id("username_input")
                    .unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value(),
            document.get_element_by_id("master_input")
                    .unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value(),
            document.get_element_by_id("service_input")
                    .unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap().value()
                    .parse::<u64>().unwrap()));
    }).forget();
}

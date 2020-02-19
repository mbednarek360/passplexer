use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};
//use web_sys::*;
mod pass;

// console log macro
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// copy text
fn copy_string(input: String) {




    web_sys::ClipboardEvent::new("copy");
}

// run on load
#[wasm_bindgen]
pub fn main() {

    // setup dom
    let window: web_sys::Window = web_sys::window().expect("");    
    let document: web_sys::Document = window.document().expect("");
    
    // attach event listener
    EventListener::new(&document.get_element_by_id("gen_button").unwrap(), "click", move |_event| {
        console_log!("{}", pass::pass_gen(
            document.get_element_by_id("username_input")
                    .unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value(),
            document.get_element_by_id("master_input")
                    .unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value(),
            document.get_element_by_id("salt_input")
                    .unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
                    .parse::<u64>().unwrap()
        ));
    }).forget();
}

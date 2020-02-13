use wasm_bindgen::prelude::*;
use gloo::{events::EventListener};
//mod pass;

// run on load
#[wasm_bindgen]
pub fn setup() {

    // setup dom
    let window: web_sys::Window = web_sys::window().expect("");    
    let document: web_sys::Document = window.document().expect("");
    
    // attach event listeners
    EventListener::new(&document.get_element_by_id("main_button").unwrap(), "click", move |_event| {

    }).forget()




}

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use reqwest::*;

// read local storage
#[wasm_bindgen(module = "/src/id.js")]
extern "C" {
    fn get_source() -> String;
}

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


// read csv
// load from url into selection box

// for testing
pub fn load_services() -> HashMap<u64, String> {

    // init hash map
    let mut service_map: HashMap<u64, String> = HashMap::new();

    // load elements
    console_log!("{}", get_source());
   
    let body = reqwest::blocking::get(&get_source()).unwrap().text();

    console_log!("{:?}", &body);

    // get data from url
    // loop through each line and seperate by comma
    // add to list




    // return
    service_map
}

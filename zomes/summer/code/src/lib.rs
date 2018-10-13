#[macro_use]
extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::globals::G_MEM_STACK;

genesis! {
 true
}

zome_functions! {
    sum: |num1: u32, num2: u32| {
        let sum = num1 + num2;
        let string_sum: String = sum.to_string();
        return json!({sum: string_sum});
    }
}


#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate boolinator;

fn handle_sum(num1: u32, num2: u32) -> serde_json::Value {
    let sum = num1 + num2;
    return json!({"sum": format!("{}",sum)});
}

define_zome! {
    entries: []

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            sum: {
                inputs: |num1: u32, num2: u32|,
                outputs: |post: serde_json::Value|,
                handler: handle_sum
            }
        }
    }
}

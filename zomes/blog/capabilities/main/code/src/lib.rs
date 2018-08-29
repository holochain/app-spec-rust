extern crate hdk;
#[macro_use]
extern crate serde_json;
use std::time::SystemTime;


#[no_mangle]
pub extern "C" fn create_post(input: serde_json::Value) -> serde_json::Value {
    match hdk::commit_entry("post", json!(
        {
            "content": input["content"],
            "date_created": SystemTime::now()
        }
    )) {
        Ok(post_hash) => {
            hdk::link_entries(hdk::APP_AGENT_KEY_HASH.to_string(), post_hash.clone(), "authored_posts");

            let in_reply_to = input["in_reply_to"].to_string();
            if in_reply_to != "" {
                if let Ok(_) = hdk::get_entry(in_reply_to.clone()) {
                    hdk::link_entries(in_reply_to, post_hash.clone(), "comments");
                }
            }

            json!({"hash": post_hash})
        },
        Err(_) => json!({"error": "commit failed"})
    }
}

#[no_mangle]
pub extern "C" fn posts_by_agent(input: serde_json::Value) -> serde_json::Value {
    match hdk::get_links(input["agent"].to_string(), "authored_posts") {
        Ok(links) => json!({"post_hashes": links}),
        Err(hdk_error) => hdk_error.to_json(),
    }
}

#[no_mangle]
pub extern "C" fn get_post(input: serde_json::Value) -> serde_json::Value {
    match hdk::get_entry(input["post_hash"].to_string()) {
        Ok(entry) => json!({"post":  entry}),
        Err(hdk_error) => hdk_error.to_json(),
    }
}

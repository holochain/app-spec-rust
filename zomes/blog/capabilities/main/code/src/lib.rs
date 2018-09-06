#[macro_use]
extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
//use std::time::SystemTime;

zome_functions! {
    create_post: |content: String, in_reply_to: String| {
        hdk::start_bundle();

        match hdk::commit_entry("post", json!(
            {
                "content": content,
                "date_created": "now"//SystemTime::now()
                //SystemTime::now() panics when executed in wasmi
            }
        )) {
            Ok(post_hash) => {
                hdk::link_entries(hdk::APP_AGENT_KEY_HASH.to_string(), post_hash.clone(), "authored_posts");

                let in_reply_to = in_reply_to;
                if !in_reply_to.is_empty() {
                    if let Ok(_) = hdk::get_entry(in_reply_to.clone()) {
                        hdk::link_entries(in_reply_to, post_hash.clone(), "comments");
                    }
                }

                hdk::finish_bundle();

                json!({"hash": post_hash})
            },
            Err(_) => {
                hdk::cancel_bundle();
                json!({"error": "commit failed"})
            }
        }
    }

    posts_by_agent: |agent: String| {
        hdk::get_links(agent, "authored_posts")
            .and_then(|links| json!({"post_hashes": links}))
            .unwrap_or_else(|hdk_error| hdk_error.to_json())
    }

    get_post: |post_hash: String| {
        hdk::get_entry(post_hash)
            .and_then(|entry| json!({"post":  entry}))
            .unwrap_or_else(|hdk_error| hdk_error.to_json())
    }
}

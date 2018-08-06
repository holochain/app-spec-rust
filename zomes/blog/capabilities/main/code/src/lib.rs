extern crate hdk;
#[macro_use]
extern crate serde_json;
use std::time::SystemTime;

fn create_post(input: serde_json::Value) -> serde_json::Value {
    let post_hash = hdk::commit("post", json!(
        {
            "content": input["content"],
            "date_created": SystemTime::now()
        }
    ));

    hdk::link(hdk::anchors::agent_key_hash(), "authored_posts", post_hash.clone());

    let in_reply_to = input["in_reply_to"].to_string();
    if in_reply_to != "" {
        if let Some(_) = hdk::get(in_reply_to.clone()) {
            hdk::link(in_reply_to, "comments", post_hash.clone());
        }
    }

    json!({"hash": post_hash})
}

fn posts_by_agent(input: serde_json::Value) -> serde_json::Value {
    let links = hdk::get_links(input["agent"].to_string(), "authored_posts");
    json!({"post_hashes": links})
}

fn get_post(input: serde_json::Value) -> serde_json::Value {
    json!({"post": hdk::get(input["post_hash"].to_string()) })
}

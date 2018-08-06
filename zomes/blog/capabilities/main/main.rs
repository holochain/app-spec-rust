extern crate hdk;
extern crate serde_json;
use std::time::SystemTime;

fn create_post(input: serde_json::Value) -> serde_json::Value {
    let post_hash = hdk::commit("post", json!(
        {
            "content": input["content"],
            "date_created": SystemTime::now()
        }
    ));

    hdk::link(hdk::anchors::agent_key_hash(), "authored_posts", post_hash);

    let in_repl_to = input["in_reply_to"];
    if in_reply_to != "" {
        if Ok(commented_post) = hdk::get(in_reply_to) {
            hdk::link(in_repl_to, "comments", post_hash);
        }
    }

    json!({"hash": post_hash})
}

fn posts_by_agent(input: serde_json::Value) -> serde_json::Value {
    let links = get_links(input["agent"], "authored_posts");
    json!({"post_hashes": links})
}

fn get_post(input: serde_json::Value) -> serde_json::Value {
    json!({"post": get(input["post_hash"]) })
}

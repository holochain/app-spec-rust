use hdk;
use hdk::globals::G_MEM_STACK;
use serde_json;

zome_functions! {
    create_post: |content: String, in_reply_to: String| {
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

                json!({"hash": post_hash})
            },
            Err(hdk_error) => hdk_error.to_json(),
        }
    }

    posts_by_agent: |agent: String| {
        match hdk::get_links(agent, "authored_posts") {
            Ok(links) => json!({"post_hashes": links}),
            Err(hdk_error) => hdk_error.to_json(),
        }
    }

    get_post: |post_hash: String| {
        match hdk::get_entry(post_hash) {
            Ok(maybe_entry) => {
                match maybe_entry {
                    Some(entry) => match serde_json::from_str(&entry) {
                        Ok(post) => post,
                        Err(err) => json!({"error deserializing post": err.to_string()}),
                    },
                    None => json!({}),
                }
            },
            Err(hdk_error) => hdk_error.to_json(),
        }
    }
}

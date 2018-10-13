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
                hdk::link_entries(hdk::APP_AGENT_KEY_HASH.to_string(), post_hash.clone(), "authored_posts")?;

                let in_reply_to = in_reply_to;
                if !in_reply_to.is_empty() {
                    // check the entry exists, before linking
                    if let Ok(_) = hdk::get_entry(in_reply_to.clone()) {
                        hdk::link_entries(in_reply_to, post_hash.clone(), "comments")?;
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
        // get_entry returns a Result<<Option<String>>, RibosomeError>
        // It's a RibosomeError if something went wrong.
        // The Option<String> means we can either find the requested
        // entry or not (both are not errrors).
        match hdk::get_entry(post_hash) {
            // In the case we don't get an error
            // it might be an entry ...
            Ok(maybe_entry) => match maybe_entry {
                // ...so we match on that Option<String>
                // If it is some String we expect that string
                // to hold a stringified JSON object.
                // serde_json::from_str() tries to deserialize
                // that String into a serde_json::Value and
                // returns a result:
                Some(entry) => match serde_json::from_str(&entry) {
                    // In case deserialization worked, we return
                    // that object as it is:
                    Ok(post) => post,
                    // This error means that the string in `entry`
                    // is not a stringified JSON which should not
                    // happen but might be a bug somewhere else:
                    Err(err) => json!({"error deserializing post": err.to_string()}),
                },
                // If get_entry() could not find an entry with the given
                // hash, we just return an empty JSON object:
                None => json!({}),
            },
            // In case of an error we just use RibosomeError's
            // to_json() function to return that error
            Err(hdk_error) => hdk_error.to_json(),
        }
    }
}

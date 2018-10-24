use hdk::{
    self,
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions, GetResultStatus,
    },
    holochain_wasm_utils::holochain_core_types::hash::HashString,
    AGENT_INITIAL_HASH,
};

pub fn handle_create_post(content: String, in_reply_to: HashString) -> serde_json::Value {
    match hdk::commit_entry("post", json!(
        {
            "content": content,
            "date_created": "now"//SystemTime::now()
            //SystemTime::now() panics when executed in wasmi
        }
    )) {
        Ok(post_hash) => {
            let link_result = hdk::link_entries(
                &HashString::from(AGENT_INITIAL_HASH.to_string()),
                &post_hash,
                "authored_posts"
            );

            if link_result.is_err() {
                return json!({"link error": link_result.err().unwrap()})
            }

            let in_reply_to = in_reply_to;
            if !in_reply_to.to_string().is_empty() {
                if let Ok(_) = hdk::get_entry(in_reply_to.clone(), GetEntryOptions{}) {
                    let _ = hdk::link_entries(&in_reply_to, &post_hash, "comments");
                }
            }
            json!({"hash": post_hash})
        }
        Err(hdk_error) => hdk_error.to_json(),
    }
}

pub fn handle_posts_by_agent(agent: HashString) -> serde_json::Value {
    match hdk::get_links(&agent, "authored_posts") {
        Ok(result) => json!({"post_hashes": result.links}),
        Err(hdk_error) => hdk_error.to_json(),
    }
}

pub fn handle_my_posts() -> serde_json::Value {
    match hdk::get_links(&HashString::from(AGENT_INITIAL_HASH.to_string()), "authored_posts") {
        Ok(result) => json!({"post_hashes": result.links}),
        Err(hdk_error) => hdk_error.to_json(),
    }
}

pub fn handle_get_post(post_hash: HashString) -> serde_json::Value {
    // get_entry returns a Result<GetEntryResult, RibosomeError>
    // It's a RibosomeError if something went wrong
    // The contents of the GetEntryResult will depend on the options passed
    // in to get_entry()
    match hdk::get_entry(post_hash,GetEntryOptions{}) {
        // In the case we don't get an error
        // it might be an entry ...
        Ok(result) => match result.status {
            // ...so we match on that ResultStatus
            // If it is Found then we can use
            // serde_json::from_str() to deserialize
            // the result.entry a serde_json::Value and
            // returns a result:
            GetResultStatus::Found => match serde_json::from_str(&result.entry) {
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
            GetResultStatus::NotFound => json!({}),
        },
        // In case of an error we just use RibosomeError's
        // to_json() function to return that error
        Err(hdk_error) => hdk_error.to_json(),
    }
}

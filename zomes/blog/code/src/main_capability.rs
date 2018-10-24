use hdk::{
    self, error::ZomeApiError, holochain_wasm_utils::api_serialization::get_entry::GetEntryOptions,
    holochain_wasm_utils::holochain_core_types::hash::HashString, AGENT_INITIAL_HASH,
};

use post::Post;

zome_functions! {
    create_post: |content: String, in_reply_to: HashString| {
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
                    if let Ok(_) = hdk::get_entry_result(in_reply_to.clone(),GetEntryOptions{}) {
                        let _ = hdk::link_entries(&in_reply_to, &post_hash, "comments");
                    }
                }

                json!({"hash": post_hash})
            },
            Err(hdk_error) => hdk_error.to_json(),
        }
    }

    posts_by_agent: |agent: HashString| {
        match hdk::get_links(&agent, "authored_posts") {
            Ok(result) => json!({"post_hashes": result.links}),
            Err(hdk_error) => hdk_error.to_json(),
        }
    }

    my_posts: | | {
        match hdk::get_links(&HashString::from(AGENT_INITIAL_HASH.to_string()), "authored_posts") {
            Ok(result) => json!({"post_hashes": result.links}),
            Err(hdk_error) => hdk_error.to_json(),
        }
    }

    get_post: |post_hash: HashString| {
        // get_entry returns a Result<Option<T>, ZomeApiError>
        // where T is the type that you used to commit the entry, in this case a Blog
        // It's a ZomeApiError if something went wrong (i.e. wrong type in deserialization)
        // Otherwise its a Some(T) or a None
        let result : Result<Option<Post>,ZomeApiError> = hdk::get_entry(post_hash);
        match result {
            // In the case we don't get an error
            // it might be an entry ...
            Ok(Some(post)) => json!(post),
            Ok(None) =>  json!({}),

            // This error means that the string in `entry`
            // is not a stringified JSON which should not
            // happen but might be a bug somewhere else:
            Err(err) => json!({"error deserializing post": err.to_string()}),
        }
    }
}

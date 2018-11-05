
use hdk::error::ZomeApiError;
use hdk::holochain_core_types::error::HolochainError;
use hdk::holochain_core_types::json::default_to_json;
use hdk::{
    self,
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions,
    },
    holochain_core_types::hash::HashString,
    holochain_core_types::json::JsonString,
    holochain_core_types::entry::Entry,
    holochain_core_types::entry_type::EntryType,
    AGENT_ADDRESS,
};

use post::Post;

pub fn handle_check_sum(num1: u32, num2: u32) -> JsonString {
    #[derive(Serialize, Deserialize, Debug, DefaultJson)]
    struct SumInput {
        num1: u32,
        num2: u32,
    };

    let call_input = SumInput {
        num1: num1,
        num2: num2,
    };
    let maybe_result = hdk::call("summer", "main", "sum", call_input.into());
    match maybe_result {
        Ok(result) => result.into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

pub fn handle_hash_post(content: String) -> JsonString {
    let post_entry = Entry::new(EntryType::App("post".into()), 
        Post {
            content: content.to_string(),
            date_created: "now".to_string()
        }
    );


    match hdk::hash_entry(&post_entry) {
        Ok(address) => json!({"address": address}).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

pub fn handle_create_post(content: String, in_reply_to: HashString) -> JsonString {

    let post_entry = Entry::new(EntryType::App("post".into()), 
        Post {
            content: content.to_string(),
            date_created: "now".to_string()
        }
    );

    match hdk::commit_entry(&post_entry) {
        Ok(post_hash) => {
            let link_result = hdk::link_entries(
                &HashString::from(AGENT_ADDRESS.to_string()),
                &post_hash,
                "authored_posts"
            );

            if link_result.is_err() {
                return json!({"link error": link_result.err().unwrap()}).into()
            }

            let in_reply_to = in_reply_to;
            if !in_reply_to.to_string().is_empty() {
                if let Ok(_) = hdk::get_entry_result(in_reply_to.clone(), GetEntryOptions{}) {
                    let _ = hdk::link_entries(&in_reply_to, &post_hash, "comments");
                }
            }
            json!({"address": post_hash}).into()
        }
        Err(hdk_error) => hdk_error.into(),
    }
}

pub fn handle_posts_by_agent(agent: HashString) -> JsonString {
    match hdk::get_links(&agent, "authored_posts") {
        Ok(result) => json!({"post_hashes": result}).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

pub fn handle_my_posts() -> JsonString {
    match hdk::get_links(&HashString::from(AGENT_ADDRESS.to_string()), "authored_posts") {
        Ok(result) => json!({"post_hashes": result}).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}


pub fn handle_my_posts_as_commited() -> JsonString {
    // in the current implementation of hdk::query the second parameter
    // specifies the maximum number of items to return, with 0 meaning all.
    // future versions will also include more parameters for more complex
    // queries.
    match hdk::query("post",0) {
        Ok(posts) => json!({"post_hashes": posts}).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}
pub fn handle_get_post(post_address: HashString) -> JsonString {
    // get_entry returns a Result<Option<T>, ZomeApiError>
    // where T is the type that you used to commit the entry, in this case a Blog
    // It's a ZomeApiError if something went wrong (i.e. wrong type in deserialization)
    // Otherwise its a Some(T) or a None
    let result : Result<Option<Entry>,ZomeApiError> = hdk::get_entry(post_address);
    match result {
        // In the case we don't get an error
        // it might be an entry ...
        Ok(Some(entry)) => {
            entry.value().to_owned()
        },
        Ok(None) =>  json!({}).into(),

        // This error means that the string in `entry`
        // is not a stringified JSON which should not
        // happen but might be a bug somewhere else:
        Err(err) => json!({"error deserializing post": err.to_string()}).into(),
    }
}

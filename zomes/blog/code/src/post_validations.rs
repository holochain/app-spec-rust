// THIS IS NOT FINAL
// There are several things missing and not worked out yet.
// Like:
//   - validate_* functions need to receive a complex struct as argument
//     that holds the entry to be validated next to the sources,
//     validation package etc.
//   - return type of these callbacks should be Result<..>, right?
//   - how to declare/write validation packages?
//   - do we need a guarding macro like zome_fns!?
//
// This file is WIP and placeholder to be overwritten by future
// PR after completing the design discussion and reaching consensus
// about these specifics.
use serde_json;

#[derive(Serialize, Deserialize)]
struct Post {
    content: String,
    date_created: String
}

#[no_mangle]
pub extern "C" fn validate_commit(_offset: i32) -> i32{
    0
}

#[no_mangle]
pub extern "C" fn validate_post(payload: String) -> bool {
    serde_json::from_str::<Post>(&payload).is_ok()
}

#[no_mangle]
pub extern "C" fn validate_post_comments() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn validate_post_authored_posts() -> bool {
    true
}

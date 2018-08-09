extern crate hdk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct Post {
    content: String,
    date_created: String
}

#[no_mangle]
pub extern "C" fn validate_post(payload: String) -> bool {
    match serde_json::from_str::<Post>(&payload) {
        Ok(_) => true,
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn validate_post_comments() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn validate_post_authored_posts() -> bool {
    true
}

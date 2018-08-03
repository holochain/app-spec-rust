extern crate hdk;
#[macro_use]
extern crate serde_derive;

struct Post {
    content: String,
    date_created: String
}

fn validate_post(payload: String) -> bool {
    match serde_json::from_str(&payload) {
        Ok(Post) => true,
        _ => false
    }
}

fn validate_post_comments() -> bool {
    true
}

fn validate_post_authored_posts() -> bool {
    true
}

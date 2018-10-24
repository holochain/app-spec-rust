#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate boolinator;

pub mod blog;
pub mod post;

use hdk::holochain_core_types::hash::HashString;

define_zome! {
    entries: [
        post::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            create_post: {
                inputs: |content: String, in_reply_to: HashString|,
                outputs: |hash: String|,
                handler: blog::handle_create_post
            }

            posts_by_agent: {
                inputs: |agent: HashString|,
                outputs: |post_hashes: Vec<HashString>|,
                handler: blog::handle_posts_by_agent
            }

            get_post: {
                inputs: |post_hash: HashString|,
                outputs: |post: serde_json::Value|,
                handler: blog::handle_get_post
            }

            my_posts: {
                inputs: | |,
                outputs: |post_hashes: Vec<HashString>|,
                handler: blog::handle_my_posts
            }
        }
    }
}

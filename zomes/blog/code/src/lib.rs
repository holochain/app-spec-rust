#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate boolinator;

pub mod main;
pub mod post;


define_zome! {
    entries [
        post::definition()
    ],

    genesis: {
        true
    }

    zome_functions {
        main (public) {
            create_post {
                inputs: |content: String, in_reply_to: HashString|,
                output: |hash: String|,
                func: main::create_post,
            },

            posts_by_agent: {
                inputs: |agent: HashString|,
                outputs: |post_hashes: Vec<HashString>|,
                func: main::posts_by_agent,
            },

            get_post: {
                inputs: |post_hash: HashString|,
                outputs: |post: serde_json::Value|,
                func: main::get_post,
            }
        }
    }
}

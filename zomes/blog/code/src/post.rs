/// This file holds everything that represents the "post" entry type.
use boolinator::*;
use hdk::{self, entry_definition::ValidatingEntryType, holochain_dna::zome::entry_types::Sharing};
use serde_json;

/// We declare the structure of our entry type with this Rust struct.
/// It will be checked automatically by the macro below, similar
/// to how this happens with functions parameters and zome_functions!.
///
/// So this is our normative schema definition:
#[derive(Serialize, Deserialize)]
pub struct Post {
    content: String,
    date_created: String,
}

/// This is what creates the full definition of our entry type.
/// The entry! macro is wrapped in a function so that we can have the content
/// in this file but call it from zome_setup() in lib.rs, which is like the
/// zome's main().
///
/// We will soon be able to also replace the json files that currently hold
/// most of these values. The only field that is really used is the
/// validation_package callback.
/// The validation_function still has to be defined with the macro below.
pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "post",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },

        validation_function: |_entry: Post, _ctx: hdk::ValidationData| {
            Err(String::from("Not in use yet. Will replace validations! macro below soon."))
        }
    )
}

/// This macro wraps validation functions similar to how
/// zome_functions! takes care of argument serialization.
/// The fist arguments type (here Post) can be set to anything that
/// implements serde::Deserialize, like our Post struct above.
/// Schema check is then handled automatically by the code produced by this macro.
validations! {
    [ENTRY] validate_post {
        |post: Post, _ctx: hdk::ValidationData| {
            (post.content.len() < 280)
                .ok_or_else(|| String::from("Content too long"))
        }
    }
}

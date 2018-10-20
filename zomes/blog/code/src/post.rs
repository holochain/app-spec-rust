use boolinator::*;
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_dna::zome::entry_types::Sharing,
};
use serde_json;

/// We declare the structure of our entry type with this Rust struct.
/// It will be checked automatically by the macro below, similar
/// to how this happens with functions parameters and zome_functions!.
#[derive(Serialize, Deserialize)]
struct Post {
    content: String,
    date_created: String,
}

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

/// We need a macro that wraps those validation functions similar to
/// zome_functions!, but with different parameters.
validations! {
    [ENTRY] validate_post {
        |post: Post, _ctx: hdk::ValidationData| {
            (post.content.len() < 280)
                .ok_or_else(|| String::from("Content too long"))
        }
    }
}

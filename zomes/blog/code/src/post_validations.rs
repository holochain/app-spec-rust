use serde_json;

/// We declare the structure of our entry type with this Rust struct.
/// It will be checked automatically by the macro below, similar
/// to how this happens with functions parameters and zome_functions!.
#[derive(Serialize, Deserialize)]
struct Post {
    content: String,
    date_created: String,
    allow_comments: bool,
}

/// We need a macro that wraps those validation functions similar to
/// zome_functions!, but with different parameters.
validations! {

    /// We also need to distinguish between the validation of an entry
    /// (here) and validation of links (below)
    [ENTRY] validate_post {

        /// This is an enum that needs to be added to the HDK and that tells
        /// Holochain what validation package the source needs to send
        /// to validating DHT nodes.
        /// Should be something like:
        ///
        /// enum ValidationPackage {
        ///    Entry,           //sending only the entry
        ///    ChainEntries,    //sending all (public?) source chain entries
        ///    ChainHeaders,    //sending all source chain headers
        ///    ChainFull,       //sending the whole chain, entries and headers
        ///    Custom(String),  //sending something custom
        /// }
        [ValidationPackage::ChainFull]

        /// Here we have the validation function itself.
        /// Deserializing the JSON happens behind the scenes and we get
        /// a Post object.
        /// ValidationData needs to be added to the HDK and should look
        /// like this:
        ///
        /// struct ValidationData {
        ///    sources : Vec<HashString>
        ///    sourceChainEntries : Option<Vec<serde_json::Value>>
        ///    sourceChainHeaders : Option<Vec<Header>>
        ///    custom : Option<serde_json::Value>
        /// }
        |post: Post, ctx: hdk::ValidationData| {
            post.content.len() < 280
        }
    }

    /// Validation functions for links need to have a different signature:
    [LINK] validate_post_comments: |base: HashString, target: HashString, ctx: hdk::ValidationData| {
        let base = hdk::get_entry(base);
        base.allow_comments
    }

    [LINK] validate_post_authored_posts: |base: HashString, target: HashString, ctx: hdk::ValidationData| {
        ctx.sources[0] == base
    }
}

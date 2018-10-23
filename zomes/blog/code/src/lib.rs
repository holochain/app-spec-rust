#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate boolinator;

pub mod hc_lifecycle_capability;
pub mod main_capability;
pub mod post;

use hdk::meta::ZomeDefinition;

/// This function needs to be implemented by every zome.
/// Every entry type needs to be registered by passing its
/// definition object (constructed by entry!) to the define
/// function.
///
/// Will soon be replaced by a define_zome! macro
#[no_mangle]
pub extern fn zome_setup(zd: &mut ZomeDefinition) {
    zd.define(post::definition());
}

extern crate hdk;
mod hdk_membrane;


// Following @sphinxc0re's suggestion for a capabilities macro
// that also produces the JSON for the whole capability
// including all functions.
capabilities!{

    // Assuming a system capability that we can implement and thus
    // override membrane defaults (which we'll have to set).
    // (system) is the visibilty - introduced a new one.
    hc_membrane (system) : {

        // The join membrane is the first to pass.
        // Not passing means no node would talk to you
        // (not even answer a get/routing request)
        hc_join_membrane: |object: MembraneObject| {
            true
        },

        // This hooks into the DHT code and makes it possible
        // to define when we trust their DHT responses.
        // This is super important to make the DHT Sybil resistant
        // even though the app itself is permissionless (as is the case here).
        //
        // In this example we need to restrict the DHT with the same
        // custom "like_membrane" (defined below) because that membrane
        // uses the DHT for its implementation.
        // Would every node be inlcuded in the DHT (and trusted), attackers
        // could easily fool a node into believing their constructed
        // like-reality.
        hc_dht_membrane: |object: MembraneObject| {
            hdk::check_membrane("liked", object.agent_key())
        },

        // This is a custom membrane that we can use in zome functions
        // and in entry validations.
        // (My intention is to let only "liked" people comment on posts).
        //
        // This is basically an implementation of social triangulation.
        // You need a certain number of people who are "liked" to also
        // like you.
        //
        // Of course, this only works when we now with whom to start.
        // Those seed agents as well as the number of needed likes
        // are properties, i.e. defined in and thus part of the DNA.
        liked_membrane: |object: MembraneObject| {
            let agent = object.agent_key();
            // Get the seed agents that are defined in the DNA
            let seeds : Vec<String> = hdk::properties("seeds");

            // Seeds are liked by definition
            if seeds.include(agent) {
                true
            } else {
                 // I'm using a LikedNotarization entry to constrain
                 // the transitive requirement of this membrane
                 // (that only likes of liked ones are valid).
                 // We don't want to run a depth-search on the like-graph
                 // every time we have to check the membrane - instead
                 // we use the trusted DHT as cache:
                 if hdk::get_links(agent, "like_notarization").size() > 0 {
                    true
                } else {
                    // Ok, no seed agent and no cached notarization yet.
                    // We have to check ourselves:
                    let likes = hdk::get_links(agent, "received_likes");
                    let min_likes = hdk::properties("required_number_of_likes");
                    // if the number of likes is less than need, don't bother..
                    if likes.size() >= min_likes {
                        // But if we have enough likes we need to check
                        // how many of them are actually valid, i.e. coming
                        // from a liked source
                        let num_valid_likes = likes.iter()
                            .filter(|like| {
                                let like_source = hdk::get_links(like, "source")[0];
                                hdk::check_membrane(like_source)
                            })
                            .collect::<Vec<Like>>()
                            .size();

                        // If that number is high enough, we have to creat the
                        // notarization entry:
                        if num_valid_likes > min_likes {
                            hdk::commit("liked_notarization", json!({
                                liked_agent_key: agent,
                                date: now(),
                            }))
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            }
        }
    },

    main (public) : {
        like: |agent_key: String| {
            hdk::commit("like", json!({
                liked_agent_key: agent_key,
            }))
        }
    }
}

// I'm also defining a macro for entry types.
// This is supposed to spit out rust structs and the JSON
// definition at the same time, similiar to capabilities!.
entry_types!{
    // Turns into:
    //
    // struct Like {
    //   liked_agent_key : String,
    // }
    //
    // and:
    //
    //  {
    //    "name": "like",
    //    "sharing": "public",
    //    "links_to": [
    //      {
    //        "target_type": "HcSysAgentKeyHash",
    //        "tag": "target"
    //      },
    //      {
    //        "target_type": "HcSysAgentKeyHash",
    //        "tag": "source"
    //      }
    //    ],
    //    "linked_from": [
    //      {
    //        "base_type": "HcSysAgentKeyHash",
    //        "tag": "received_likes"
    //      },
    //      {
    //        "base_type": "HcSysAgentKeyHash",
    //        "tag": "given_likes"
    //      }
    //    ]
    //  }
    Like (public):
        | linkes_to HcSysAgentKeyHash with target |
        | linkes_to HcSysAgentKeyHash with source |
        | linked_from HcSysAgentKeyHash as received_likes |
        | linked_from HcSysAgentKeyHash as given_likes |

    {
        liked_agent_key : String,
    },

    LikedNotarization (public):
        | linked_from HcSysAgentKeyHash as like_notarization |
        | linkes_to HcSysAgentKeyHash with notary |
    {
        liked_agent_key : String,
        date : String,
    },
}


// And a macro for validation callbacks.
// Very much similiar to the existing zome_functions!.
entry_validations!{

    // A like is only valid if the source is liked.
    validate_like: |like: Like, ctx: hdk::ValidationData| {
        hdk::check_membrane("liked", ctx.sources[0].hash())
    },

    // The like notarization is valid if it was created by
    // a DHT node and if the target is actually liked:
    validate_liked_notarization: |ln: LikedNotarization, ctx: hdk::ValidationData| {
        let source_is_dht = hdk::check_membrane("hc_dht", ctx.sources[0].hash());
        let target_is_liked = hdk::check_membrane("liked", ln.liked_agent_key);

        source_is_dht && target_is_liked
    }
}

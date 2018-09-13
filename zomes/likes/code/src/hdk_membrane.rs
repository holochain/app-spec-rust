struct MembraneObject {

}

trait MembraneObject {
    pub fn get_source_chain() -> Vec<Pair>;
    pub fn query_source_chain(entry_type: String) -> Vec<Pair>;
    pub fn agent_key() -> String;
    pub fn agent_id() -> String;
}

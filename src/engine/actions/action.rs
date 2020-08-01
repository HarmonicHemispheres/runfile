
use uuid::Uuid;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Action {
    hash_id: String,
    script: String,
    flags: Vec<String>,
    attrs: HashMap<String, String>
}



impl Action {
    pub fn new(settings: EngineSettings) -> Engine 
    {
        let hash = Uuid::new_v4();
        Engine {
            hash_id: hash.to_simple().to_string(),
            settings: settings,
            var_store: VarStore::new()
        }
    }

    pub fn insert_var(&mut self, action: Action) {}

    pub fn get_var(&mut self, key: String) {}

    pub fn run_command(&mut self, action: Action) {}
}
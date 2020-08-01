
use uuid::Uuid;
use crate::parser::actions::Action;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct VarStore {
    hash_id: String,
    
    // the global variable store
    g_store: HashMap<String, String>
}



impl VarStore {
    pub fn new() -> VarStore 
    {
        let hash = Uuid::new_v4();
        VarStore {
            hash_id: hash.to_simple().to_string(),
            g_store: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, val: String) {}

    pub fn get(&mut self, key: String) {}
}
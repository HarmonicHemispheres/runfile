
use uuid::Uuid;
use crate::parser::actions::Action;
use crate::engine::settings::EngineSettings;
use crate::engine::storage::VarStore;


#[derive(Debug, Clone)]
pub struct Engine {
    hash_id: String,
    settings: EngineSettings,
    var_store: VarStore
}



impl Engine {
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
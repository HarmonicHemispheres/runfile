// use crate::crypto::digest::Digest;
// use crypto::sha1::Sha1;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Attr {
    value {v: String},
    commands {v: Vec<Action>}
}


#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    name: String,
    hash_id: String,
    attrs: HashMap<String, Attr>,
    val: Option<String>
}



impl Action {
    pub fn new(name: String,
               attrs: HashMap<String, Attr>,
               val: Option<String>
              ) -> Action 
    {
        // let hash = Sha1::new().result_str();
        let hash = Uuid::new_v4();
        Action {
            name: name,
            hash_id: hash.to_simple().to_string(),
            attrs: attrs,
            val: val
        }
    }
}

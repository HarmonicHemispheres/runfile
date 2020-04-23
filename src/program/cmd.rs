use std::collections::HashMap;
use crate::lexer::tokens::*;
use crate::parser::actions::Action;


#[derive(Debug)]
pub enum Setting {
    Bool {v: bool},
    Str {s: String}
}

#[derive(Debug)]
pub struct Cmd {
    cfg: HashMap<String, Setting>,
    scripts: Vec<Action>
}


impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            cfg: HashMap::new(),
            scripts: vec![]
        }
    }

    pub fn add_setting (&mut self, ident: String, value: Setting) {
        self.cfg.insert(ident, value);
    }

    // pub fn add_script (&self, token: Token) {

    // }
}
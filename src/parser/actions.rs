

#[derive(Debug, Clone)]
pub enum Action {
    Value {v: String },
    VarRef {v: String },
    Script {
        attrs: Vec<String>,
        cmd: Vec<Action>
    },
    Variable {
        ident: String,
        value: Vec<Action>
    },
    Command {
        ident: String,
        attrs: Vec<String>,
        scripts: Vec<Action>
    },
    Null
}
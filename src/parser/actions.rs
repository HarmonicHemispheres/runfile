

#[derive(Debug)]
pub enum Action {
    Script {cmd: String},
    Variable {
        ident: String,
        value: String
    },
    Command {
        ident: String,
        attrs: Vec<String>,
        scripts: Vec<Action>
    }
}

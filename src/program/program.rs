use std::collections::HashMap;
use crate::parser::actions::Action;
use std::process::Command;
use super::config::Cli;



pub struct Program {
    actions: Vec<Action>,
    vars: HashMap<String, String>,
    act_idx: usize
}

impl Iterator for Program {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        if self.act_idx < self.actions.len() {
            self.act_idx += 1;
            Some(self.actions[self.act_idx].clone())
        } else {
            None
        }
    }
}

impl Program {
    pub fn new(actions: Vec<Action>) -> Program {
        let vars = HashMap::new();
        let acts = actions.clone();
        Program {
            actions: acts,
            vars: vars,
            act_idx: 0
        }
    }

    fn act_cmd(&mut self, action: &Action) -> Result<String, String>{
        match action {
            Action::Command{ident, attrs, scripts} => {
                for script in scripts {
                    match script {
                        Action::Script{cmd, attrs} => {
                            let res = self.act_script(script);
                            match res {
                                Ok(_) => {},
                                Err(d) => return Err(d)
                            };
                        },
                        _ => {}
                    }
                };
                let fmt = format!("command '{}' completed", ident);
                return Ok(fmt);
            },
            _ =>  return Err("action not a command".to_owned())
        };
    }
    
    fn act_script(&mut self, action: &Action) -> Result<String, String>{

        let mut cmd_str = String::new();
        let mut attrs_dict = HashMap::new();
        match action {
            Action::Script{cmd, attrs} => {
                for s in cmd {
                    match s {
                        Action::Value{v} => {
                            cmd_str.push_str(v);
                        },
                        Action::VarRef{v} => {
                            if self.vars.contains_key(v) {
                                cmd_str.push_str(self.vars.get(v).unwrap())
                            } else {
                                let fmt = format!("variable, '{}' has not been defined", v);
                                return Err(fmt);
                            }
                        },
                        _ => {}
                    }
                }

                for attr in attrs {
                    attrs_dict.insert(attr.to_owned(), 1);
                }
            },
            _ => {}
        };
    
        let args: Vec<&str> = cmd_str.split(" ").collect();
        let msg: String = format!(">> {}", cmd_str);

        // check for plaform specific flags
        let mut should_run = false;
        if (attrs_dict.contains_key("mac") && cfg!(target_os="macos") )|| 
            (attrs_dict.contains_key("win") && cfg!(target_os="windows")) ||
            (attrs_dict.contains_key("linux") && cfg!(target_os="linux")) {
            should_run = true;
        } else if (!attrs_dict.contains_key("mac")) &&
                  (!attrs_dict.contains_key("win")) &&
                  (!attrs_dict.contains_key("linux")) {
            should_run = true;
        }

        if should_run {
            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                for arg in args[1..].iter() {
                    cmd.arg(arg);
                }
            }
            let status = cmd.status().expect("script found error");
            if status.success() {
                Ok(msg)
            } else {
                Err(msg.into())
            }
        } else {
            Ok("not running".to_string())
        }
    }
    
    fn act_variable(&mut self, action: &Action) -> Result<String, String>{
        match action {
            Action::Variable{ident, value} => {
                let mut s = String::new();
                for val in value.iter(){
                    match val {
                        Action::Value{v} => {
                            s.push_str(v);
                        },
                        Action::VarRef{v} => {
                            if self.vars.contains_key(v) {
                                s.push_str(self.vars.get(v).unwrap())
                            } else {
                                let fmt = format!("variable, '{}' has not been defined", v);
                                return Err(fmt);
                            }
                        },
                        _ => {}
                    }
                }
                self.vars.insert(ident.to_owned(), s.to_owned());
            },
            _ => {}
        }
        Ok("defined variable".to_owned())
    }
    
    pub fn run(&mut self, cli: Cli) {
        
        let mut i: usize = 0;
        while i < self.actions.len() {
            // for act in self.actions.iter(){
            let act = &self.actions[i].clone();
            let res = match act {
                Action::Variable{ident, value} => self.act_variable(act),
                Action::Script{cmd, attrs} => self.act_script(act),
                Action::Command{ident, attrs, scripts} => {
                    if cli.cmd == *ident {
                        self.act_cmd(act)
                    } else {
                        Ok("".to_owned())
                    }
                },
                _ => {
                    let fmt = format!("invalid action: {:?}", act);
                    Err(fmt.into())
                }
            };
    
            match res {
                Ok(_) => {},
                Err(d) => {
                    println!("[E]  {}", d);
                    return;
                }
            };

            i += 1;
        }
    }
}
use std::error::Error;
use std::process::{Command, ExitStatus};
// use subprocess::{Exec, Redirection};
use std::io::{self, Write};
use crate::parser::actions::Action;
// use std::env;


fn act_script(script: &String) -> Result<String, Box<dyn Error>>{
    let args: Vec<&str> = script.split(" ").collect();
    let msg: String = format!(">> {}", script);

    let mut cmd = Command::new("powershell");
    for arg in args.iter() {
        &cmd.arg(arg);
    };
    let status = cmd.status()?;
    if status.success() {
        Ok(msg)
    } else {
        Err(msg.into())
    }
    
}

fn act_variable(script: &String) -> Result<String, Box<dyn Error>>{
    Ok("defined variable".to_owned())
}

pub fn run(actions: Vec<Action>, options: Vec<String>) {

    let mut cmd_to_run = String::new();
    if options.len() > 1 {
        cmd_to_run = options[1].to_owned();
    }

    for act in actions.iter(){
        let res = match act {
            Action::Script{cmd} => act_script(cmd),
            Action::Variable{ident, value} => act_variable(value),
            Action::Command{ident, attrs, scripts} => {
                if cmd_to_run == &ident[..] {
                    for script in scripts {
                        match script {
                            Action::Script{cmd} => {
                                let res = act_script(cmd);
                                match res {
                                    Ok(msg) => println!("{} [.]  {}", ident, msg),
                                    Err(d) => {
                                        println!("{} [E]  {}", ident, d);
                                        return;
                                    }
                                };
                            },
                            _ => {}
                        }
                    }
                }
                Ok("".to_owned())
            }
        };

        match res {
            Ok(msg) => {
                if msg.len() > 1 {
                    println!("[.]  {}", msg);
                }
            },
            Err(d) => {
                println!("[E]  {}", d);
                return;
            }
        };
    }
}

use super::actions::Action;
use crate::lexer::tokens::{Token, TokenType};
use crate::utils::logger::{Level, Logger};
// use crate::utils::cmd::{Cmd, Setting};

#[allow(dead_code)]
pub fn parse(s: &String) -> Vec<Vec<&str>> {
    let mut output: Vec<Vec<&str>> = vec![];
    for line in s.lines() {
        match line {
            ln if ln.starts_with(">>") => {
                let args: Vec<&str> = ln.split(" ").collect();
                let polished_args = args[1..].to_owned();
                output.push(polished_args);
            },
            _ => {}
        };
    };
    output
}
    

pub struct Parser<'a> {
    pub actions: Vec<Action>,
    pub tokens: &'a Vec<Token>,
    logger: &'a Logger,
    pos: usize
}


impl<'a> Parser<'a> {
    pub fn new(logger: &'a Logger, tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            actions: vec![],
            tokens: tokens,
            logger: logger,
            pos: 0
        }
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn nextif(&mut self, tok: TokenType) -> Result<(), String>{
        let mtok = self.curr();
        if mtok.t == tok {
            self.next();
            Ok(())
        } else {
            let fmt = format!("expected '{:?}' but got '{:?}'", tok, mtok);
            Err(fmt)
        }
    }

    fn curr(&mut self) -> Token {
        self.tokens[self.pos].clone()
    }

    pub fn show_actions(&self) {
        for (i, act) in self.actions.iter().enumerate() {
            println!("   {}. {:?}", i, act);
        }
        println!();
    }

    pub fn analyze(&mut self) -> Result<&str, String> {
        if self.logger.verbose {
            self.logger.log(
                "Parsing Token List".to_string(), 
                Level::Info
            );
        }
        self.actions = vec![];
        loop {
            let tok = self.curr();
            let res = match tok.t {
                TokenType::Command => self.parse_cmd(),
                TokenType::OpenAttr => self.parse_script_attrs(),
                TokenType::Script => self.parse_script(),
                TokenType::Variable => self.parse_var(),
                TokenType::EOF => break,
                _ => {
                    let n = Action::Null;
                    self.next();
                    Ok(n)
                }
            };

            match &res {
                Ok(_) => {
                    let r_u = res.unwrap();
                    match r_u {
                        Action::Null => {},
                        _ => self.actions.push(r_u)
                    }
                },
                Err(msg) => return Err(msg.to_string())
            };
        };
        if self.logger.verbose {
            self.show_actions();
        }
        Ok("success parsing")
    }

    // ------ PARSING ------
    fn parse_script (&mut self) -> Result<Action, String> {
        let mut vals: Vec<Action> = vec![];
        loop {
            self.next();
            let token = self.curr();
            match token.t {
                TokenType::Value => vals.push(
                    Action::Value{v: token.v.unwrap()}
                ),
                TokenType::VarRef => {
                    self.next();
                    match self.nextif(TokenType::OpenScope) {
                        Ok(_) => {},
                        Err(msg) => return Err(msg)
                    }

                    let tok = self.curr();
                    match tok.t {
                        TokenType::Ident => vals.push(
                            Action::VarRef{ v:tok.v.unwrap() }
                        ),
                        _ => return Err("expected ident in var ref!".to_owned())
                    }
                    
                    self.next();
                    match self.curr().t {
                        TokenType::CloseScope => {},
                        _ => return Err("expected '}' to close token ref".to_owned())
                    }
                },
                _ => break
            }; 
        };
        
        let action = Action::Script {
            cmd: vals,
            attrs: vec![]
        };
        Ok(action)
    }

    fn parse_script_attrs (&mut self) -> Result<Action, String> {
        let mut vals:  Vec<Action> = vec![];
        let mut attrs: Vec<String> = vec![];

        loop {
            let token = self.curr();
            self.next();
            let token = self.curr();
            match token.t {
                TokenType::CloseAttr => break,
                TokenType::Ident => attrs.push(token.v.unwrap().to_owned()),
                TokenType::NewLine | TokenType::Comma => {},
                _ => break
            }
        };
        self.next();

        let script_cmd = self.parse_script();
        match script_cmd.unwrap() {
            Action::Script{attrs,cmd} => vals.extend(&mut cmd.iter().cloned()),
            _ => {}
        }
        
        let action = Action::Script {
            cmd: vals,
            attrs: attrs
        };
        Ok(action)
    }

    fn parse_var (&mut self) -> Result<Action, String> {
        let mut ident = String::new();
        let mut value: Vec<Action> = vec![];
        
        self.next();
        let token = self.curr();
        match token.t {
            TokenType::Ident => {
                ident = token.v.unwrap();
            },
            _ => return Err("Expected Identity for variable!".to_owned())
        };

        // check for equal sign
        self.next();
        let token = self.curr();
        match token.t {
            TokenType::EqualSign => {},
            _ => return Err("Expected '=' for variable!".to_owned())
        };
        
        self.next();
        let token = self.curr();
        match token.t {
            TokenType::Value => {
                value.push(Action::Value{v: token.v.unwrap()});
            },
            TokenType::OpenMLineScope => {
                loop {
                    self.next();
                    let token = self.curr();
                    match token.t {
                        TokenType::Value => {
                            value.push(Action::Value{v: token.v.unwrap()});
                        },
                        TokenType::VarRef => {
                            self.next();
                            match self.nextif(TokenType::OpenScope) {
                                Ok(_) => {},
                                Err(msg) => return Err(msg)
                            }
        
                            let tok = self.curr();
                            match tok.t {
                                TokenType::Ident => value.push(
                                    Action::VarRef{ v:tok.v.unwrap() }
                                ),
                                _ => return Err("expected ident in var ref!".to_owned())
                            }
                            
                            self.next();
                            match self.curr().t {
                                TokenType::CloseScope => {},
                                _ => return Err("expected '}' to close token ref".to_owned())
                            }
                        },
                        TokenType::NewLine => {
                            value.push(Action::Value{v: "\n".to_owned()});
                        },
                        TokenType::CloseMLineScope => break,
                        _ => return Err("Excepted value for multiline variable!".to_owned())
                    }
                }
            },
            _ => return Err("Excepted value for variable!".to_owned())
        };

        let action = Action::Variable {
            ident: ident,
            value: value
        };
        Ok(action)
    }

    fn parse_cmd (&mut self) -> Result<Action, String>{
        let mut ident = String::from("");
        let mut attrs: Vec<String> = vec![];
        let mut scripts: Vec<Action> = vec![];

        // check for ident
        self.next();
        let mut tok = self.curr();
        match tok.t {
            TokenType::Ident => {
                ident.push_str(&tok.v.unwrap()[..]);
                self.next();
            },
            _ => {}
        }

        // check for attributes
        // let mut found_attrs = 0;
        // loop {
        //     tok = self.curr();
        //     match tok.t {
        //         TokenType::OpenAttr => found_attrs += 1,
        //         TokenType::Ident => {},
        //         TokenType::CloseAttr => {
        //             found_attrs += 1;
        //             break;
        //         },
        //         _ => {}
        //     }
        //     self.next();
        // }

        // check for scripts
        let mut found_open = false;
        loop {
            tok = self.curr();
            match tok.t {
                TokenType::OpenScope => {
                    self.next();
                    found_open = true;
                },
                TokenType::OpenAttr if found_open => {
                    let script = self.parse_script_attrs();
                    scripts.push(script.unwrap());
                },
                TokenType::Script if found_open => {
                    let script = self.parse_script();
                    scripts.push(script.unwrap());
                },
                TokenType::CloseScope if found_open => {
                    self.next();
                    break;
                },
                TokenType::NewLine => {
                    self.next();
                },
                _ => {
                    let msg = format!("invalid item at: {:?}", tok.t);   
                    return Err(msg)
                }
            }
        }

        let action = Action::Command{
            ident: ident,
            attrs: attrs,
            scripts: scripts
        };
        Ok(action)
    }
}
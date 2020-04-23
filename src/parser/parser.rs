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
            match tok.t {
                TokenType::Command => {
                    let cmd = self.parse_cmd();
                    match &cmd {
                        Ok(_) => {},
                        Err(msg) => return Err(msg.to_string())
                    };

                    self.actions.push(cmd.unwrap());
                },
                TokenType::Script => {
                    let script = self.parse_script().unwrap();
                    self.actions.push(script);
                },
                TokenType::EOF => break,
                _ => self.next()
            }
        }
        if self.logger.verbose {
            self.show_actions();
        }
        Ok("success parsing")
    }

    // ------ PARSING ------
    fn parse_script (&mut self) -> Result<Action, String> {
        let token = self.curr();
        let mut action = Action::Script {
            cmd: token.v.unwrap()
        };
        self.next();
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
                TokenType::Script if found_open => {
                    let script = self.parse_script();
                    scripts.push(script.unwrap());
                },
                TokenType::CloseScope if found_open => {
                    self.next();
                    break;
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
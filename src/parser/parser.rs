use super::actions::{Action, Attr};
use crate::lexer::tokens::{Token, TokenType};
use crate::utils::logger::{Level, Logger};
use std::collections::{HashMap, HashSet};

// use crate::utils::cmd::{Cmd, Setting};
    

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

    /*
    --------- --------- UTILS --------- ---------
    */
    fn next(&mut self) {
        self.pos += 1;
    }

    fn has_next(&mut self, pat: Vec<TokenType>) -> bool {
        for i in 0..pat.len() {
            if self.pos + i >= self.tokens.len() {
                return false;
            }
            let temp_tok = self.get_tok(self.pos + i);
            match temp_tok {
                Ok(tok) if tok.t == pat[i] => {
                    // println!("[{}] {} == {} = {}",i+self.pos, tok.t, pat[i], (tok.t == pat[i]));
                },
                Ok(tok) => {
                    // println!("[{}] {} == {} = {}",i+self.pos, tok.t, pat[i], (tok.t == pat[i]));
                    return false
                },
                Err(msg) => return false
            }
        }
        true
    }

    fn curr(&mut self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn get_tok(&mut self, pos: usize) -> Result<Token, String> {
        if pos < self.tokens.len() {
            Ok(self.tokens[pos].clone())
        } else {
            Err("pos requested is longer than token list".to_owned())
        }
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
            let res: Result<Action, String> = match tok.t {
                TokenType::Ident => self.parse_kw_generic(),
                TokenType::EOF => break,
                _ => {
                    self.next();
                    continue
                }
            };

            match &res {
                Ok(_) => self.actions.push(res.unwrap()),
                Err(msg) => return Err(msg.to_string())
            };
        };
        if self.logger.verbose {
            self.show_actions();
        }
        Ok("success parsing")
    }

    /*
    --------- --------- PARSING --------- ---------
    */
    fn parse_keyword(&mut self) -> Result<Action, String> {
        let tok = self.curr();
        match tok.v {
            Some(s) => {
                if !s.contains("!") {
                    return self.parse_kw_generic()
                }
                else {
                    let err_msg = format!("keyword '{}' is not a valid runfile action", s);
                    Err(err_msg) 
                }
            },
            _ => {
                self.next();
                Err("expected keyword".to_owned())
            }
        }
    }

    fn parse_attrs(&mut self) -> Result<HashMap<String, Attr>, String> {
        let mut attrs: HashMap<String, Attr> = HashMap::new();

        // --- parse intro
        loop {
            let token = self.curr();
            match token.t {
                TokenType::OpenParen => {
                    self.next();
                    break
                },
                TokenType::Space | TokenType::NewLine => self.next(),
                _ => {
                    let err_msg = format!("1. invalid attributes definition @ {}:{} ", token.ln, token.col);
                    return Err(err_msg);
                }
            }
        }
        // --- parse body
        loop {
            let token = self.curr();
            match token.t {
                TokenType::CloseParen => {
                    self.next();
                    break
                },
                TokenType::Space | TokenType::NewLine => self.next(),
                TokenType::Ident if self.has_next(vec![TokenType::Ident, TokenType::Space]) 
                                 || self.has_next(vec![TokenType::Ident, TokenType::CloseParen]) => {
                    attrs.insert(token.v.unwrap(), Attr::value{v:"1".to_owned()});
                    self.next();
                },
                TokenType::Ident 
                    if self.has_next(vec![TokenType::Ident, 
                                            TokenType::Colon,
                                            TokenType::Ident
                                            ]) 
                        || self.has_next(vec![TokenType::Ident, 
                                            TokenType::Colon,
                                            TokenType::StringValue
                                            ]) => {
                                            let a_key = token.v.unwrap();
                                            self.next();
                                            self.next();
                                            let token = self.curr();
                                            attrs.insert(a_key, Attr::value{v:token.v.unwrap()});
                                            self.next();
                },
                TokenType::Ident 
                    if self.has_next(vec![TokenType::Ident, 
                                          TokenType::Colon,
                                          TokenType::IntValue
                    ]) => {
                        let mut int_as_str = String::new();
                        self.next();
                        self.next();
                        loop {
                            let token = self.curr();
                            match token.t {
                                TokenType::CloseParen => break,
                                TokenType::Space | TokenType::NewLine => {
                                    self.next();
                                    break
                                },
                                TokenType::IntValue  => {
                                    int_as_str.push_str(&token.v.unwrap().to_owned());
                                    self.next();
                                },
                                TokenType::Period => {
                                    int_as_str.push('.');
                                    self.next();
                                },
                                _ => {
                                    let err_msg = format!("2.1. invalid attributes definition @ {}:{} ", token.ln, token.col);
                                    return Err(err_msg);
                                }
                            }
                        }
                        let a_key = token.v.unwrap();
                        attrs.insert(a_key, Attr::value{v:int_as_str});
                },
                _ => {
                    let err_msg = format!("2. invalid attributes definition @ {}:{} ", token.ln, token.col);
                    return Err(err_msg);
                }
            }
        }
        Ok(attrs)
    }

    fn parse_in_value(&mut self) -> Result<String, String> {
        let mut in_value: String = String::new();
        self.next();
        loop {
            let token = self.curr();
            match token.t {
                TokenType::NewLine => { 
                    self.next();
                    break
                },
                TokenType::EOF => break,
                TokenType::Space => self.next(),
                TokenType::Value => {
                    in_value.push_str(&token.v.unwrap());
                    self.next();
                }
                _ => {
                    let err_msg = format!("invalid input value definition @ {}:{} ", token.ln, token.col);
                    return Err(err_msg);
                }
            }
        }
        Ok(in_value)
    }

    fn parse_kw_generic(&mut self) -> Result<Action, String> {
        let token = self.curr();
        let action_name = token.v.unwrap();
        // let mut flags: HashSet<String> = HashSet::new();
        let mut attrs: HashMap<String, Attr> = HashMap::new();
        let mut in_val: String = String::new();
        self.next();
        loop {
            let token = self.curr();
            match token.t {
                TokenType::EOF | TokenType::Ident => break,
                TokenType::Space | TokenType::Comment | TokenType::NewLine => self.next(),
                TokenType::RunSym => {
                    let value = self.parse_in_value();
                    match value {
                        Ok(value) => {
                            in_val = value;
                        },
                        Err(msg) => {
                            return Err(msg) 
                        }
                    }
                },
                //     self.next();
                //     loop {
                //         let token = self.curr();
                //         match token.t {
                //             TokenType::NewLine => {
                //                 self.next();
                //                 break
                //             },
                //             TokenType::Value => {
                //                 run_vals.push(RunVal::Value{v:token.v.unwrap()});
                //                 self.next()
                //             },
                //             TokenType::DollarSign if self.has_next(vec![
                //                 TokenType::DollarSign,
                //                 TokenType::OpenScope,
                //                 TokenType::Value
                //             ]) => {
                //                 self.next();
                //                 self.next();
                //                 let token = self.curr();
                //                 run_vals.push(RunVal::Expr{v:token.v.unwrap()});
                                
                //                 self.next();
                //                 if !self.has_next(vec![TokenType::CloseScope]){
                //                     return Err("reference expression is not closed with '}'!".to_owned())
                //                 } else {
                //                     self.next();
                //                 }
                //             },
                //             _ => {
                //                 let msg = format!("invalid token, '{}' in RUN command!", token.t);
                //                 return Err(msg)
                //             }
                //         }
                //     }
                //     break
                // },
                TokenType::OpenParen => {
                    let attrs_parsed = self.parse_attrs();
                    match attrs_parsed {
                        Ok(a) => {
                            attrs = a;
                        },
                        Err(msg) => {
                            return Err(msg) 
                        }
                    }
                },
                _ => {
                    let err_msg = format!("token '{}' is not a valid runfile action", token.t);
                    return Err(err_msg)
                }
            }
        }
        // let action = Action::new(action_name, Some(flags), attrs, Some(run_vals));
        let action = Action::new(action_name, attrs, Some(in_val));
        Ok(action)
    }
}
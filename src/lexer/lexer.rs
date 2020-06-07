
// use super::actions::{Cmd};
use super::tokens::{TokenType, Token};
// use regex::Regex;
// use std::io::Error;
// use crate::utils::logger;
use crate::utils::logger::{Level, Logger};


pub struct Lexer<'a>  {
    pos: usize,
    line: usize,
    content: String,
    pub token_stack: Vec<Token>,
    logger: &'a Logger, 
}

impl<'a> Lexer<'a>  {

    pub fn new(logger: &'a Logger, content: String) -> Lexer {
        Lexer {
            pos: 0,
            line: 0,
            content: content,
            token_stack: Vec::new(),
            logger: logger, 
        }
    }

    fn at_end(&self) -> bool {
        if self.pos > self.content.len()-1 { 
            true
        } else {
            false
        }
    }

    fn adv(&mut self, chars: usize, lines: usize) {
        self.pos += chars;
        self.line += lines;
        // println!("....|  len:{}  i:{}", self.content.len()-1, self.pos);
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.line = 0;
        self.token_stack = vec![];
    }

    fn current(&self) -> Option<char> {
        // println!("cur is (p:{}) '{}'  len:{}", 
        //         self.pos, 
        //         self.content.chars().nth(self.pos).unwrap(),
        //         self.content.len()
        //     );
        self.content.chars().nth(self.pos)
    }

    fn adv_past_spaces(&mut self) {
        loop {
            match self.current().unwrap() {
                ' ' => self.adv(1, 0),
                _ => break
            };
        };
    }

    fn has_pat(&self, pat: &str) -> bool {
        if self.content[self.pos..].starts_with(pat) {
            true
        } else {
            false
        }
    }

    fn add(&mut self, t: TokenType, v: Option<String>){
        self.token_stack.push(
            Token{t: t, v: v}
        ); 
    }

    pub fn show_tokens(&mut self) {
        for tok in self.token_stack.iter() {
            println!("   {:?}", tok);
        }
        println!();
    }

    pub fn analyze(&mut self) -> Result<&str, String> {
        if self.logger.verbose {
            self.logger.log(
                "Lexing Characters".to_string(), 
                Level::Info
            );
        }
        self.reset();
        loop {
            let c = self.current().unwrap();
            match c {
                '#' => self.parse_comment(),
                '!' => {
                    if self.has_pat("!cmd") {
                        self.adv(4, 0);
                        self.parse_cmd();
                    } else if self.has_pat("!var") {
                        self.adv(4, 0);
                        self.parse_var();
                    } else {
                        self.adv(1, 0);
                    }
                },
                '>'  => {
                    self.parse_script();
                },
                '"' | '\'' => {
                    self.adv(1, 0);
                    self.parse_string(c);
                },
                '[' => {
                    self.add(TokenType::OpenAttr, None);
                    self.adv(1, 0);
                },
                ']' => {
                    self.add(TokenType::CloseAttr, None);
                    self.adv(1, 0);
                },
                '{' => {
                    self.add(TokenType::OpenScope, None);
                    self.adv(1, 0);
                },
                '}' => {
                    self.add(TokenType::CloseScope, None);
                    self.adv(1, 0);
                },
                ':' => {
                    self.add(TokenType::Colon, None);
                    self.adv(1, 0);
                },
                ' ' => self.adv(1, 0),
                ',' => {
                    self.add(TokenType::Comma, None);
                    self.adv(1, 0);
                },
                _ if c.is_alphabetic() => self.parse_ident(),
                '\n' | '\r' => {
                    self.add(TokenType::NewLine, None);
                    self.adv(1, 1);
                },
                _ => {

                    let e_msg = format!("ERROR: invalid character '{}' @ l:{}, p:{}", c, self.line+1, self.pos+1);
                    return Err(e_msg);
                }
            };
            if self.at_end() {
                self.add(TokenType::EOF, None);
                break; 
            }
        };

        if self.logger.verbose {
            self.show_tokens();
        }
        Ok("SUCCESS: string has proper format")
    }

    fn parse_value(&mut self, terminator: Option<char>) {
        let mut val = String::new();
        loop {
            let c = self.current().unwrap();
            match terminator {
                Some(ch) if ch == c => {
                    self.adv(1,0);
                    break;
                },
                Some(_) | None => {}
            }
            match c {
                '\n' | '\r' => {
                    self.adv(1, 1);
                    let b = self.current().unwrap();
                    self.add(TokenType::Value, Some(val)); 
                    self.add(TokenType::NewLine, None); 
                    break;
                },
                '$' if self.has_pat("${") => {
                    self.add(TokenType::Value, Some(val)); 
                    self.parse_var_ref();
                    self.parse_value(None);
                    break
                },
                _ => {
                    val.push(c);
                    self.adv(1, 0);
                }
            };
            if self.at_end() {
                break
            }
        };
    }

    fn parse_mline_value(&mut self){
        self.add(TokenType::OpenMLineScope, None);
        self.adv(2, 0);
        loop {
            let c = self.current().unwrap();
            match c {
                '}' if self.has_pat("}}") => {
                    self.token_stack.pop();
                    self.add(TokenType::CloseMLineScope, None);
                    self.adv(2, 0);
                    break
                },
                '\n' | '\r' => self.adv(1, 1),
                _ if self.at_end() => {
                    self.adv(1, 1);
                    break
                },
                _ => self.parse_value(None)
            }
            if self.at_end() {
                break
            }
        }
    }

    fn parse_string(&mut self, quote_char: char) {
        let mut val = String::new();
        loop {
            let c = self.current().unwrap();
            match c {
                _ if c == quote_char => {
                    self.adv(1, 0);
                    break;
                },
                '\n' | '\r' => {
                    val.push(c);
                    self.adv(1, 1);
                },
                _ => {
                    val.push(c);
                    self.adv(1, 0);
                }
            };
            if self.at_end() {
                break
            }
        };
        self.add(TokenType::StringValue, Some(val)); 
    }

    fn parse_var_ref(&mut self){
        if !self.has_pat("${") {
            return
        }
        self.adv(2, 0);
        self.add(TokenType::VarRef, None);
        self.add(TokenType::OpenScope, None);
        loop {
            let c = self.current().unwrap();
            match c {
                c if c.is_alphanumeric() => {
                    self.parse_ident();  
                },
                ' ' => self.adv_past_spaces(),
                '}' => {
                    self.add(TokenType::CloseScope, None);
                    self.adv(1, 0);
                    break;
                },
                _ => break
            };
            if self.at_end() {
                break
            }
        };
    }

    fn parse_script(&mut self){
        self.adv_past_spaces();
        if !self.has_pat(">> ") {
            self.adv(1, 0);
            return
        }
        self.adv(3, 0);
        self.add(TokenType::Script, None);
        loop {
            let c = self.current().unwrap();
            match c {
                '\\' if self.has_pat("\\\r\n") || self.has_pat("\\\n") => {
                    self.adv(2, 1);
                    self.adv_past_spaces();
                },
                '$' => {
                    self.parse_var_ref();
                },
                '\n' | '\r' => {
                    self.adv(1, 1);
                    break;
                },
                _ => {
                    self.parse_value( None );
                    break;
                }
            };
            if self.at_end() {
                break
            }
        };
    }

    fn parse_ident(&mut self) {
        let mut val = String::new();
        self.adv_past_spaces();
        loop {
            let c = self.current().unwrap();
            match c {
                '_' | '-' => {
                    self.adv(1, 0);
                    val.push(c);
                },
                c if c.is_alphanumeric() => {
                    self.adv(1, 0);
                    val.push(c);
                },
                _ => {
                    break;
                }
            };
            if self.at_end() {
                break
            }
        };
        self.add(TokenType::Ident, Some(val));
    }

    fn parse_var(&mut self) {
        self.adv_past_spaces();
        self.add(TokenType::Variable, None);
        self.parse_ident();
        loop {
            let c = self.current().unwrap();
            match c {
                '=' => {
                    self.add(TokenType::EqualSign, None);
                    self.adv(1, 0);
                },
                '{' if self.has_pat("{{") => {
                    self.parse_mline_value();
                    break;
                },
                ' ' => self.adv(1, 0),
                c if c.is_ascii() => {
                    self.parse_value(None);
                    break;
                },
                _ => {}
            };
            if self.at_end() {
                break
            }
        }; 
    }

    fn parse_comment(&mut self) {
        loop {
            let c = self.current().unwrap();
            match c {
                '\n' | '\r' => {
                    self.adv(1, 1); 
                    break
                },
                _ => self.adv(1, 0)
            };
            if self.at_end() {
                break
            }
        }
    }

    fn parse_cmd(&mut self){
        self.add(TokenType::Command, None);
        self.parse_ident();        
    }
}
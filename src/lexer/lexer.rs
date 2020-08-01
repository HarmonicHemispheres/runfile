
// use super::actions::{Cmd};
use super::tokens::{TokenType, Token};
// use regex::Regex;
// use std::io::Error;
// use crate::utils::logger;
use crate::utils::logger::{Level, Logger};


pub struct Lexer<'a>  {
    pos: usize,
    line: usize,
    col: usize,
    content: String,
    pub token_stack: Vec<Token>,
    logger: &'a Logger, 
}

impl<'a> Lexer<'a>  {

    pub fn new(logger: &'a Logger, content: String) -> Lexer {
        Lexer {
            pos: 0,
            line: 1,
            col: 1,
            content: content,
            token_stack: Vec::new(),
            logger: logger, 
        }
    }

    fn current(&self) -> Option<char> {
        self.content.chars().nth(self.pos)
    }

    // 'advance' N chars in the file string 'position'
    fn adv(&mut self, chars: usize) -> bool {
        for _ in 0..chars {
            // get the current character
            let c = self.current().unwrap(); 
            self.pos += 1;

            // check if line and/or need to advance
            match c {
                '\n' | '\r' => { self.line += 1; self.col = 1; },
                _ => { self.col += 1; }
            }

            // if at end of file string, return
            if self.at_end() {
                return true
            }
        }
        false
    }

    fn rev(&mut self, chars: usize){
        for _ in 0..chars {
            let c = self.current().unwrap(); 
            self.pos -= 1;
            match c {
                '\n' | '\r' => { 
                    //
                    // find line length of previous line 
                    //
                    self.line -= 1; 
                    let mut line_length = 0;
                    loop {
                        let c = self.current().unwrap(); 
                        match c {
                            '\n' => break,
                            _ if self.pos == 0 => {
                                line_length += 1;
                                break
                            },
                            _ => { line_length += 1; }
                        }
                        self.pos -= 1;
                    }
                    self.pos += line_length;
                    self.col = line_length; 
                },
                _ if self.pos == 0 => break,
                _ => { self.col -= 1; }
            }
        }
    }

    fn at_end(&self) -> bool {
        if self.pos > self.content.len()-1 { 
            true
        } else {
            false
        }
    }

    fn has_pat(&self, pat: &str) -> bool {
        if self.content[self.pos..].starts_with(pat) {
            true
        } else {
            false
        }
    }

    fn add(&mut self, 
           t: TokenType, 
           v: Option<String>,
           line: usize, 
           col: usize){
        self.token_stack.push(
            Token{t: t, 
                  v: v, 
                  col: col,
                  ln: line
                 }
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

        loop {
            let c = self.current().unwrap();

            match c {
                '#' => self.parse_comment(),
                '>' if self.has_pat(">>")  => {
                    self.add(TokenType::RunSym, None, self.line, self.col);
                    self.adv(2);
                    self.parse_value(vec!['\n', '$']);
                },
                '=' => {
                    self.add(TokenType::EqualSign, None, self.line, self.col);
                    self.adv(1);
                },
                '[' => {
                    if self.has_pat("[[") {
                        self.adv(2);
                        self.parse_raw_value("]]");
                    } else {
                        self.add(TokenType::OpenAttr, None, self.line, self.col);
                        self.adv(1);
                    }
                },
                ']' => {
                    self.add(TokenType::CloseAttr, None, self.line, self.col);
                    self.adv(1);
                },
                '(' => {
                    self.add(TokenType::OpenParen, None, self.line, self.col);
                    self.adv(1);
                },
                ')' => {
                    self.add(TokenType::CloseParen, None, self.line, self.col);
                    self.adv(1);
                },
                '$' => {
                    self.add(TokenType::DollarSign, None, self.line, self.col);
                    self.adv(1);
                },
                '{' => {
                    self.add(TokenType::OpenScope, None, self.line, self.col);
                    self.adv(1);
                },
                '}' => {
                    self.add(TokenType::CloseScope, None, self.line, self.col);
                    self.adv(1);
                },
                ':' => {
                    self.add(TokenType::Colon, None, self.line, self.col);
                    self.adv(1);
                },
                ';' => {
                    self.add(TokenType::SemiColon, None, self.line, self.col);
                    self.adv(1);
                },
                ' ' => {
                    self.parse_space();
                },
                ',' => {
                    self.add(TokenType::Comma, None, self.line, self.col);
                    self.adv(1);
                },
                '.' => {
                    self.add(TokenType::Period, None, self.line, self.col);
                    self.adv(1);
                },
                '"' | '\'' => self.parse_string(),
                '\n' | '\r' => {
                    self.add(TokenType::NewLine, None, self.line, self.col);
                    self.adv(1);
                },
                _ if c.is_alphabetic() => {
                    if !self.parse_keyword() {
                        if !self.parse_ident() {
                            self.parse_value(vec![' ', '\n']);
                        }
                    }
                },
                _ if c.is_numeric() => {
                    self.parse_int();
                },
                _ if !c.is_ascii() => {
                    let e_msg = format!("ERROR: invalid character '{}' @ line:{}, col:{}", c, self.line, self.col);
                    return Err(e_msg);
                },
                _ => self.parse_value(vec![' ', '\n', '}']),
            };
            if self.at_end() {
                self.add(TokenType::EOF, None, self.line, self.col);
                break; 
            }
        };

        if self.logger.verbose {
            self.show_tokens();
        }
        Ok("SUCCESS: string has proper format")
    }
    
    fn parse_keyword(&mut self) -> bool {

        // parse for normal action formats
        
        // if self.has_pat("run ") || self.has_pat("RUN "){
        //     self.add(TokenType::Keyword, Some("run".to_owned()), self.line, self.col);
        //     self.adv(3);
        // }
        // else if self.has_pat("var ") || self.has_pat("VAR ") {
        //     self.add(TokenType::Keyword, Some("var".to_owned()), self.line, self.col);
        //     self.adv(3);
        // }
        // else if self.has_pat("cmd ") || self.has_pat("CMD ") {
        //     self.add(TokenType::Keyword, Some("cmd".to_owned()), self.line, self.col);
        //     self.adv(3);
        // }
        
        // parse for special format edition
        // else if self.has_pat("run! ") || self.has_pat("RUN! "){
        //     self.add(TokenType::Keyword, Some("run!".to_owned()), self.line, self.col);
        //     self.adv(4);
        // }
        // else if self.has_pat("var! ") || self.has_pat("VAR! ") {
        //     self.add(TokenType::Keyword, Some("var!".to_owned()), self.line, self.col);
        //     self.adv(3);
        // } 
        // else {
        //    return false
        // }
        false
    }

    fn parse_string(&mut self) {
        let mut val = String::new();
        let marker: char = self.current().unwrap();
        let curr_line = self.line;
        let curr_col = self.col;
        self.adv(1);
        loop {
            let c = self.current().unwrap();
            match c {
                _ if c == marker => {
                    self.adv(1);
                    break
                },
                _ => {
                    val.push(c);
                    self.adv(1);
                }
            }
            if self.at_end() { break; }
        }
        self.add(TokenType::StringValue, Some(val), curr_line, curr_col);
    }

    fn parse_int(&mut self) {
        let mut val = String::new();
        let curr_line = self.line;
        let curr_col = self.col;
        // self.adv(1);
        loop {
            let c = self.current().unwrap();
            match c {
                _ if c.is_numeric() => {
                    self.adv(1);
                    val.push(c);
                },
                _ => break
            }
            if self.at_end() { break; }
        }
        self.add(TokenType::IntValue, Some(val), curr_line, curr_col); 
    }

    fn parse_space(&mut self) {
        let curr_line = self.line;
        let curr_col = self.col;
        let mut found_space = false;
        loop {
            let c = self.current().unwrap();
            match c {
                ' ' => {
                    self.adv(1);
                    found_space = true;
                },
                _ => break
            }
            if self.at_end() { break; }
        }
        if found_space {
            self.add(TokenType::Space, None, curr_line, curr_col);
        }
    }

    fn parse_ident(&mut self) -> bool {
        let mut is_ident = true;
        let mut val = String::new();
        let mut adv_counter = 0;
        let curr_line = self.line;
        let curr_col = self.col;
        loop {
            let c = self.current().unwrap();
            match c {
                '_' | '-' | '.' => {
                    self.adv(1);
                    adv_counter += 1;
                    val.push(c);
                },
                _ if c.is_alphanumeric() => {
                    self.adv(1);
                    adv_counter += 1;
                    val.push(c);
                },
                ' ' | ':' | ';' | '{' | '(' | '[' | '\n' | ',' | '}' | ')' | ']' => break,
                _ => {
                    is_ident = false;
                    break
                }
            }
            if self.at_end() { break; }
        }
        if is_ident {
            self.add(TokenType::Ident, Some(val), curr_line, curr_col);
            return true
        } else {
            self.rev(adv_counter);
            false
        }
           
    }

    fn parse_value(&mut self, delims: Vec<char>) {
        let mut val = String::new();
        self.parse_space();
        let curr_line = self.line;
        let curr_col = self.col;
        loop {
            let c = self.current().unwrap();
            match c {
                _ if delims.contains(&c) => break,
                _ => {
                    self.adv(1);
                    val.push(c);
                }
            }
            if self.at_end() { break; }
        }
        self.add(TokenType::Value, Some(val), curr_line, curr_col);
    }

    fn parse_raw_value(&mut self, delim: &str) {
        let mut val = String::new();
        let curr_line = self.line;
        let curr_col = self.col;
        loop {
            let c = self.current().unwrap();
            match c {
                _ if self.has_pat(delim) => {
                    self.adv(delim.len());
                    break
                },
                _ => {
                    self.adv(1);
                    val.push(c);
                }
            }
            if self.at_end() { break; }
        }
        self.add(TokenType::Value, Some(val), curr_line, curr_col);
    }
    
    fn parse_comment(&mut self) {
        let mut val = String::new();
        let curr_line = self.line;
        let curr_col = self.col;
        loop {
            let c = self.current().unwrap();
            match c {
                '\n' | '\r' => {
                    self.adv(1); 
                    break
                },
                _ => {
                    val.push(c);
                    self.adv(1);
                }
            };
            if self.at_end() { break }
        }
        self.add(TokenType::Comment, Some(val), curr_line, curr_col);
    }
}
mod parser;
mod lexer;
mod runner;
mod utils;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use utils::logger::Logger;
use std::fs;
use std::env;


fn main() {

    //->  Read CLI Args <-//
    let args: Vec<String> = env::args().collect();

    //->  Read Script File's Content <-//
    let file: String = "runfile".to_string();
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    
    
    //->  Setup logger <-//
    let logger = Logger::new(None, false, true, true);
    
    //->  Lex characters into tokens <-//
    let mut lexer = Lexer::new(&logger, contents);
    let _l_analyze = lexer.analyze();
    
    // //->  parse tokens into actions <-//
    let mut parser = Parser::new(&logger, &lexer.token_stack);
    let _p_analyze = parser.analyze();
    match _p_analyze {
        Ok(_) => {},
        Err(msg) => {
            println!("[E]  {}", msg);
            return;
        }
    }
    
    // //-> execute program actions <-//
    runner::run(parser.actions, args);
}

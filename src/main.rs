mod parser;
mod lexer;
mod utils;
mod program;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use utils::logger::Logger;
use program::program::Program;
use program::config::Cli;
use std::fs;
use std::env;
use clap::Clap;


fn main() {

    //->  Read CLI Args <-//
    let cli: Cli = Cli::parse();
    println!("CLI -->  {:?}", cli);

    //->  Read Script File's Content <-//
    let contents = fs::read_to_string(&cli.runfile)
        .expect("Something went wrong reading the file");
    
    //->  Setup logger <-//
    let logger = Logger::new(None, false, true, cli.debug);
    
    //->  Lex characters into tokens <-//
    let mut lexer = Lexer::new(&logger, contents);
    let _l_analyze = lexer.analyze();
    
    //->  parse tokens into actions <-//
    let mut parser = Parser::new(&logger, &lexer.token_stack);
    let _p_analyze = parser.analyze();
    match _p_analyze {
        Ok(_) => {},
        Err(msg) => {
            println!("[E]  {}", msg);
            return;
        }
    }
    
    //-> execute program actions <-//
    let mut prg = Program::new(parser.actions);
    prg.run(cli);
}

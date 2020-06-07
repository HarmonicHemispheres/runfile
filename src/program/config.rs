
use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(version = "0.1.2", author = "Robby B. <robby.boney@harmonichemispheres.com>")]
pub struct Cli {
    // choose a specific runfile command to run
    #[clap(short="c", long, default_value="__main__")]
    pub cmd: String,

    /// Specify a specific runfile to use    
    #[clap(short="r", long, default_value="runfile")]
    pub runfile: String,
    
    /// Print extra content to the console
    #[clap(short="d", long, takes_value=false)]
    pub debug: bool


    // args for runfile engine
    // #[clap(multiple=true)]
    // pub args: Vec<String>
}

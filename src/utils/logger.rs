
pub enum Level {
    Info,
    Warning,
    Error
}



#[derive(Debug)]
pub struct Logger {
    path: Option<String>,
    log_to_path: bool,
    log_to_console: bool,
    pub verbose: bool
}



impl Logger {
    pub fn new(
        path: Option<String>,
        log_to_path: bool, 
        log_to_console: bool,
        verbose: bool
    ) -> Logger {
        Logger {
            path: path,
            log_to_path: log_to_path, 
            log_to_console: log_to_console,
            verbose: verbose
        }
    }

    fn write_to_path(&self, content: String) {}

    pub fn log(&self, content: String, lvl: Level) {

        let lvl_s = match lvl {
            Level::Info => ".".to_string(),
            Level::Warning => "w".to_string(),
            Level::Error => "X".to_string()
        };

        let s = format!("[{}]  {}", lvl_s, content);

        if self.log_to_path {}

        if self.log_to_console {
            println!("{}", s);
        }
    }
}

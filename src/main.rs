mod commands;
mod config;
mod handler;

use std::{env::current_dir, process::exit};
use rustyline::Editor;

fn main() {
    let mut editor = Editor::<()>::new();

    let mut state = handler::Shell {
        cd: current_dir().unwrap().display().to_string()
    };

    loop {
        let conf = config::load_config(&state);
        let input = editor.readline(conf.prompt.as_str());

        match input {
            Ok(command) =>
                handler::handle(command, &mut state),
            Err(_) => exit(0)
        }
    }
}

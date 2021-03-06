mod commands;
mod config;
mod handler;
mod helper;

use std::env::{self, current_dir};
use rustyline::Editor;

fn main() {
    let home = env::var("HOME").unwrap();
    let mut editor = Editor::<helper::WooshHelper>::new();
    editor.set_helper(helper::get_helper());
    let res = editor.load_history(&format!("{}/.woosh-history", home));

    if let Err(_) = res {
        println!("WARNING: Failed to load history file!");
    }

    let mut state = handler::Shell {
        cd: current_dir().unwrap().display().to_string()
    };

    let conf = config::load_config(&state);
    println!("{}", conf.welcome);

    loop {
        let conf = config::load_config(&state);
        let input = editor.readline(conf.prompt.as_str());

        match input {
            Ok(command) => {
                editor.add_history_entry(command.as_str());
                handler::handle(command, &mut state);
            },
            Err(rustyline::error::ReadlineError::Interrupted) => {},
            Err(_) => {
                break;
            }
        }

        env::set_var("PWD", state.cd.clone());
    }

    editor.save_history(&format!("{}/.woosh-history", home)).unwrap();
}

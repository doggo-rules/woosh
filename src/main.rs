mod handler;

use std::{env::{self, current_dir}, process::exit};
use rustyline::Editor;

fn main() {
    let mut editor = Editor::<()>::new();

    let mut state = handler::Shell {
        cd: current_dir().unwrap().display().to_string()
    };

    loop {
        let input = editor.readline(
                        format!(
                            "{} $ ",
                            state.cd.replace(env::var("HOME").unwrap().as_str(), "~")
                        ).as_str()
                    );

        match input {
            Ok(command) =>
                handler::handle(command, &mut state),
            Err(_) => exit(0)
        }
    }
}

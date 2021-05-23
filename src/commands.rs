use std::process::{Command, Stdio};
use std::{env, fs::canonicalize, path::PathBuf};
use crate::handler::Shell;

pub fn eval_cmd(cmd_name: String, cmd_args: &[String], state: &mut Shell) {
        let output = Command::new(cmd_name)
            .args(cmd_args)
            .current_dir(state.cd.clone())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output();

        if let Err(_) = output {
            println!("Err: File not found!");
        }
}

pub fn cmd_cd(args: &Vec<String>, state: &mut Shell) {
    let path = args.get(1).unwrap();
    let new_path = PathBuf::from(&state.cd)
        .join(
            path.replace("~", env::var("HOME").unwrap().as_str())
        )
        .display()
        .to_string();

    state.cd = canonicalize(new_path).unwrap().display().to_string();
    return;
}

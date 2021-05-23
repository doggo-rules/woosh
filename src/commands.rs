use std::process::{Command, Stdio};
use std::{fs::canonicalize, path::PathBuf};
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
    let p = args.get(1);

    if let Some(path) = p {
        let new_path = PathBuf::from(&state.cd)
            .join(path);

        if !new_path.is_dir() {
            println!("Err: Path is not a directory!");
            return;
        }

        let cd = canonicalize(new_path.display().to_string());
        if let Ok(pathb) = cd {
            state.cd = pathb.display().to_string();
        } else {
            println!("Err: Directory not found!");
        }
    } else {
        println!("Err: No path passed to cd!");
    }
}

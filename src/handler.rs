use std::{env, ffi::OsStr, fs::canonicalize, path::PathBuf, process::{Command, Stdio}};

pub struct Shell {
    pub cd: String
}

pub fn handle(command: String, state: &mut Shell) {
    let args = parse_command(command);

    if let Err(e) = args {
        println!("Err: {}", e);
    } else {
        let args = args.unwrap();

        if args[0] == String::from("cd") {
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

        let cmd_name = &args.first().unwrap();
        let cmd_args = &args[1..];
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
}

fn parse_command(command: String) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut current = String::new();

    for c in command.chars() {
        if *stack.last().unwrap_or(&'.') == '"' {
            if c == '"' {
                stack.pop();
                args.push(current.clone());
                current = String::new();
            } else {
                current.push(c);
            }
            continue;
        }

        match c {
            '"' => {
                if !current.is_empty() {
                    return Err(
                        format!("Unexpected '\"'")
                    );
                }
                stack.push(c);
            },
            ' ' => {
                args.push(current.clone());
                current = String::new();
            },
            _ => current.push(c)
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    if !stack.is_empty() {
        return Err(
            format!("Unclosed '{}'", stack.last().unwrap())
        );
    }

    Ok(args)
}

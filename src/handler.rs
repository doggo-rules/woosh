use crate::commands;
use std::env;

pub struct Shell {
    pub cd: String
}

pub fn handle(command: String, state: &mut Shell) {
    let args = parse_command(command);

    if let Err(e) = args {
        println!("Err: {}", e);
    } else {
        let args = args.unwrap();

        if args.len() < 1 {
            return;
        }

        match args[0].as_str() {
            "cd" => commands::cmd_cd(&args, state),
            _ => {
                let cmd_name = args.first().unwrap();
                let cmd_args = &args[1..];
                commands::eval_cmd(cmd_name.clone(), cmd_args, state)
            }
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
            '~' => {
                if current == "" {
                    current = env::var("HOME").unwrap();
                }
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

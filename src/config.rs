use std::{collections::HashMap, env, fs::read_to_string};
use yaml_rust::YamlLoader;
use crate::handler::Shell;

pub struct Config {
    pub prompt: String
}

pub fn load_config(state: &Shell) -> Config {
    // Prompt formats
    let mut prompt_vars = HashMap::new();
    let home = env::var("HOME").unwrap();
    let cd = &state.cd.replace(home.as_str(), "~");
    prompt_vars.insert("cd".to_string(), cd);

    // Default config
    let default =  Config {
        prompt: format_variables("%{cd} $ ".to_string(), &prompt_vars)
    };

    // Load the config file
    let file = read_to_string(format!("{}/.woosh.yml", home));

    if let Err(_) = file {
        return default;
    } else {
        let docs = YamlLoader::load_from_str(file.unwrap().as_str()).expect("Err: Couldn't parse config file!");
        let doc = &docs[0];

        return Config {
            prompt: if !doc["prompt"].is_badvalue() {
                format_variables(doc["prompt"].as_str().unwrap().to_string(), &prompt_vars)
            } else {
                default.prompt
            }
        };
    }
}

pub fn format_variables(text: String, vars: &HashMap<String, &String>) -> String {
    let mut formatted = text;
    for (name, val) in vars {
        formatted = formatted.replace(format!("%{{{}}}", name).as_str(), val.as_str());
    }
    formatted
}

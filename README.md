# Woo Shell

### About Woo Shell
Woo Shell (woosh) is a shell written in Rust. It is made to be simple yet easy to configure.

### Configuring The Shell
The configuration file is at `~/.woosh.yml`. It's a yaml file because I find yaml files easy to work with, and they seem much friendlier than other formats like xml and json. Settings in this file have formats, which look like this: `%{name}`. You can replace name with any of the formats, and it gets replaced when the config is loaded. Here's a list of settings, valid formats for them, and examples:


#### Prompt
**Description:**
The prompt for the shell.
\
**Formats:**
- cd = Current directory

**Examples:**
```yml
# This is the default prompt
prompt: "%{cd} $ "
```

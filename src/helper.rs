use std::borrow::Cow;
use std::env;
use std::fs::read_dir;
use rustyline::Context;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::Validator;
use rustyline_derive::Helper;

#[derive(Helper)]
pub struct WooshHelper {
    pub hinter: HistoryHinter
}

impl Completer for WooshHelper {
    type Candidate = Pair;

    fn complete(&self, _: &str, pos: usize, _: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let cd = env::var("PWD").unwrap();
        let files = read_dir(cd).unwrap();

        let mut candidates: Vec<Pair> = Vec::new();
        for file in files {
            let entry = file.unwrap();
            let entry_name = entry.file_name().into_string().unwrap();
            
            candidates.push(Pair {
                display: entry_name.clone(),
                replacement: entry_name.clone()
            });
        }
        Ok((pos, candidates))
    }
}

impl Hinter for WooshHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for WooshHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned("\x1b[0;2;30m".to_owned() + hint + "\x1b[0m")
    }
}

impl Validator for WooshHelper {}

pub fn get_helper() -> Option<WooshHelper> {
    Some(WooshHelper {
        hinter: HistoryHinter {}
    })
}

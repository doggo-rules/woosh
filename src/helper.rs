// TODO: Fix this huge import
use std::borrow::Cow;
use rustyline::{Context, completion::{Completer, FilenameCompleter, Pair}, error::ReadlineError, highlight::Highlighter, hint::{Hinter, HistoryHinter}, validate::Validator};
use rustyline_derive::Helper;

#[derive(Helper)]
pub struct WooshHelper {
    pub completer: FilenameCompleter,
    pub hinter: HistoryHinter
}

impl Completer for WooshHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
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
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter {}
    })
}

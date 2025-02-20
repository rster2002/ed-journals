use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref CODEX_REGEX: Regex =
        Regex::new(r#"^\$?[cC]odex_[eE]nt_(.+?)_[nN]ame;?$"#).unwrap();
}

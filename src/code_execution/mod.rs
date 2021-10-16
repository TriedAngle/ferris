// this module contains code from
// https://github.com/kangalioo/rustbot/tree/b618057cb60a40829187b1baf20d0a6ab57e6c0e/src/code_execution/playground

pub mod api;
pub mod eval;
pub mod util;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeBlock {
    pub content: String,
    pub language: Language,
    pub full: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match *self {
            Language::Rust => String::from("Rust"),
        }
    }
}

impl std::fmt::Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "```{}\n{}\n```",
            self.language.to_string(),
            &self.content
        )
    }
}

use comments_docsource::CommentDocumentation;
use core::fmt;
use options_docsource::OptionDocumentation;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub mod comments_docsource;
pub mod options_docsource;

pub struct CustomError(pub String);
impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CustomError {}

pub enum DocEntry {
    OptionDoc(OptionDocumentation),
    CommentDoc(CommentDocumentation),
}

impl DocEntry {
    pub fn name(&self) -> String {
        match self {
            DocEntry::OptionDoc(x) => x.name(),
            DocEntry::CommentDoc(x) => x.name(),
        }
    }
    pub fn pretty_printed(&self) -> String {
        match self {
            DocEntry::OptionDoc(x) => x.pretty_printed(),
            DocEntry::CommentDoc(x) => x.pretty_printed(),
        }
    }
}

pub trait DocSource {
    fn all_keys(&self) -> Vec<&str>;
    fn search(&self, query: &str) -> Vec<DocEntry>;
}

#[derive(Default)]
pub struct AggregateDocSource {
    sources: Vec<Box<dyn DocSource + Sync>>,
}

impl AggregateDocSource {
    pub fn add_source(&mut self, source: Box<dyn DocSource + Sync>) {
        self.sources.push(source)
    }
}

impl DocSource for AggregateDocSource {
    fn all_keys(&self) -> Vec<&str> {
        self.sources
            .par_iter()
            .flat_map(|source| source.all_keys())
            .collect()
    }
    fn search(&self, query: &str) -> Vec<DocEntry> {
        self.sources
            .par_iter()
            .flat_map(|source| source.search(query))
            .collect()
    }
}
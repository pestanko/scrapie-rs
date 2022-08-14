use std::collections::HashSet;

pub trait EntityNamespace {
    fn namespace(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub category: String,
    pub names: HashSet<String>,
    pub tags: HashSet<String>,
}
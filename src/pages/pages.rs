use std::collections::HashSet;
use serde::{Serialize, Deserialize};

use crate::pages::namespace::EntityNamespace;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Page {
    #[serde(default)]
    pub category: String,
    pub codename: String,
    #[serde(default)]
    pub name: String,
    pub homepage: String,
    pub url: String,
    pub query: Option<String>,
    pub xpath: Option<String>,
    #[serde(default)]
    pub cache_policy: String,
    #[serde(default)]
    pub resolver: String,
    #[serde(default)]
    pub disabled: bool,
    pub tags: HashSet<String>,
    pub filters: Option<FiltersConfig>,
    pub command: Option<CommandsConfig>,
}

impl EntityNamespace for Page {
    fn namespace(&self) -> String {
        return format!("{}/{}", self.category, self.codename)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommandsConfig {
    pub content: CommandConfig
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommandConfig {
    pub name: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FiltersConfig {
    pub cut: Option<CutFilter>,
    pub cut_line: Option<CutLineFilter>,
    pub day: Option<DayFilter>,
    pub html: Option<HtmlFilter>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CutFilter {
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DayFilter {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    days: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HtmlFilter {

}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CutLineFilter {
    pub starts_with: String,
    pub contains: String,
    pub cut_after: String,
}


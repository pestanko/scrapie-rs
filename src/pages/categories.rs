use serde::{Serialize, Deserialize};
use tracing::info;

use crate::pages::namespace::EntityNamespace;
use crate::pages::pages;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub name: String,
    pub pages: Vec<pages::Page>,
}

impl EntityNamespace for Category {
    fn namespace(&self) -> String {
        self.name.to_string()
    }
}

pub fn load_category(name: &str) -> Result<Category, Box<dyn std::error::Error>> {
    let full_path = format!("config/categories/{name}.yml");
    let fd = std::fs::File::open(&full_path)?;
    let mut cat: Category = serde_yaml::from_reader(fd)?;
    info!(path = &full_path, "loaded category: {name}");

    cat.pages
        .iter_mut()
        .for_each(|p| p.category = cat.name.clone());

    return Ok(cat)
}


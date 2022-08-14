use std::collections::HashMap;

use crate::{cfg::{self, AppConfig}, pages::{Category, categories::load_category, namespace::Selector, Page}};

pub struct Application {
    config: cfg::AppConfig,
    categories: HashMap<String, Category>
}

impl Application {
    pub fn new(config: cfg::AppConfig) -> Self {
        let categories = config.categories
            .iter()
            .filter_map(|name| {
                match load_category(name) {
                    Ok(category) => Some((name.to_string(), category)),
                    Err(err) => {
                        tracing::error!(category = name, "unable to load category: {}", err);
                        None
                    },
                }
            })
            .collect();
        Self { 
            config: config, 
            categories: categories 
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn category(&self, name: &str) -> Option<&Category> {
        self.categories.get(name)
    }

    pub fn pages<'t>(&'t self, sel: &'t Selector) -> impl Iterator<Item=&'t Page> + '_ {
        let cat: &'t Category = self.category(&sel.category).expect("FIXME: FUTURE");
        cat.pages
            .iter()
            .filter(|page| filter_page(page, sel))
    }
}

fn filter_page(page: &Page, sel: &Selector) -> bool {
    if !sel.names.is_empty() {
        return sel.names.
            iter().
            any(|name| page.name.contains(name));
    }

    if !sel.tags.is_empty() {
        return sel.tags.is_subset(&page.tags);
    }
    
    false
}
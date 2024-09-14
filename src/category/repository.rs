use std::collections::HashMap;
use crate::category::model::{Category, CategoryId};

pub trait CategoryRepository: Clone + Send + Sync + 'static {
    fn get(&self, category_id: &CategoryId) -> Option<&Category>;
    fn get_all(&self) -> Vec<Category>;
}

#[derive(Clone)]
pub struct LocalCategoryRepository {
    category_map: HashMap<CategoryId, Category>
}

impl LocalCategoryRepository {
    pub fn new(category_map: HashMap<CategoryId, Category>) -> Self {
        LocalCategoryRepository {
            category_map
        }
    }
}

impl CategoryRepository for LocalCategoryRepository {
    fn get(&self, category_id: &CategoryId) -> Option<&Category> {
        self.category_map.get(category_id)
    }

    fn get_all(&self) -> Vec<Category> {
        self.category_map.values().cloned().collect()
    }
}

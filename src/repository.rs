use std::collections::HashMap;
use crate::model::{Category, CategoryId, Restaurant, RestaurantId};

#[derive(Clone)]
pub struct CategoryRepository {
    category_map: HashMap<CategoryId, Category>
}

impl CategoryRepository {

    pub fn new(category_map: HashMap<CategoryId, Category>) -> Self {
        CategoryRepository {
            category_map
        }
    }

    pub fn get(&self, category_id: &CategoryId) -> Option<&Category> {
        self.category_map.get(category_id)
    }

    pub fn get_all(&self) -> Vec<Category> {
        self.category_map.values().cloned().collect()
    }
}

#[derive(Clone)]
pub struct RestaurantRepository {
    restaurant_map: HashMap<RestaurantId, Restaurant>,
}

impl RestaurantRepository {
    pub fn new(restaurant_map: HashMap<RestaurantId, Restaurant>) -> Self {
        RestaurantRepository {
            restaurant_map,
        }
    }

    pub fn get(&self, restaurant_id: &RestaurantId) -> Option<&Restaurant> {
        self.restaurant_map.get(restaurant_id)
    }
    pub fn get_all(&self) -> Vec<Restaurant> {
        self.restaurant_map.values().cloned().collect()
    }
}


use std::collections::HashMap;

use crate::restaurant::model::{Restaurant, RestaurantId};

pub trait RestaurantRepository: Clone + Send + Sync + 'static {
    fn get(&self, restaurant_id: &RestaurantId) -> Option<&Restaurant>;
    fn get_all(&self) -> Vec<Restaurant>;
}

#[derive(Clone)]
pub struct LocalRestaurantRepository {
    restaurant_map: HashMap<RestaurantId, Restaurant>,
}

impl LocalRestaurantRepository {
    pub fn new(restaurant_map: HashMap<RestaurantId, Restaurant>) -> Self {
        LocalRestaurantRepository {
            restaurant_map,
        }
    }
}

impl RestaurantRepository for LocalRestaurantRepository {
    fn get(&self, restaurant_id: &RestaurantId) -> Option<&Restaurant> {
        self.restaurant_map.get(restaurant_id)
    }
    fn get_all(&self) -> Vec<Restaurant> {
        self.restaurant_map.values().cloned().collect()
    }
    
}
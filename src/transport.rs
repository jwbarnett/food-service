use serde::{Deserialize, Serialize};
use crate::model::RestaurantId;

#[derive(Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub restaurant_id: RestaurantId,
    pub restaurant_name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoriesWithRestaurants {
    pub category_name: String,
    pub restaurants: Vec<Restaurant>
}

use std::sync::Arc;
use crate::repository::{CategoryRepository, RestaurantRepository};
use crate::transport::{CategoriesWithRestaurants, Restaurant};

#[derive(Clone)]
pub struct CategoryAggregator {
    category_repository: Arc<CategoryRepository>,
    restaurant_repository: Arc<RestaurantRepository>
}

impl CategoryAggregator {
    pub fn new(category_repository: Arc<CategoryRepository>, restaurant_repository: Arc<RestaurantRepository>) -> Self {
        CategoryAggregator {
            category_repository,
            restaurant_repository
        }
    }

    pub fn aggregate(&self) -> Vec<CategoriesWithRestaurants> {
        let result: &mut Vec<CategoriesWithRestaurants> = &mut Vec::new();

        let mut categories = self.category_repository.get_all();
        categories.sort_by(|a, b| a.name.cmp(&b.name));

        let mut restaurants = self.restaurant_repository.get_all();
        restaurants.sort_by(|a, b| a.name.cmp(&b.name));

        for category in categories {
            let restaurants_in_category = &mut Vec::new();
            for restaurant in &restaurants {
                if restaurant.category_id == category.id {
                    restaurants_in_category.push(Restaurant {
                        restaurant_id: restaurant.id,
                        restaurant_name: restaurant.name.clone()
                    });
                }
            }

            result.push(CategoriesWithRestaurants {
                category_name: category.name.clone(),
                restaurants: restaurants_in_category.clone()
            });
        }

        result.clone()
    }
}

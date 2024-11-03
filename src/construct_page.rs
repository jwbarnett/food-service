use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_template::{Key, RenderHtml};
use handlebars::to_json;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::model::RestaurantId;
use crate::transport::CategoriesWithRestaurants;

pub async fn construct_home_page(
    State(AppState {template_engine, ..}): State<AppState>,
    Key(key): Key
) -> impl IntoResponse {
    RenderHtml(key, template_engine, "")
}

#[derive(Clone, Serialize, Deserialize)]
struct ListPage {
    categories: Vec<CategoriesWithRestaurants>
}

pub async fn construct_list_page(
    State(AppState {category_aggregator, template_engine, ..}): State<AppState>,
    Key(key): Key
) -> impl IntoResponse {
    let category_restaurants_mapping = category_aggregator.aggregate();

    let template_variables = ListPage {
        categories: category_restaurants_mapping.clone()
    };

    RenderHtml(key, template_engine, to_json(template_variables))
}

#[derive(Clone, Serialize, Deserialize)]
struct RestaurantPage {
    name: String,
    website: String,
}

pub async fn construct_restaurant_page(
    State(AppState {restaurant_repository, template_engine, ..}): State<AppState>,
    Key(key): Key,
    Path(restaurant_id): Path<RestaurantId>
) -> impl IntoResponse {
    let restaurant = restaurant_repository.get(&restaurant_id);

    let unsafe_restaurant = restaurant.unwrap();

    let template_variables = RestaurantPage {
        name: unsafe_restaurant.name.clone(),
        website: unsafe_restaurant.website_url.to_string()
    };

    RenderHtml(key, template_engine, to_json(template_variables))
}

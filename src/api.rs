use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::AppState;
use crate::model::{Category, CategoryId, Restaurant, RestaurantId};

pub async fn get_categories(
    State(state): State<AppState>
) -> Json<Vec<Category>> {
    let categories = state.category_repository.get_all();
    Json(categories)
}

pub async fn get_restaurants(
    State(state): State<AppState>
) -> Json<Vec<Restaurant>> {
    let restaurants = state.restaurant_repository.get_all();
    Json(restaurants)
}

pub async fn get_category_by_id(
    Path(category_id): Path<CategoryId>,
    State(state): State<AppState>
) -> Result<Json<Category>, StatusCode> {
    match state.category_repository.get(&category_id) {
        Some(category) => Ok(Json(category.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_restaurant_by_id(
    Path(restaurant_id): Path<RestaurantId>,
    State(state): State<AppState>
) -> Result<Json<Restaurant>, StatusCode> {
    match state.restaurant_repository.get(&restaurant_id) {
        Some(restaurant) => Ok(Json(restaurant.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

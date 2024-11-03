use std::collections::HashMap;
use std::sync::Arc;
use axum::{extract::State, routing::get, Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_template::engine::Engine;
use axum_template::{Key, RenderHtml};
use handlebars::{to_json, Handlebars};
use crate::aggregator::CategoryAggregator;
use crate::local_data::generate_data;
use crate::model::{Category, CategoryId, Restaurant, RestaurantId};
use crate::repository::{CategoryRepository, RestaurantRepository};

mod transport;
mod aggregator;
mod repository;
mod model;
mod local_data;

#[derive(Clone)]
struct AppState {
    category_repository: Arc<CategoryRepository>,
    restaurant_repository: Arc<RestaurantRepository>,
    category_aggregator: Arc<CategoryAggregator>,
    template_engine: Engine<Handlebars<'static>>
}

#[tokio::main]
async fn main() {

    let (categories, restaurants) = generate_data();

    let category_repository = Arc::new(CategoryRepository::new(categories));
    let restaurant_repository = Arc::new(RestaurantRepository::new(restaurants));
    let category_aggregator = CategoryAggregator::new(category_repository.clone(),
                                                      restaurant_repository.clone());

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("base", "templates/base.hbs").unwrap();
    handlebars.register_template_file("/", "templates/home.hbs").unwrap();
    handlebars.register_template_file("/list", "templates/list.hbs").unwrap();

    let api_routes = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories/:category_id", get(get_category_by_id))
        .route("/restaurants", get(get_restaurants))
        .route("/restaurants/:restaurant_id", get(get_restaurant_by_id));

    let page_routes = Router::new()
        .route("/", get(get_home))
        .route("/list", get(get_list));

    // build our application with a single route
    let app = Router::new()
        .nest("/api", api_routes)
        .nest("/", page_routes)
        .with_state(AppState {
            category_repository,
            restaurant_repository,
            category_aggregator: Arc::new(category_aggregator),
            template_engine: Engine::from(handlebars)
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_home(
    State(AppState {template_engine, ..}): State<AppState>,
    Key(key): Key
) -> impl IntoResponse {
    RenderHtml(key, template_engine, "")
}

async fn get_list(
    State(AppState {category_aggregator, template_engine, ..}): State<AppState>,
    Key(key): Key
) -> impl IntoResponse {
    let category_restaurants_mapping = category_aggregator.aggregate();
    let data = HashMap::from([("categories", to_json(category_restaurants_mapping))]);
    RenderHtml(key, template_engine, data)
}

async fn get_categories(
    State(state): State<AppState>
) -> Json<Vec<Category>> {
    let categories = state.category_repository.get_all();
    Json(categories)
}

async fn get_restaurants(
    State(state): State<AppState>
) -> Json<Vec<Restaurant>> {
    let restaurants = state.restaurant_repository.get_all();
    Json(restaurants)
}

async fn get_category_by_id(
    Path(category_id): Path<CategoryId>,
    State(state): State<AppState>
) -> Result<Json<Category>, StatusCode> {
    match state.category_repository.get(&category_id) {
        Some(category) => Ok(Json(category.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn get_restaurant_by_id(
    Path(restaurant_id): Path<RestaurantId>,
    State(state): State<AppState>
) -> Result<Json<Restaurant>, StatusCode> {
    match state.restaurant_repository.get(&restaurant_id) {
        Some(restaurant) => Ok(Json(restaurant.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

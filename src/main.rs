use std::sync::Arc;
use axum::{routing::get, Router};
use axum_template::engine::Engine;
use handlebars::Handlebars;
use tower_http::services::ServeDir;
use crate::aggregator::CategoryAggregator;
use crate::api::{get_categories, get_category_by_id, get_restaurant_by_id, get_restaurants};
use crate::construct_page::{construct_home_page, construct_list_page, construct_restaurant_page};
use crate::local_data::generate_data;
use crate::repository::{CategoryRepository, RestaurantRepository};

mod transport;
mod aggregator;
mod repository;
mod model;
mod local_data;
mod construct_page;
mod api;

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
    handlebars.register_template_file("/restaurant/:restaurant_id", "templates/restaurant.hbs").unwrap();

    let api_routes = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories/:category_id", get(get_category_by_id))
        .route("/restaurants", get(get_restaurants))
        .route("/restaurants/:restaurant_id", get(get_restaurant_by_id));

    let page_routes = Router::new()
        .route("/", get(construct_home_page))
        .route("/list", get(construct_list_page))
        .route("/restaurant/:restaurant_id", get(construct_restaurant_page));

    let app = Router::new()
        .nest("/api", api_routes)
        .nest("/", page_routes)
        .nest_service("/static", ServeDir::new("static"))
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

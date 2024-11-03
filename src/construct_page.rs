use axum::extract::State;
use axum::response::IntoResponse;
use axum_template::{Key, RenderHtml};
use handlebars::to_json;
use serde::{Deserialize, Serialize};
use crate::AppState;
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

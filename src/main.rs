use std::collections::HashMap;
use std::sync::Arc;
use axum::{extract::State, routing::get, Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use url::Url;

mod category;
mod restaurant;

use crate::category::model::*;
use crate::category::repository::*;
use crate::restaurant::model::*;
use crate::restaurant::repository::*;

#[derive(Clone)]
struct AppState<CR: CategoryRepository, RR: RestaurantRepository> {
    category_repository: Arc<CR>,
    restaurant_repository: Arc<RR>,
}

#[tokio::main]
async fn main() {

    let (categories, restaurants) = generate_data();

    let category_repository = LocalCategoryRepository::new(categories);
    let restaurant_repository = LocalRestaurantRepository::new(restaurants);

    // build our application with a single route
    let app = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories/:category_id", get(get_category_by_id))
        .route("/restaurants", get(get_restaurants))
        .route("/restaurants/:restaurant_id", get(get_restaurant_by_id))
        .with_state(AppState {
            category_repository: Arc::new(category_repository),
            restaurant_repository: Arc::new(restaurant_repository),
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_categories<CR: CategoryRepository, RR: RestaurantRepository>(
    State(state): State<AppState<CR, RR>>
) -> Json<Vec<Category>> {
    let categories = state.category_repository.get_all();
    Json(categories)
}

async fn get_restaurants<CR: CategoryRepository, RR: RestaurantRepository>(
    State(state): State<AppState<CR, RR>>
) -> Json<Vec<Restaurant>> {
    let restaurants = state.restaurant_repository.get_all();
    Json(restaurants)
}

async fn get_category_by_id<CR: CategoryRepository, RR: RestaurantRepository>(
    Path(category_id): Path<CategoryId>,
    State(state): State<AppState<CR, RR>>
) -> Result<Json<Category>, StatusCode> {
    match state.category_repository.get(&category_id) {
        Some(category) => Ok(Json(category.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn get_restaurant_by_id<CR: CategoryRepository, RR: RestaurantRepository>(
    Path(restaurant_id): Path<RestaurantId>,
    State(state): State<AppState<CR, RR>>
) -> Result<Json<Restaurant>, StatusCode> {
    match state.restaurant_repository.get(&restaurant_id) {
        Some(restaurant) => Ok(Json(restaurant.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

fn generate_data() -> (HashMap<CategoryId, Category>, HashMap<RestaurantId, Restaurant>) {
    let ice_cream_category = Category::new("Ice Cream");

    let oddonos = Restaurant::new("Oddono's", ice_cream_category.id.clone(), Url::try_from("http://www.oddonos.com").unwrap());
    let bilmonte = Restaurant::new("Bilmonte", ice_cream_category.id.clone(), Url::try_from("https://www.bilmonte.it").unwrap());

    let baked_goods_category = Category::new("Baked Goods");

    let crosstown = Restaurant::new("Crosstown", baked_goods_category.id.clone(), Url::try_from("https://www.crosstown.co.uk").unwrap());
    let crumbs_and_doilies = Restaurant::new("Crumbs and Doilies", baked_goods_category.id.clone(), Url::try_from("https://www.crumbsanddoilies.co.uk").unwrap());
    let buns_from_home = Restaurant::new("Buns From Home", baked_goods_category.id.clone(), Url::try_from("https://www.bunsfromhome.com").unwrap());
    let ole_and_steen = Restaurant::new("Ole & Steen", baked_goods_category.id.clone(), Url::try_from("https://oleandsteen.co.uk").unwrap());

    let chocolates_category = Category::new("Chocolates");

    let xo_chocolate = Restaurant::new("XO Chocolate", chocolates_category.id.clone(), Url::try_from("https://www.xochocolate.co.uk").unwrap());
    let laderach = Restaurant::new("Laderach", chocolates_category.id.clone(), Url::try_from("https://laderach.com/uk-en/").unwrap());

    let steak_category = Category::new("Steak");

    let blacklock = Restaurant::new("Blacklock", steak_category.id.clone(), Url::try_from("https://www.theblacklock.com").unwrap());
    let flat_iron = Restaurant::new("Flat Iron", steak_category.id.clone(), Url::try_from("https://flatironsteak.co.uk").unwrap());
    let heliot = Restaurant::new("The Heliot", steak_category.id.clone(), Url::try_from("https://www.hippodromecasino.com/restaurant-bars-lounges/").unwrap());

    let pizza_category = Category::new("Pizza");

    let yard_sale_pizza = Restaurant::new("Yard Sale Pizza", pizza_category.id.clone(), Url::try_from("https://yardsalepizza.com").unwrap());
    let pizza_pilgrims = Restaurant::new("Pizza Pilgrims", pizza_category.id.clone(), Url::try_from("https://pizzapilgrims.co.uk").unwrap());
    let world_famous_gordos = Restaurant::new("World Famous Gordos", pizza_category.id.clone(), Url::try_from("https://www.instagram.com/worldfamousgordos").unwrap());
    let alley_cats_pizza = Restaurant::new("Alley Cats Pizza", pizza_category.id.clone(), Url::try_from("https://www.alleycatspizza.co.uk").unwrap());

    let italian_restaurants_category = Category::new("Italian Restaurants");

    let bancone = Restaurant::new("Bancone", italian_restaurants_category.id.clone(), Url::try_from("https://www.bancone.co.uk").unwrap());
    let padella = Restaurant::new("Padella", italian_restaurants_category.id.clone(), Url::try_from("https://www.padella.co").unwrap());
    let circolo_popolaire = Restaurant::new("Circolo Popolaire", italian_restaurants_category.id.clone(), Url::try_from("https://www.bigmammagroup.com/en/trattorias/circolo-popolare").unwrap());
    let gloria = Restaurant::new("Gloria", italian_restaurants_category.id.clone(), Url::try_from("https://www.bigmammagroup.com/en/trattorias/gloria").unwrap());
    let ave_mario = Restaurant::new("Ave Mario", italian_restaurants_category.id.clone(), Url::try_from("https://www.bigmammagroup.com/en/trattorias/ave-mario").unwrap());
    let luca = Restaurant::new("Luca", italian_restaurants_category.id.clone(), Url::try_from("https://luca.restaurant").unwrap());

    let japanese_restaurants_category = Category::new("Japanese Restaurants");

    let zaibatsu = Restaurant::new("Zaibatsu", japanese_restaurants_category.id.clone(), Url::try_from("https://www.zaibatsufusion.co.uk").unwrap());
    let sticks_n_sushi = Restaurant::new("Sticks 'n' Sushi", japanese_restaurants_category.id.clone(), Url::try_from("https://sticksnsushi.com/en").unwrap());
    let kulu_kulu = Restaurant::new("Kulu Kulu", japanese_restaurants_category.id.clone(), Url::try_from("https://www.kulukulusushi.com").unwrap());
    let maki_yaki = Restaurant::new("Maki Yaki", japanese_restaurants_category.id.clone(), Url::try_from("https://makiyakiepsom.com").unwrap());
    let kintan = Restaurant::new("Kintan Japanese BBQ", japanese_restaurants_category.id.clone(), Url::try_from("https://kintan.uk").unwrap());
    let kampai = Restaurant::new("Kampai Sushi", japanese_restaurants_category.id.clone(), Url::try_from("https://www.kampaisushi.co.uk").unwrap());

    let coffee_category = Category::new("Coffee");

    let formative = Restaurant::new("Formative", coffee_category.id.clone(), Url::try_from("https://formative.coffee").unwrap());

    let brunch_category = Category::new("Brunch");

    let julliets = Restaurant::new("Julliet's", brunch_category.id.clone(), Url::try_from("https://o-juliets.arch2order.com/menu/juliets").unwrap());
    let dropshot = Restaurant::new("DropShot Coffee", brunch_category.id.clone(), Url::try_from("https://www.dropshotcoffee.co.uk").unwrap());
    let utter_waffle = Restaurant::new("Utter Waffle", brunch_category.id.clone(), Url::try_from("https://utterwaffle.co.uk").unwrap());

    let other_category = Category::new("Other fave restaurants");

    let roti_king = Restaurant::new("Roti King", other_category.id.clone(), Url::try_from("https://www.rotiking.com").unwrap());
    let dishoom = Restaurant::new("Dishoom", other_category.id.clone(), Url::try_from("https://www.dishoom.com").unwrap());
    let golden_union = Restaurant::new("Golden Union", other_category.id.clone(), Url::try_from("https://goldenunion.co.uk").unwrap());
    let darjeeling_express = Restaurant::new("Darjeeling Express", other_category.id.clone(), Url::try_from("https://www.darjeeling-express.com").unwrap());

    let categories = HashMap::from([
        (ice_cream_category.id.clone(), ice_cream_category.clone()),
        (baked_goods_category.id.clone(), baked_goods_category.clone()),
        (chocolates_category.id.clone(), chocolates_category.clone()),
        (steak_category.id.clone(), steak_category.clone()),
        (pizza_category.id.clone(), pizza_category.clone()),
        (italian_restaurants_category.id.clone(), italian_restaurants_category.clone()),
        (japanese_restaurants_category.id.clone(), japanese_restaurants_category.clone()),
        (coffee_category.id.clone(), coffee_category.clone()),
        (brunch_category.id.clone(), brunch_category.clone()),
        (other_category.id.clone(), other_category.clone()),
    ]);

    let restaurants = HashMap::from([
        (oddonos.id.clone(), oddonos.clone()),
        (bilmonte.id.clone(), bilmonte.clone()),
        (crosstown.id.clone(), crosstown.clone()),
        (crumbs_and_doilies.id.clone(), crumbs_and_doilies.clone()),
        (buns_from_home.id.clone(), buns_from_home.clone()),
        (ole_and_steen.id.clone(), ole_and_steen.clone()),
        (xo_chocolate.id.clone(), xo_chocolate.clone()),
        (laderach.id.clone(), laderach.clone()),
        (blacklock.id.clone(), blacklock.clone()),
        (flat_iron.id.clone(), flat_iron.clone()),
        (heliot.id.clone(), heliot.clone()),
        (yard_sale_pizza.id.clone(), yard_sale_pizza.clone()),
        (pizza_pilgrims.id.clone(), pizza_pilgrims.clone()),
        (world_famous_gordos.id.clone(), world_famous_gordos.clone()),
        (alley_cats_pizza.id.clone(), alley_cats_pizza.clone()),
        (bancone.id.clone(), bancone.clone()),
        (padella.id.clone(), padella.clone()),
        (circolo_popolaire.id.clone(), circolo_popolaire.clone()),
        (gloria.id.clone(), gloria.clone()),
        (ave_mario.id.clone(), ave_mario.clone()),
        (luca.id.clone(), luca.clone()),
        (zaibatsu.id.clone(), zaibatsu.clone()),
        (sticks_n_sushi.id.clone(), sticks_n_sushi.clone()),
        (kulu_kulu.id.clone(), kulu_kulu.clone()),
        (maki_yaki.id.clone(), maki_yaki.clone()),
        (kintan.id.clone(), kintan.clone()),
        (kampai.id.clone(), kampai.clone()),
        (formative.id.clone(), formative.clone()),
        (julliets.id.clone(), julliets.clone()),
        (dropshot.id.clone(), dropshot.clone()),
        (utter_waffle.id.clone(), utter_waffle.clone()),
        (roti_king.id.clone(), roti_king.clone()),
        (dishoom.id.clone(), dishoom.clone()),
        (golden_union.id.clone(), golden_union.clone()),
        (darjeeling_express.id.clone(), darjeeling_express.clone()),
    ]);

    (categories, restaurants)
}

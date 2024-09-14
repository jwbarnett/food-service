use serde::{Deserialize, Serialize};
use uuid::Uuid;
use url::Url;
use crate::category::model::CategoryId;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RestaurantId(Uuid);


#[derive(Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: RestaurantId,
    pub name: String,
    pub category_id: CategoryId,
    pub website_url: Url,
    pub source: Option<String>
}

impl RestaurantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl From<Uuid> for RestaurantId {
    fn from(uuid: Uuid) -> Self {
        RestaurantId(uuid)
    }
}

impl Restaurant {
    pub fn new(name: &str, category_id: CategoryId, website_url: Url) -> Self {
        Restaurant {
            id: RestaurantId::new(),
            name: name.into(),
            category_id,
            website_url,
            source: None
        }
    }
}

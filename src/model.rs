use uuid::Uuid;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId(Uuid);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RestaurantId(Uuid);

#[derive(Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub name: String
}

impl CategoryId {
    pub fn new() -> Self {
        CategoryId(Uuid::new_v4())
    }
}

impl From<Uuid> for CategoryId {
    fn from(uuid: Uuid) -> Self {
        CategoryId(uuid)
    }
}

impl Category {
    pub fn new(name: &str) -> Self {
        Category {
            id: CategoryId::new(),
            name: name.into()
        }
    }

}

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

use chrono::{DateTime, NaiveDate, Utc};
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
    pub name: String,
    pub created_date_time: DateTime<Utc>
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
            name: name.into(),
            created_date_time: Utc::now(),
        }
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: RestaurantId,
    pub name: String,
    pub category_id: CategoryId,
    pub website_url: Url,
    pub source: Option<String>,
    pub last_visit_date: Option<NaiveDate>,
    pub created_date_time: DateTime<Utc>
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
            source: None,
            last_visit_date: None,
            created_date_time: Utc::now()
        }
    }

    pub fn new_with_last_visit(name: &str, category_id: CategoryId, website_url: Url, last_visit_date: NaiveDate) -> Self {
        Restaurant {
            id: RestaurantId::new(),
            name: name.into(),
            category_id,
            website_url,
            source: None,
            last_visit_date: Some(last_visit_date),
            created_date_time: Utc::now()
        }
    }
}

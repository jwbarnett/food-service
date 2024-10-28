use uuid::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId(Uuid);

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

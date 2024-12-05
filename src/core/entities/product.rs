use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: Option<u32>,
    name: String,
    cost: f64,
    active: bool,
}

impl Product {
    pub fn new(name: String, cost: f64, active: bool, id: Option<u32>) -> Product {
        Product {
            name,
            cost,
            active,
            id,
        }
    }

    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

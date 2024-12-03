#[derive(Clone)]
pub struct Variant {
    id: Option<u32>,
    name: String,
}

impl Variant {
    pub fn new(name: String, id: Option<u32>) -> Variant {
        Variant { name, id }
    }

    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

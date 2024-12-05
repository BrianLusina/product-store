pub struct ProductVariant {
    product_id: i32,
    variant_id: i32,
    value: Option<String>,
}

impl ProductVariant {
    pub fn new(product_id: i32, variant_id: i32, value: Option<String>) -> Self {
        ProductVariant {
            product_id,
            variant_id,
            value,
        }
    }

    pub fn product_id(&self) -> i32 {
        self.product_id
    }

    pub fn variant_id(&self) -> i32 {
        self.variant_id
    }

    pub fn value(&self) -> &Option<String> {
        &self.value
    }
}

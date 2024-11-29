use crate::core::entities::product::Product;
use crate::core::entities::variant_value::VariantValue;

pub struct CompleteProduct {
    product: Product,
    variants: Vec<VariantValue>
}

impl CompleteProduct {
    pub fn new(product: Product, variants: Vec<VariantValue>) -> CompleteProduct {
        Self { product, variants }
    }

    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn variants(&self) -> &[VariantValue] {
        &self.variants
    }
}

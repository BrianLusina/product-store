use crate::core::entities::product::Product;
use crate::core::entities::product_variant::ProductVariant;
use crate::core::entities::variant::Variant;
use crate::datastore::models::product_models::ProductModel;
use crate::datastore::models::variant_models::{ProductVariantModel, VariantModel};

pub fn map_product_model_to_product(product_model: ProductModel) -> Product {
    Product::new(
        product_model.name,
        product_model.cost,
        product_model.active,
        Some(product_model.id as u32)
    )
}

pub fn map_product_variant_model_to_product_variant(product_variant_model: ProductVariantModel) -> ProductVariant {
    ProductVariant::new(
        product_variant_model.product_id,
        product_variant_model.variant_id,
        product_variant_model.value,
    )
}

pub fn map_variant_model_to_variant(variant_model: VariantModel) -> Variant {
    Variant::new(
        variant_model.name,
        Some(variant_model.id as u32),
    )
}

pub fn map_product_and_variant_model_to_variant(model: (ProductVariantModel, VariantModel)) -> (ProductVariant, Variant) {
    (map_product_variant_model_to_product_variant(model.0), map_variant_model_to_variant(model.1))
}

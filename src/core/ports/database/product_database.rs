use crate::core::entities::complete_product::CompleteProduct;
use crate::core::entities::product::Product;
use crate::core::entities::product_variant::ProductVariant;
use crate::core::entities::variant::Variant;
use crate::core::ports::database::utils::ListQueryParams;
use anyhow::Result as AnyResult;

pub trait ProductDatastore {
    // creates a product
    fn create_product(&self, product: Product) -> AnyResult<Product>;

    // creates a complete product
    fn create_complete_product(&self, complete_product: CompleteProduct) -> AnyResult<i32>;

    // get product by a given ID
    fn get_product(&self, id: u32) -> AnyResult<Product>;

    // get product with a given ID with its variants
    fn get_product_with_variants(&self, id: u32) -> AnyResult<(Product, Vec<ProductVariant, Variant>)>;

    // lists products
    fn list_products(&self, params: ListQueryParams) -> Vec<Product>;

    // lists products with their variants
    fn list_products_with_variants(
        &self,
        params: ListQueryParams,
    ) -> AnyResult<Vec<(Product, Vec<ProductVariant, Variant>)>>;
}

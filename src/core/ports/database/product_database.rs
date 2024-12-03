use crate::core::entities::complete_product::CompleteProduct;
use crate::core::entities::product::Product;
use anyhow::Result as AnyResult;

pub trait ProductDatastore {
    fn create_product(&self, product: Product) -> AnyResult<Product>;

    fn create_complete_product(&self, complete_product: CompleteProduct) -> AnyResult<i32>;
}

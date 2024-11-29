use diesel::PgConnection;
use crate::core::entities::product::Product;

pub trait ProductDatastore {
    fn create_product(&self, product: Product) -> Result<Product, Err>;
}

use crate::datastore::models::product_models::{NewProductModel, ProductModel};
use crate::datastore::models::schema::products::dsl::products;
use crate::core::ports::database::product_database::ProductDatastore;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::core::entities::product::Product;

struct ProductRepository<'a> {
    connection: &'a mut PgConnection
}

impl<'a> ProductRepository<'a> {
    fn new(connection: &'a mut PgConnection) -> ProductRepository<'a> {
        ProductRepository { connection }
    }
}

impl ProductDatastore for ProductRepository {
    fn create_product(&self, product: Product) -> Result<Product, Err> {
        let new_product = NewProductModel {
            name: (*product.name()).parse().unwrap(),
            cost: &product.cost(),
            active: *product.active(),
        };

        let result = diesel::insert_into(products)
            .values(new_product)
            .returning(ProductModel::as_returning())
            .get_result(self.connection);

        let record = result.unwrap();

        Result::ok(record)
    }
}

#[cfg(test)]
mod product_repository_tests {
    use crate::datastore::repositories::product_repository::ProductRepository;
    use diesel::Connection;
    use product_store::establish_connection_test;
    use crate::core::ports::database::product_database::ProductDatastore;
    use crate::core::entities::product::Product;

    #[test]
    fn test_create_product() {
        let mut conn = establish_connection_test();
        let product_repository = ProductRepository::new(*conn);

        conn.test_transaction::<_, diesel::result::Error, _>(|_| {
            let product_name = String::from("boots");
            let product_cost = 1323.12;
            let is_product_active = true;

            let actual = product_repository.create_product(
                Product::new(
                    product_name,
                    product_cost,
                    is_product_active,
                    None,
                ),
            );

            let actual_product = actual.unwrap();

            assert_eq!(Some(1), actual_product.id());
            assert_eq!(product_name, actual_product.name());
            assert_eq!(is_product_active, actual_product.active());
            assert_eq!(product_cost, actual_product.cost());

            Ok(())
        })
    }
}

use crate::datastore::models::{NewProductModel, ProductModel};
use crate::datastore::schema::products::dsl::products;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

fn create_product(new_product: NewProductModel, conn: &mut PgConnection) -> Result<ProductModel, Error> {
    diesel::insert_into(products)
        .values(new_product)
        .returning(ProductModel::as_returning())
        .get_result(conn)
}

#[cfg(test)]
mod datastore_tests {
    use diesel::Connection;
    use product_store::establish_connection_test;
    use crate::datastore::datastore::create_product;
    use crate::datastore::models::{NewProductModel, ProductModel};

    #[test]
    fn test_create_product() {
        let mut conn = establish_connection_test();

        conn.test_transaction::<_, diesel::result::Error, _>(|_| {

            let product_name = String::from("boots");
            let product_cost = 1323.12;
            let is_product_active = &true;

            let expected = ProductModel {
                id: 1,
                name: (*product_name).parse().unwrap(),
                cost: product_cost,
                active: *is_product_active,
            };

            let actual = create_product(
                NewProductModel {
                    name: &product_name,
                    cost: &product_cost,
                    active: is_product_active
                }, &mut conn);

            let actual_product = actual.unwrap();

            assert_eq!(expected.id, actual_product.id);
            assert_eq!(expected.name, actual_product.name);
            assert_eq!(expected.active, actual_product.active);
            assert_eq!(expected.cost, actual_product.cost);

            Ok(())
        })
    }
}
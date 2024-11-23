use crate::datastore::models::{NewProductModel, ProductModel};
use crate::datastore::schema::products::dsl::products;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use diesel::result::Error;

fn create_product(new_product: NewProductModel, conn: &mut PgConnection) -> Result<ProductModel, Error> {
    diesel::insert_into(products::table)
        .values(new_product)
        .returning(ProductModel::as_returning())
        .get_result(conn)
        .execute(conn)?
}

#[cfg(test)]
mod datastore_tests {
    use diesel::Connection;
    use diesel::data_types::PgMoney;
    use product_store::establish_connection_test;
    use crate::datastore::datastore::create_product;
    use crate::datastore::models::{NewProductModel, ProductModel};

    #[test]
    fn test_create_product() {
        let mut conn = establish_connection_test();

        conn.test_transaction::<_, diesel::result::Error, _>(|| {

            let product_name = String::from("boots");
            let product_cost = PgMoney(1323);
            let is_product_active = &true;

            let expected = ProductModel {
                id: 1,
                name: (*product_name).parse().unwrap(),
                cost: *product_cost,
                active: *is_product_active,
            };

            let actual = create_product(
                NewProductModel {
                    name: &product_name,
                    cost: &product_cost,
                    active: is_product_active
                }, &mut conn);

            assert_eq!(Ok(expected), actual);

            Ok(())
        })
    }
}
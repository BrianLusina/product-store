use crate::core::entities::complete_product::CompleteProduct;
use crate::core::entities::product::Product;
use crate::core::ports::database::product_database::ProductDatastore;
use crate::datastore::models::product_models::{NewProductModel, ProductModel};
use crate::datastore::models::schema::product_variants::dsl::{
    product_id as product_variant_product_id,
    product_variants, value as product_variant_value,
    variant_id as product_variant_variant_id,
};
use crate::datastore::models::schema::products::dsl::products;
use crate::datastore::models::schema::variants::dsl::{name as variant_name, variants};
use crate::datastore::models::variant_models::VariantModel;
use anyhow::Result as AnyResult;
use diesel::row::NamedRow;
use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

struct ProductRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> ProductRepository<'a> {
    fn new(connection: &'a mut PgConnection) -> ProductRepository<'a> {
        ProductRepository { connection }
    }
}

impl ProductDatastore for ProductRepository<'_> {
    fn create_product(&self, product: Product) -> AnyResult<Product> {
        let new_product = NewProductModel {
            name: (&product.name()).parse().unwrap(),
            cost: &product.cost(),
            active: &product.active(),
        };

        let result = diesel::insert_into(products)
            .values(new_product)
            .returning(ProductModel::as_returning())
            .get_result(self.connection)?;

        Ok(Product::new(result.name, result.cost, result.active, Some(result.id as u32)))
    }

    // creates a new product along with its variants. If variants exists already, they are skipped
    // and if not, a new one is created and that is attached to the product
    fn create_complete_product(&self, complete_product: CompleteProduct) -> AnyResult<i32> {
        self.connection.transaction(|| {
            let created_product = diesel::insert_into(products)
                .values(complete_product.product())
                .returning(ProductModel::as_returning())
                .get_result(self.connection)?;

            for new_variant in complete_product.variants() {
                let variant_results = variants
                    .filter(variant_name.eq(&new_variant.variant().name()))
                    .limit(1)
                    .load::<VariantModel>(self.connection)?;

                let last_variant = match variant_results.first() {
                    Some(variant) => variant,
                    None => {
                        diesel::insert_into(variants)
                            .values(variant_name.eq(&new_variant.variant().name()))
                            .returning(VariantModel::as_returning())
                            .get_result(self.connection)?;
                    }
                };

                for new_value in new_variant.values() {
                    diesel::insert_into(product_variants)
                        .values(
                            (
                                product_variant_product_id.eq(created_product.id),
                                product_variant_variant_id.eq(last_variant.id),
                                product_variant_value.eq(new_value),
                            )
                        )
                        .execute(self.connection)?
                }
            }

            created_product.id
        })
    }
}

#[cfg(test)]
mod product_repository_tests {
    use crate::core::entities::product::Product;
    use crate::core::ports::database::product_database::ProductDatastore;
    use crate::datastore::repositories::product_repository::ProductRepository;
    use diesel::Connection;
    use product_store::establish_connection_test;
    use crate::core::entities::complete_product::CompleteProduct;
    use crate::core::entities::variant::Variant;
    use crate::core::entities::variant_value::VariantValue;

    #[test]
    fn test_create_product() {
        let mut conn = establish_connection_test();
        let product_repository = ProductRepository::new(&mut conn);

        conn.test_transaction::<_, diesel::result::Error, _>(|_| {
            let product_name = String::from("boots");
            let product_cost = 1323.12;
            let is_product_active = true;

            let actual = product_repository.create_product(Product::new(
                product_name,
                product_cost,
                is_product_active,
                None,
            ));

            let actual_product = actual.unwrap();
            let expected_id: u32 = 1;

            assert_eq!(Some(expected_id), actual_product.id());

            Ok(())
        })
    }

    #[test]
    fn test_create_complete_product() {
        let mut conn = establish_connection_test();
        let product_repository = ProductRepository::new(&mut conn);

        conn.test_transaction::<_, diesel::result::Error, _>(|_| {
            let product_name = String::from("boots");
            let product_cost = 1323.12;
            let is_product_active = true;
            let product = Product::new(
                product_name,
                product_cost,
                is_product_active,
                None,
            );
            let variant_value = VariantValue::new(
                Variant::new("size".to_string(), None),
                vec![
                    Some(12.to_string()),
                    Some(14.to_string()),
                    Some(16.to_string()),
                    Some(18.to_string())
                ]
            );

            let variants = vec![variant_value];

            let actual = product_repository.create_complete_product(CompleteProduct::new(
                product,
                variants,
            ));

            assert_eq!(
                serde_json::to_string(&list_products(&connection).unwrap()).unwrap(),
                serde_json::to_string(&vec![
                    (
                        Product {
                            id: 1,
                            name: "boots".to_string(),
                            cost: 13.23,
                            active: true
                        },
                        vec![
                            (
                                Some(12.to_string()),
                                "size".to_string()
                            ),
                            (
                                Some(14.to_string()),
                                "size".to_string()
                            ),
                            (
                                Some(16.to_string()),
                                "size".to_string()
                            ),
                            (
                                Some(18.to_string()),
                                "size".to_string()
                            )
                        ]
                    ),
                ]).unwrap()
            );

            let actual_product = actual.unwrap();
            let expected_id: u32 = 1;

            assert_eq!(Some(expected_id), actual_product.id());

            Ok(())
        })
    }
}

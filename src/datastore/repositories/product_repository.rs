use crate::core::entities::complete_product::CompleteProduct;
use crate::core::entities::product::Product;
use crate::core::entities::product_variant::ProductVariant;
use crate::core::entities::variant::Variant;
use crate::core::ports::database::product_database::ProductDatastore;
use crate::core::ports::database::utils::ListQueryParams;
use crate::datastore::models::product_models::{NewProductModel, ProductModel};
use crate::datastore::models::schema::product_variants::dsl::{
    product_id as product_variant_product_id, product_variants, value as product_variant_value,
    variant_id as product_variant_variant_id,
};
use crate::datastore::models::schema::products::dsl::products;
use crate::datastore::models::schema::variants::dsl::{name as variant_name, variants};
use crate::datastore::models::variant_models::{ProductVariantModel, VariantModel};
use anyhow::Result as AnyResult;
use diesel::row::NamedRow;
use diesel::{
    BelongingToDsl, Connection, GroupedBy, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

struct ProductRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> ProductRepository<'a> {
    fn new(connection: &'a mut PgConnection) -> ProductRepository<'a> {
        ProductRepository { connection }
    }
}

impl<'a> ProductRepository<'a> {
    fn fetch_products(&self, params: ListQueryParams) -> Vec<ProductModel> {
        products
            .limit(params.limit)
            .offset(params.offset)
            .load::<ProductModel>(self.connection)
            .unwrap_or_else(|error| {
                panic!("Error loading products: {:?}", error);
            })
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

        Ok(Product::new(
            result.name,
            result.cost,
            result.active,
            Some(result.id as u32),
        ))
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
                        .values((
                            product_variant_product_id.eq(created_product.id),
                            product_variant_variant_id.eq(last_variant.id),
                            product_variant_value.eq(new_value),
                        ))
                        .execute(self.connection)?
                }
            }

            created_product.id
        })
    }

    fn list_products(&self, params: ListQueryParams) -> Vec<Product> {
        let records = self.fetch_products(params);

        let mut product_records: Vec<Product> = vec![];

        for record in records {
            let product = Product::new(
                record.name,
                record.cost,
                record.active,
                Some(record.id as u32),
            );

            product_records.push(product);
        }

        product_records
    }

    fn list_products_with_variants(
        &self,
        params: ListQueryParams,
    ) -> AnyResult<Vec<(Product, Vec<ProductVariant, Variant>)>> {
        let product_records = self.fetch_products(params);
        let variants_result = ProductVariantModel::belonging_to(&product_records)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(self.connection)?
            .grouped_by(&product_records);

        let data = product_records
            .into_iter()
            .zip(variants_result)
            .collect::<Vec<_>>();

        Ok(data)
    }
}

#[cfg(test)]
mod product_repository_tests {
    use crate::core::entities::product::Product;
    use crate::core::ports::database::product_database::ProductDatastore;
    use crate::core::ports::database::utils::ListQueryParams;
    use crate::datastore::repositories::product_repository::ProductRepository;
    use diesel::Connection;
    use product_store::establish_connection_test;

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

    // #[test]
    // fn test_create_complete_product() {
    //     let mut conn = establish_connection_test();
    //     let product_repository = ProductRepository::new(&mut conn);
    //
    //     conn.test_transaction::<_, diesel::result::Error, _>(|_| {
    //         let product_name = String::from("boots");
    //         let product_cost = 1323.12;
    //         let is_product_active = true;
    //         let product = Product::new(
    //             product_name,
    //             product_cost,
    //             is_product_active,
    //             None,
    //         );
    //         let variant_value = VariantValue::new(
    //             Variant::new("size".to_string(), None),
    //             vec![
    //                 Some(12.to_string()),
    //                 Some(14.to_string()),
    //                 Some(16.to_string()),
    //                 Some(18.to_string())
    //             ]
    //         );
    //
    //         let variants = vec![variant_value];
    //
    //         let actual = product_repository.create_complete_product(CompleteProduct::new(
    //             product,
    //             variants,
    //         ));
    //
    //         assert_eq!(
    //             // serde_json::to_string(&list_products(&connection).unwrap()).unwrap(),
    //             serde_json::to_string(&vec![
    //                 (
    //                     Product {
    //                         id: 1,
    //                         name: "boots".to_string(),
    //                         cost: 13.23,
    //                         active: true
    //                     },
    //                     vec![
    //                         (
    //                             Some(12.to_string()),
    //                             "size".to_string()
    //                         ),
    //                         (
    //                             Some(14.to_string()),
    //                             "size".to_string()
    //                         ),
    //                         (
    //                             Some(16.to_string()),
    //                             "size".to_string()
    //                         ),
    //                         (
    //                             Some(18.to_string()),
    //                             "size".to_string()
    //                         )
    //                     ]
    //                 ),
    //             ]).unwrap()
    //         );
    //
    //         let actual_product = actual.unwrap();
    //         let expected_id: u32 = 1;
    //
    //         assert_eq!(Some(expected_id), actual_product);
    //
    //         Ok(())
    //     })
    // }

    #[test]
    fn test_list_products() {
        use diesel::result::Error;

        let mut database_connection = establish_connection_test();
        let product_repository = ProductRepository::new(&mut database_connection);
        let product_one = Product::new("boots".to_string(), 13.23, true, None);

        let product_two = Product::new("running shoes".to_string(), 10.99, true, None);
        let product_three = Product::new("running shoes".to_string(), 10.99, true, None);

        database_connection.test_transaction::<_, Error, _>(|| {
            product_repository
                .create_product(product_one)
                .expect("Error creating product");
            product_repository
                .create_product(product_two)
                .expect("Should be able to created");
            product_repository
                .create_product(product_three)
                .expect("Failed to insert product ");

            let actual_products = product_repository.list_products(ListQueryParams {
                limit: 10,
                offset: 10,
            });

            assert_eq!(
                serde_json::to_string(&actual_products).unwrap(),
                serde_json::to_string(&vec![
                    Product::new("boots".to_string(), 13.23, true, Some(1),),
                    Product::new("high heels".to_string(), 20.99, true, Some(2),),
                    Product::new("running shoes".to_string(), 10.99, true, Some(3),)
                ])
                .unwrap()
            );

            Ok(())
        });
    }
}

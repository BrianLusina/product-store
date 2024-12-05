use crate::datastore::models::schema::{
    product_variants as ProductVariantsTable, products, variants as VariantsTable,
};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Selectable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = VariantsTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VariantModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = VariantsTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewVariantModel {
    pub name: String,
}

#[derive(Debug, Insertable, Associations)]
#[diesel(belongs_to(products))]
#[diesel(table_name = ProductVariantsTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductVariantModel {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>,
}

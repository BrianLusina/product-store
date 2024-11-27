use crate::datastore::schema::{
    product_variants as ProductVariantsTable, variants as VariantsTable,
};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
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

#[derive(Debug, Insertable)]
#[diesel(table_name = ProductVariantsTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProductVariantModel {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>,
}

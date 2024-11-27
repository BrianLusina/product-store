use crate::datastore::schema::products as ProductsTable;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = ProductsTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = ProductsTable)]
pub struct NewProductModel<'a> {
    pub name: &'a String,
    pub cost: &'a f64,
    pub active: &'a bool,
}

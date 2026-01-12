pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251226_144810_ool_10026;
mod m20251227_100831_customer_order_product;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251226_144810_ool_10026::Migration),
            Box::new(m20251227_100831_customer_order_product::Migration),
        ]
    }
}

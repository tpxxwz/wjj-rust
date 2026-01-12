use crate::m20220101_000001_create_table;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        m20220101_000001_create_table::Migration.down(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        m20220101_000001_create_table::Migration.up(manager).await
    }
}

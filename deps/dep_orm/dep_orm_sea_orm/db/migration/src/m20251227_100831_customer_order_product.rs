use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 customers 表
        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Customers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Customers::Name).string().not_null())
                    .col(ColumnDef::new(Customers::Email).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建 products 表
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::Price).double().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建 orders 表
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Orders::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Orders::CustomerId).integer().not_null())
                    .col(ColumnDef::new(Orders::TotalPrice).double().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-customer_id")
                            .from(Orders::Table, Orders::CustomerId)
                            .to(Customers::Table, Customers::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除 orders 表
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;

        // 删除 products 表
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await?;

        // 删除 customers 表
        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await?;

        Ok(())
    }
}

// =======================
// 定义枚举用于列名
// =======================
#[derive(Iden)]
enum Customers {
    Table,
    Id,
    Name,
    Email,
}

#[derive(Iden)]
enum Orders {
    Table,
    Id,
    CustomerId,
    TotalPrice,
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    Name,
    Price,
}

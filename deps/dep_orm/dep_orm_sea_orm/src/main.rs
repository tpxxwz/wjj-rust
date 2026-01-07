// cargo install sea-orm-cli
// sea-orm-cli migrate init
// sea-orm-cli migrate init --migration-dir ./db/migration
// 在生成的migration项目的cargo.toml 文件加上空的[workspace] 在futures里面加上 "runtime-tokio-native-tls", "sqlx-postgres"
// sea-orm-cli migrate up -d ./db/migration // 运行迁移
// sea-orm-cli migrate down -d ./db/migration // 或回滚
// sea-orm-cli migrate generate -d ./db/migration ool_10026

// 已有的库生成rust对象
// sea-orm-cli generate entity -u postgres://postgres:wzzst310@localhost:5432/sea_orm -o entity

// 当然提前定义好 entity的mod
mod entity;

use std::env;
use dotenvy::dotenv;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DbBackend, DbErr, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, QuerySelect, Set, Statement, TransactionTrait,
};

// =======================
// customers 模块
// =======================
mod customers {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "customers")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub email: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Orders,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Orders => Entity::has_many(orders::Entity).into(),
            }
        }
    }

    impl ActiveModelBehavior for ActiveModel {}

    impl Related<orders::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Orders.def()
        }
    }
}

// =======================
// orders 模块
// =======================
mod orders {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "orders")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub customer_id: i32,
        pub total_price: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Customer,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Customer => Entity::belongs_to(customers::Entity)
                    .from(Column::CustomerId)
                    .to(customers::Column::Id)
                    .into(),
            }
        }
    }

    impl Related<customers::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Customer.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// =======================
// products 模块
// =======================
mod products {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "products")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub price: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            panic!("No relations defined")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// =======================
// 自定义结构体
// =======================
#[derive(Debug, FromQueryResult)]
struct CustomerSummary {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    // from_filename(".env_prod").ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:wzzst310@localhost:5432/sea_orm".to_string());
    let db = Database::connect(database_url).await?;

    // ✅ 插入 Customer
    let new_customer = customers::ActiveModel {
        name: Set("Alice".to_owned()),
        email: Set("alice@example.com".to_owned()),
        ..Default::default()
    };
    let inserted_customer = new_customer.insert(&db).await?;
    println!("Inserted Customer ID: {}", inserted_customer.id);

    // ✅ 插入 Order
    let new_order = orders::ActiveModel {
        customer_id: Set(inserted_customer.id),
        total_price: Set(199.99),
        ..Default::default()
    };
    new_order.insert(&db).await?;

    // ✅ 查询 Customer + Orders（关联查询）
    let customers_with_orders = customers::Entity::find()
        .find_with_related(orders::Entity)
        .all(&db)
        .await?;
    println!("Customers with Orders: {:?}", customers_with_orders);

    // ✅ 自定义结构体映射
    let summaries: Vec<CustomerSummary> = customers::Entity::find()
        .select_only()
        .column(customers::Column::Name)
        .column(customers::Column::Email)
        .into_model::<CustomerSummary>()
        .all(&db)
        .await?;
    println!("Customer summaries: {:?}", summaries);

    // 批量插入
    let customers = vec![
        customers::ActiveModel {
            name: Set("Alice".to_owned()),
            email: Set("alice@example.com".to_owned()),
            ..Default::default()
        },
        customers::ActiveModel {
            name: Set("Bob".to_owned()),
            email: Set("bob@example.com".to_owned()),
            ..Default::default()
        },
        customers::ActiveModel {
            name: Set("Charlie".to_owned()),
            email: Set("charlie@example.com".to_owned()),
            ..Default::default()
        },
    ];
    customers::Entity::insert_many(customers).exec(&db).await?;
    println!("Inserted multiple customers!");

    // ✅ 事务示例：插入多个记录并回滚
    let txn = db.begin().await?;
    let customer_txn = customers::ActiveModel {
        name: Set("Bob".to_owned()),
        email: Set("bob@example.com".to_owned()),
        ..Default::default()
    };
    customer_txn.insert(&txn).await?;

    let order_txn = orders::ActiveModel {
        customer_id: Set(inserted_customer.id),
        total_price: Set(500.0),
        ..Default::default()
    };
    order_txn.insert(&txn).await?;
    println!("Rolling back transaction...");
    txn.rollback().await?;
    println!("Transaction rolled back!");

    // ✅ 条件更新
    customers::Entity::update_many()
        .col_expr(customers::Column::Email, Expr::value("updated@example.com"))
        .filter(customers::Column::Name.eq("Alice"))
        .exec(&db)
        .await?;
    println!("Updated Alice's email");

    // ✅ 条件删除
    orders::Entity::delete_many()
        .filter(orders::Column::TotalPrice.gt(100.0))
        .exec(&db)
        .await?;
    println!("Deleted orders with price > 100");

    // ✅ 自定义 SQL 查询
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT name, email FROM customers WHERE id > $1",
        vec![0.into()],
    );
    let summaries2: Vec<CustomerSummary> =
        CustomerSummary::find_by_statement(stmt).all(&db).await?;
    println!("Custom SQL summaries: {:?}", summaries2);

    Ok(())
}

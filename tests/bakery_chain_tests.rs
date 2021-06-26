use sea_orm::{sea_query, DbConn, ExecErr, ExecResult};
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, SqliteQueryBuilder};

pub mod bakery_chain;
mod setup;
pub use bakery_chain::*;

#[async_std::test]
// cargo test --test bakery_chain_tests -- --nocapture
async fn main() {
    let db: DbConn = setup::setup().await;
    setup_schema(&db).await;
}

async fn setup_schema(db: &DbConn) {
    assert!(create_bakery(db).await.is_ok());
    assert!(create_baker(db).await.is_ok());
}

async fn create_bakery(db: &DbConn) -> Result<ExecResult, ExecErr> {
    let stmt = sea_query::Table::create()
        .table(bakery::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(bakery::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(bakery::Column::Name).string())
        .col(ColumnDef::new(bakery::Column::ProfitMargin).float())
        .build(SqliteQueryBuilder);

    db.execute(stmt.into()).await
}

async fn create_baker(db: &DbConn) -> Result<ExecResult, ExecErr> {
    let stmt = sea_query::Table::create()
        .table(baker::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(baker::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(baker::Column::Name).string())
        .col(ColumnDef::new(baker::Column::BakeryId).integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from(baker::Entity, baker::Column::BakeryId)
                .to(bakery::Entity, bakery::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .build(SqliteQueryBuilder);

    db.execute(stmt.clone().into()).await
}

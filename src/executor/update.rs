use crate::{
    error::*, ActiveModelTrait, DatabaseConnection, EntityTrait, Statement, UpdateMany, UpdateOne,
};
use sea_query::UpdateStatement;
use std::future::Future;

#[derive(Clone, Debug)]
pub struct Updater {
    query: UpdateStatement,
    check_record_exists: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateResult {
    pub rows_affected: u64,
}

impl<'a, A: 'a> UpdateOne<A>
where
    A: ActiveModelTrait,
{
    pub fn exec(self, db: &'a DatabaseConnection) -> impl Future<Output = Result<A, DbErr>> + 'a {
        // so that self is dropped before entering await
        exec_update_and_return_original(self.query, self.model, db)
    }
}

impl<'a, E> UpdateMany<E>
where
    E: EntityTrait,
{
    pub fn exec(
        self,
        db: &'a DatabaseConnection,
    ) -> impl Future<Output = Result<UpdateResult, DbErr>> + 'a {
        // so that self is dropped before entering await
        exec_update_only(self.query, db)
    }
}

impl Updater {
    pub fn new(query: UpdateStatement) -> Self {
        Self {
            query,
            check_record_exists: false,
        }
    }

    pub fn check_record_exists(mut self) -> Self {
        self.check_record_exists = true;
        self
    }

    pub fn exec(
        self,
        db: &DatabaseConnection,
    ) -> impl Future<Output = Result<UpdateResult, DbErr>> + '_ {
        let builder = db.get_database_backend();
        exec_update(builder.build(&self.query), db, self.check_record_exists)
    }
}

async fn exec_update_only(
    query: UpdateStatement,
    db: &DatabaseConnection,
) -> Result<UpdateResult, DbErr> {
    Updater::new(query).exec(db).await
}

async fn exec_update_and_return_original<A>(
    query: UpdateStatement,
    model: A,
    db: &DatabaseConnection,
) -> Result<A, DbErr>
where
    A: ActiveModelTrait,
{
    Updater::new(query).check_record_exists().exec(db).await?;
    Ok(model)
}

// Only Statement impl Send
async fn exec_update(
    statement: Statement,
    db: &DatabaseConnection,
    check_record_exists: bool,
) -> Result<UpdateResult, DbErr> {
    let result = db.execute(statement).await?;
    if check_record_exists && result.rows_affected() == 0 {
        return Err(DbErr::RecordNotFound(
            "None of the database rows are affected".to_owned(),
        ));
    }
    Ok(UpdateResult {
        rows_affected: result.rows_affected(),
    })
}

#[cfg(test)]
mod tests {
    use crate::{entity::prelude::*, tests_cfg::*, *};
    use pretty_assertions::assert_eq;
    use sea_query::Expr;

    #[smol_potat::test]
    async fn update_record_not_found_1() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                },
            ])
            .into_connection();

        let model = cake::Model {
            id: 1,
            name: "New York Cheese".to_owned(),
        };

        assert_eq!(
            cake::ActiveModel {
                name: Set("Cheese Cake".to_owned()),
                ..model.into_active_model()
            }
            .update(&db)
            .await?,
            cake::Model {
                id: 1,
                name: "Cheese Cake".to_owned(),
            }
            .into_active_model()
        );

        let model = cake::Model {
            id: 2,
            name: "New York Cheese".to_owned(),
        };

        assert_eq!(
            cake::ActiveModel {
                name: Set("Cheese Cake".to_owned()),
                ..model.clone().into_active_model()
            }
            .update(&db)
            .await,
            Err(DbErr::RecordNotFound(
                "None of the database rows are affected".to_owned()
            ))
        );

        assert_eq!(
            cake::Entity::update(cake::ActiveModel {
                name: Set("Cheese Cake".to_owned()),
                ..model.clone().into_active_model()
            })
            .exec(&db)
            .await,
            Err(DbErr::RecordNotFound(
                "None of the database rows are affected".to_owned()
            ))
        );

        assert_eq!(
            Update::one(cake::ActiveModel {
                name: Set("Cheese Cake".to_owned()),
                ..model.into_active_model()
            })
            .exec(&db)
            .await,
            Err(DbErr::RecordNotFound(
                "None of the database rows are affected".to_owned()
            ))
        );

        assert_eq!(
            Update::many(cake::Entity)
                .col_expr(cake::Column::Name, Expr::value("Cheese Cake".to_owned()))
                .filter(cake::Column::Id.eq(2))
                .exec(&db)
                .await,
            Ok(UpdateResult { rows_affected: 0 })
        );

        assert_eq!(
            db.into_transaction_log(),
            vec![
                Transaction::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"UPDATE "cake" SET "name" = $1 WHERE "cake"."id" = $2"#,
                    vec!["Cheese Cake".into(), 1i32.into()]
                ),
                Transaction::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"UPDATE "cake" SET "name" = $1 WHERE "cake"."id" = $2"#,
                    vec!["Cheese Cake".into(), 2i32.into()]
                ),
                Transaction::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"UPDATE "cake" SET "name" = $1 WHERE "cake"."id" = $2"#,
                    vec!["Cheese Cake".into(), 2i32.into()]
                ),
                Transaction::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"UPDATE "cake" SET "name" = $1 WHERE "cake"."id" = $2"#,
                    vec!["Cheese Cake".into(), 2i32.into()]
                ),
                Transaction::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"UPDATE "cake" SET "name" = $1 WHERE "cake"."id" = $2"#,
                    vec!["Cheese Cake".into(), 2i32.into()]
                ),
            ]
        );

        Ok(())
    }
}

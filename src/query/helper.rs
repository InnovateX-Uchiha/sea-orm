use crate::{ColumnTrait, Identity, IntoSimpleExpr, RelationDef};

pub use sea_query::JoinType;
use sea_query::{Alias, Expr, Order, SelectExpr, SelectStatement, SimpleExpr};
use std::rc::Rc;

pub trait QueryHelper: Sized {
    fn query(&mut self) -> &mut SelectStatement;

    /// Clear the selection list
    fn select_only(mut self) -> Self {
        self.query().clear_selects();
        self
    }

    /// Add a select column
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake""#
    /// );
    /// ```
    fn column<C>(mut self, col: C) -> Self
    where
        C: ColumnTrait,
    {
        self.query().expr(col.into_simple_expr());
        self
    }

    /// Add a select column with alias
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column_as(cake::Column::Id.count(), "count")
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT COUNT("cake"."id") AS "count" FROM "cake""#
    /// );
    /// ```
    fn column_as<C>(mut self, col: C, alias: &str) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().expr(SelectExpr {
            expr: col.into_simple_expr(),
            alias: Some(Rc::new(Alias::new(alias))),
        });
        self
    }

    /// Add an AND WHERE expression
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.eq(5))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = 5"
    /// );
    /// ```
    fn filter(mut self, expr: SimpleExpr) -> Self {
        self.query().and_where(expr);
        self
    }

    /// Add a group by column
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .group_by(cake::Column::Name)
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake" GROUP BY "cake"."name""#
    /// );
    /// ```
    fn group_by<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().add_group_by(vec![col.into_simple_expr()]);
        self
    }

    /// Add an order_by expression (ascending)
    /// ```
    /// use sea_orm::{EntityTrait, QueryHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by(cake::Column::Id)
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` ASC"
    /// );
    /// ```
    fn order_by<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Asc);
        self
    }

    /// Add an order_by expression (descending)
    /// ```
    /// use sea_orm::{EntityTrait, QueryHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_desc(cake::Column::Id)
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` DESC"
    /// );
    /// ```
    fn order_by_desc<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Desc);
        self
    }

    #[doc(hidden)]
    fn join_join(mut self, join: JoinType, rel: RelationDef, via: Option<RelationDef>) -> Self {
        if let Some(via) = via {
            self = self.join(join, via)
        }
        self.join(join, rel)
    }

    /// Join via [`RelationDef`].
    fn join(mut self, join: JoinType, rel: RelationDef) -> Self {
        let from_tbl = rel.from_tbl.clone();
        let to_tbl = rel.to_tbl.clone();
        let owner_keys = rel.from_col;
        let foreign_keys = rel.to_col;
        let condition = match (owner_keys, foreign_keys) {
            (Identity::Unary(o1), Identity::Unary(f1)) => {
                Expr::tbl(Rc::clone(&from_tbl), o1).equals(Rc::clone(&to_tbl), f1)
            } // _ => panic!("Owner key and foreign key mismatch"),
        };
        self.query().join(join, Rc::clone(&to_tbl), condition);
        self
    }

    /// Join via [`RelationDef`] but in reverse direction.
    /// Assume when there exist a relation A -> B.
    /// You can reverse join B <- A.
    fn join_rev(mut self, join: JoinType, rel: RelationDef) -> Self {
        let from_tbl = rel.from_tbl.clone();
        let to_tbl = rel.to_tbl.clone();
        let owner_keys = rel.from_col;
        let foreign_keys = rel.to_col;
        let condition = match (owner_keys, foreign_keys) {
            (Identity::Unary(o1), Identity::Unary(f1)) => {
                Expr::tbl(Rc::clone(&from_tbl), o1).equals(Rc::clone(&to_tbl), f1)
            } // _ => panic!("Owner key and foreign key mismatch"),
        };
        self.query().join(join, Rc::clone(&from_tbl), condition);
        self
    }
}

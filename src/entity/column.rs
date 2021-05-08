use crate::EntityTrait;
pub use sea_query::ColumnType;
use sea_query::{Expr, Iden, SimpleExpr, Value};
use std::rc::Rc;

macro_rules! bind_oper {
    ( $op: ident ) => {
        fn $op<V>(&'static self, v: V) -> SimpleExpr
        where
            V: Into<Value>,
        {
            Expr::tbl(self.entity_iden(), *self).$op(v)
        }
    };
}

pub trait ColumnTrait: Iden + Copy {
    type Entity: EntityTrait;

    fn col_type(&self) -> ColumnType;

    fn entity_iden(&'static self) -> Rc<dyn Iden> {
        Rc::new(Self::Entity::default()) as Rc<dyn Iden>
    }

    bind_oper!(eq);
    bind_oper!(ne);
    bind_oper!(gt);
    bind_oper!(gte);
    bind_oper!(lt);
    bind_oper!(lte);

    fn between<V>(&'static self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::tbl(self.entity_iden(), *self).between(a, b)
    }

    fn not_between<V>(&'static self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::tbl(self.entity_iden(), *self).not_between(a, b)
    }

    fn like(&'static self, s: &str) -> SimpleExpr {
        Expr::tbl(self.entity_iden(), *self).like(s)
    }

    fn not_like(&'static self, s: &str) -> SimpleExpr {
        Expr::tbl(self.entity_iden(), *self).not_like(s)
    }

    fn starts_with(&'static self, s: &str) -> SimpleExpr {
        let pattern = format!("{}%", s);
        Expr::tbl(self.entity_iden(), *self).like(&pattern)
    }

    fn ends_with(&'static self, s: &str) -> SimpleExpr {
        let pattern = format!("%{}", s);
        Expr::tbl(self.entity_iden(), *self).like(&pattern)
    }

    fn contains(&'static self, s: &str) -> SimpleExpr {
        let pattern = format!("%{}%", s);
        Expr::tbl(self.entity_iden(), *self).like(&pattern)
    }
}

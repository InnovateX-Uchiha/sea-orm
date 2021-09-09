use crate::{DbErr, EntityTrait, Linked, QueryFilter, QueryResult, Related, Select};
pub use sea_query::Value;
use std::fmt::Debug;

pub trait ModelTrait: Clone + Send + Debug {
    type Entity: EntityTrait;

    fn get(&self, c: <Self::Entity as EntityTrait>::Column) -> Value;

    fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: Value);

    fn find_related<R>(&self, _: R) -> Select<R>
    where
        R: EntityTrait,
        Self::Entity: Related<R>,
    {
        <Self::Entity as Related<R>>::find_related().belongs_to(self)
    }

    fn find_linked<L>(&self, l: L) -> Select<L::ToEntity>
    where
        L: Linked<FromEntity = Self::Entity>,
    {
        l.find_linked()
    }
}

pub trait FromQueryResult {
    fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, DbErr>
    where
        Self: Sized;

    fn from_query_result_optional(res: &QueryResult, pre: &str) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        Ok(Self::from_query_result(res, pre).ok())
    }
}

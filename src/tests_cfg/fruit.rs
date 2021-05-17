use crate::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[entity = "fruit"]
pub struct Entity;

#[derive(Clone, Debug, Default, PartialEq, DeriveModel)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub cake_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    CakeId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl EntityTrait for Entity {
    type Model = Model;

    type Column = Column;

    type PrimaryKey = PrimaryKey;

    type Relation = Relation;
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnType {
        match self {
            Self::Id => ColumnType::Integer(None),
            Self::Name => ColumnType::String(None),
            Self::CakeId => ColumnType::Integer(None),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!()
    }
}

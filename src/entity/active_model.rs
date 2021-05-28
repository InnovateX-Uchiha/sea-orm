use crate::{ColumnTrait, EntityTrait, Value};
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct ActiveValue<V>
where
    V: Default,
{
    value: V,
    state: ActiveValueState,
}

#[derive(Clone, Debug)]
enum ActiveValueState {
    Set,
    Unchanged,
    Unset,
}

impl Default for ActiveValueState {
    fn default() -> Self {
        Self::Unset
    }
}

pub fn unchanged_active_value_not_intended_for_public_use<V>(value: V) -> ActiveValue<V>
where
    V: Default,
{
    ActiveValue::unchanged(value)
}

impl<V> ActiveValue<V>
where
    V: Default,
{
    pub fn set(value: V) -> Self {
        Self {
            value,
            state: ActiveValueState::Set,
        }
    }

    pub fn is_set(&self) -> bool {
        matches!(self.state, ActiveValueState::Set)
    }

    pub(crate) fn unchanged(value: V) -> Self {
        Self {
            value,
            state: ActiveValueState::Unchanged,
        }
    }

    pub fn unset() -> Self {
        Self {
            value: V::default(),
            state: ActiveValueState::Unset,
        }
    }

    pub fn is_unset(&self) -> bool {
        matches!(self.state, ActiveValueState::Unset)
    }

    pub fn take(&mut self) -> V {
        self.state = ActiveValueState::Unset;
        std::mem::take(&mut self.value)
    }

    pub fn unwrap(self) -> V {
        self.value
    }
}

impl<V> std::convert::AsRef<V> for ActiveValue<V>
where
    V: Default,
{
    fn as_ref(&self) -> &V {
        &self.value
    }
}

impl<V> ActiveValue<V>
where
    V: Default + Into<Value>,
{
    pub fn into_value(self) -> Value {
        self.value.into()
    }

    pub fn into_wrapped_value(self) -> ActiveValue<Value> {
        match self.state {
            ActiveValueState::Set => ActiveValue::set(self.into_value()),
            ActiveValueState::Unchanged => ActiveValue::set(self.into_value()),
            ActiveValueState::Unset => ActiveValue::unset(),
        }
    }
}

pub trait ActiveModelOf<E>
where
    E: EntityTrait,
{
}

pub trait ActiveModelTrait: Clone + Debug {
    type Column: ColumnTrait;

    fn take(&mut self, c: Self::Column) -> ActiveValue<Value>;

    fn get(&self, c: Self::Column) -> ActiveValue<Value>;

    fn set(&mut self, c: Self::Column, v: Value);

    fn unset(&mut self, c: Self::Column);
}

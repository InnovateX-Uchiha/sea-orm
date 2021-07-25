use crate::{
    Column, ConjunctRelation, Entity, EntityWriter, Error, PrimaryKey, Relation, RelationType,
};
use sea_query::TableStatement;
use sea_schema::mysql::def::Schema;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EntityTransformer {
    pub(crate) schema: Schema,
}

impl EntityTransformer {
    pub fn transform(self) -> Result<EntityWriter, Error> {
        let mut inverse_relations: HashMap<String, Vec<Relation>> = HashMap::new();
        let mut conjunct_relations: HashMap<String, Vec<ConjunctRelation>> = HashMap::new();
        let mut entities = HashMap::new();
        for table_ref in self.schema.tables.iter() {
            let table_stmt = table_ref.write();
            let table_create = match table_stmt {
                TableStatement::Create(stmt) => stmt,
                _ => {
                    return Err(Error::TransformError(
                        "TableStatement should be create".into(),
                    ))
                }
            };
            let table_name = match table_create.get_table_name() {
                Some(s) => s,
                None => {
                    return Err(Error::TransformError(
                        "Table name should not be empty".into(),
                    ))
                }
            };
            let columns: Vec<Column> = table_create
                .get_columns()
                .iter()
                .map(|col_def| col_def.into())
                .collect();
            let unique_columns: Vec<String> = columns
                .iter()
                .filter(|col| col.unique)
                .map(|col| col.name.clone())
                .collect();
            let relations = table_create
                .get_foreign_key_create_stmts()
                .iter()
                .map(|fk_create_stmt| fk_create_stmt.get_foreign_key())
                .map(|tbl_fk| tbl_fk.into());
            let primary_keys = table_create
                .get_indexes()
                .iter()
                .filter(|index| index.is_primary_key())
                .map(|index| {
                    index
                        .get_index_spec()
                        .get_column_names()
                        .into_iter()
                        .map(|name| PrimaryKey { name })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            let entity = Entity {
                table_name: table_name.clone(),
                columns,
                relations: relations.clone().collect(),
                conjunct_relations: vec![],
                primary_keys,
            };
            entities.insert(table_name.clone(), entity.clone());
            for (i, mut rel) in relations.into_iter().enumerate() {
                let is_conjunct_relation = entity.primary_keys.len() == entity.columns.len()
                    && entity.primary_keys.len() == 2;
                match is_conjunct_relation {
                    true => {
                        let another_rel = entity.relations.get((i == 0) as usize).unwrap();
                        let conjunct_relation = ConjunctRelation {
                            via: table_name.clone(),
                            to: another_rel.ref_table.clone(),
                        };
                        if let Some(vec) = conjunct_relations.get_mut(&rel.ref_table) {
                            vec.push(conjunct_relation);
                        } else {
                            conjunct_relations.insert(rel.ref_table, vec![conjunct_relation]);
                        }
                    }
                    false => {
                        let ref_table = rel.ref_table;
                        let mut unique = true;
                        for col in rel.columns.iter() {
                            if !unique_columns.contains(col) {
                                unique = false;
                                break;
                            }
                        }
                        let rel_type = if unique {
                            RelationType::HasOne
                        } else {
                            RelationType::HasMany
                        };
                        rel.rel_type = rel_type;
                        rel.ref_table = table_name.clone();
                        rel.columns = Vec::new();
                        rel.ref_columns = Vec::new();
                        if let Some(vec) = inverse_relations.get_mut(&ref_table) {
                            vec.push(rel);
                        } else {
                            inverse_relations.insert(ref_table, vec![rel]);
                        }
                    }
                }
            }
        }
        for (tbl_name, mut relations) in inverse_relations.into_iter() {
            if let Some(entity) = entities.get_mut(&tbl_name) {
                entity.relations.append(&mut relations);
            }
        }
        for (tbl_name, mut conjunct_relations) in conjunct_relations.into_iter() {
            if let Some(entity) = entities.get_mut(&tbl_name) {
                entity.conjunct_relations.append(&mut conjunct_relations);
            }
        }
        Ok(EntityWriter {
            entities: entities.into_iter().map(|(_, v)| v).collect(),
        })
    }
}

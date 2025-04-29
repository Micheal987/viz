//! todo model

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Eq, PartialEq, Serialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde[skip_deserializing]]
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

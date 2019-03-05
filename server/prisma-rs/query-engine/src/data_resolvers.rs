mod sqlite;

pub use sqlite::Sqlite;
pub type PrismaDataResolver = Box<dyn DataResolver + Send + Sync + 'static>;

use crate::{ordering::OrderVec, protobuf::prelude::*};
use prisma_common::PrismaResult;
use prisma_models::prelude::*;
use prisma_query::ast::*;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug)]
pub struct SelectQuery {
    pub db_name: String,
    pub query_ast: Select,
    pub selected_fields: SelectedFields,
}

pub trait IntoSelectQuery {
    fn into_select_query(self) -> PrismaResult<SelectQuery>;
}

pub trait DataResolver {
    fn select_nodes(&self, query: SelectQuery) -> PrismaResult<(Vec<Vec<PrismaValue>>, Vec<String>)>;
}

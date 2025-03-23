use crate::query::components::model::QueryComponent;
use crate::query::{Expression, NativeQuery, VirtualColumn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DataSource {
    Table {
        name: String,
    },

    //Lookup {},
    Union {
        data_sources: Vec<String>,
    },

    Inline {
        column_names: Vec<String>,
        rows: Vec<Vec<String>>,
    },

    Query {
        query: Box<NativeQuery>,
    },

    Join {
        left: Box<DataSource>,
        right: Box<DataSource>,
        right_prefix: String,
        condition: Expression,
        join_type: String,
    },

    Unnest {
        base: Box<DataSource>,
        virtual_column: VirtualColumn,
        unnest_filter: Option<String>,
    },

    #[serde(untagged)]
    String(String),
}

impl QueryComponent for DataSource {
    fn validate_type(&self) -> bool {
        match self {
            //TODO
            DataSource::String(value) => !value.is_empty(),
            DataSource::Table { .. } => true,
            DataSource::Union { .. } => true,
            DataSource::Inline { .. } => true,
            DataSource::Query { .. } => true,
            DataSource::Join { .. } => true,
            DataSource::Unnest { .. } => true,
        }
    }
}

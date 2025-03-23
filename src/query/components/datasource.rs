use crate::query::{Expression, NativeQuery, VirtualColumn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataSourceType {
    Table,
    Union,
    Inline,
    Query,
    Join,
    Unnest,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DataSource {
    String(String),

    Table {
        #[serde(rename = "type")]
        type_: DataSourceType,
        name: String,
    },

    //Lookup {},
    Union {
        #[serde(rename = "type")]
        type_: DataSourceType,
        data_sources: Vec<String>,
    },

    Inline {
        #[serde(rename = "type")]
        type_: DataSourceType,
        column_names: Vec<String>,
        rows: Vec<Vec<String>>,
    },

    Query {
        #[serde(rename = "type")]
        type_: DataSourceType,
        query: Box<NativeQuery>,
    },

    Join {
        #[serde(rename = "type")]
        type_: DataSourceType,
        left: Box<DataSource>,
        right: Box<DataSource>,
        right_prefix: String,
        condition: Expression,
        join_type: String,
    },

    Unnest {
        #[serde(rename = "type")]
        type_: DataSourceType,
        base: Box<DataSource>,
        virtual_column: VirtualColumn,
        unnest_filter: Option<String>,
    },
}

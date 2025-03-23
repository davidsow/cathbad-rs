use serde::{Deserialize, Serialize};

pub type Interval = String; // TODO
pub type Expression = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultFormat {
    List,
    CompactedList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Ascending,
    Descending,
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    Lexicographic,
    Alphanumeric,
    Strlen,
    Numeric,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OutputType {
    Long,
    Float,
    Double,
    String,
    Array,
    Complex,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderByColumnSpec {
    dimension: String,
    direction: Direction,
    dimension_order: Sort,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Bound {
    MaxTime,
    MinTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AnalysisType {
    Cardinality,
    Minmax,
    Size,
    Interval,
    TimestampSpec,
    QueryGranularity,
    Aggregators,
    Rollup,
}

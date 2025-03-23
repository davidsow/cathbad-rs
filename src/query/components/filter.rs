use crate::query::components::model::QueryComponent;
use crate::query::{Expression, ExtractionFunction, Interval, SearchQuery, Sort};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    Selector,
    ColumnComparison,
    Regex,
    And,
    Or,
    Not,
    Javascript,
    Search,
    In,
    Like,
    Bound,
    Interval,
    True,
    Expression,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Filter {
    Selector {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        value: String,
    },
    ColumnComparison {
        #[serde(rename = "type")]
        type_: FilterType,
        dimensions: Vec<String>,
    },
    Regex {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        pattern: String,
    },
    And {
        #[serde(rename = "type")]
        type_: FilterType,
        fields: Vec<Box<Filter>>,
    },
    Or {
        #[serde(rename = "type")]
        type_: FilterType,
        fields: Vec<Box<Filter>>,
    },
    Not {
        #[serde(rename = "type")]
        type_: FilterType,
        field: Box<Filter>,
    },
    Javascript {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        function: String,
    },
    #[serde(rename_all = "camelCase")]
    Search {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        query: SearchQuery,
        extraction_function: Option<ExtractionFunction>,
    },
    In {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        values: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    Like {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        pattern: String,
        escape: Option<String>,
        extraction_function: Option<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    Bound {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        lower: Option<String>,
        upper: Option<String>,
        lower_strict: Option<bool>,
        upper_strict: Option<bool>,
        ordering: Option<Sort>,
        extraction_function: Option<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    Interval {
        #[serde(rename = "type")]
        type_: FilterType,
        dimension: String,
        intervals: Vec<Interval>,
        extraction_function: Option<ExtractionFunction>,
    },
    True {
        #[serde(rename = "type")]
        type_: FilterType,
    },
    Expression {
        #[serde(rename = "type")]
        type_: FilterType,
        expression: Expression,
    },
}

impl QueryComponent for Filter {
    fn validate_type(&self) -> bool {
        match self {
            Filter::Selector { type_, .. } => *type_ == FilterType::Selector,
            Filter::ColumnComparison { type_, .. } => *type_ == FilterType::ColumnComparison,
            Filter::Regex { type_, .. } => *type_ == FilterType::Regex,
            Filter::And { type_, .. } => *type_ == FilterType::And,
            Filter::Or { type_, .. } => *type_ == FilterType::Or,
            Filter::Not { type_, .. } => *type_ == FilterType::Not,
            Filter::Javascript { type_, .. } => *type_ == FilterType::Javascript,
            Filter::Search { type_, .. } => *type_ == FilterType::Search,
            Filter::In { type_, .. } => *type_ == FilterType::In,
            Filter::Like { type_, .. } => *type_ == FilterType::Like,
            Filter::Bound { type_, .. } => *type_ == FilterType::Bound,
            Filter::Interval { type_, .. } => *type_ == FilterType::Interval,
            Filter::True { type_, .. } => *type_ == FilterType::True,
            Filter::Expression { type_, .. } => *type_ == FilterType::Expression,
        }
    }
}

impl QueryComponent for Option<Filter> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|filter| filter.validate_type())
    }
}

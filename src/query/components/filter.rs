use crate::query::components::model::QueryComponent;
use crate::query::{Expression, ExtractionFunction, Interval, SearchQuery, Sort};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Filter {
    Selector {
        dimension: String,
        value: String,
    },
    ColumnComparison {
        dimensions: Vec<String>,
    },
    Regex {
        dimension: String,
        pattern: String,
    },
    And {
        fields: Vec<Box<Filter>>,
    },
    Or {
        fields: Vec<Box<Filter>>,
    },
    Not {
        field: Box<Filter>,
    },
    Javascript {
        dimension: String,
        function: String,
    },
    #[serde(rename_all = "camelCase")]
    Search {
        dimension: String,
        query: SearchQuery,
        extraction_function: Option<ExtractionFunction>,
    },
    In {
        dimension: String,
        values: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    Like {
        dimension: String,
        pattern: String,
        escape: Option<String>,
        extraction_function: Option<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    Bound {
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
        dimension: String,
        intervals: Vec<Interval>,
        extraction_function: Option<ExtractionFunction>,
    },
    True,
    Expression {
        expression: Expression,
    },
}

impl QueryComponent for Filter {
    fn validate_type(&self) -> bool {
        match self {
            // TODO
            Filter::Selector { .. } => true,
            Filter::ColumnComparison { .. } => true,
            Filter::Regex { .. } => true,
            Filter::And { .. } => true,
            Filter::Or { .. } => true,
            Filter::Not { .. } => true,
            Filter::Javascript { .. } => true,
            Filter::Search { .. } => true,
            Filter::In { .. } => true,
            Filter::Like { .. } => true,
            Filter::Bound { .. } => true,
            Filter::Interval { .. } => true,
            Filter::True { .. } => true,
            Filter::Expression { .. } => true,
        }
    }
}

impl QueryComponent for Option<Filter> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|filter| filter.validate_type())
    }
}

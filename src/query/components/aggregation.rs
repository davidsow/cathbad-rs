use crate::query::Filter;
use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Aggregation {
    Count {
        name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleSum {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongSum {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatSum {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleMax {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongMax {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatMax {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleMin {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongMin {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatMin {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleMean {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleFirst {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongFirst {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatFirst {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleLast {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongLast {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatLast {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    DoubleAny {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    LongAny {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    FloatAny {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    StringAny {
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    JavaScript {
        name: String,
        field_names: Vec<String>,
        fn_aggregate: String,
        fn_combine: String,
        fn_reset: String,
    },

    Filtered {
        filter: Filter,
        aggregator: Box<Aggregation>,
    },

    Grouping {
        name: String,
        groupings: Vec<String>,
    },
}

impl QueryComponent for Aggregation {
    fn validate_type(&self) -> bool {
        match self {
            // TODO
            Aggregation::Count { .. } => true,
            Aggregation::DoubleSum { .. } => true,
            Aggregation::LongSum { .. } => true,
            Aggregation::FloatSum { .. } => true,
            Aggregation::DoubleMax { .. } => true,
            Aggregation::LongMax { .. } => true,
            Aggregation::FloatMax { .. } => true,
            Aggregation::DoubleMin { .. } => true,
            Aggregation::LongMin { .. } => true,
            Aggregation::FloatMin { .. } => true,
            Aggregation::DoubleMean { .. } => true,
            Aggregation::DoubleFirst { .. } => true,
            Aggregation::LongFirst { .. } => true,
            Aggregation::FloatFirst { .. } => true,
            Aggregation::DoubleLast { .. } => true,
            Aggregation::LongLast { .. } => true,
            Aggregation::FloatLast { .. } => true,
            Aggregation::DoubleAny { .. } => true,
            Aggregation::LongAny { .. } => true,
            Aggregation::FloatAny { .. } => true,
            Aggregation::StringAny { .. } => true,
            Aggregation::JavaScript { .. } => true,
            Aggregation::Filtered { .. } => true,
            Aggregation::Grouping { .. } => true,
        }
    }
}

impl QueryComponent for Option<Aggregation> {
    fn validate_type(&self) -> bool {
        self.clone()
            .is_none_or(|aggregation| aggregation.validate_type())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PostAggregationType {
    Arithmetic,
    Constant,
    JavaScript,
    HyperUniqueCardinality,

    // Combined as "FieldAccessor"
    FieldAccess,
    FinalizingFieldAccess,

    // Combined as "Bound"
    DoubleGreatest,
    LongGreatest,
    DoubleLeast,
    LongLeast,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PostAggregation {
    Arithmetic {
        name: String,
        #[serde(rename = "fn")]
        fn_: String,
        fields: Vec<String>,
        ordering: Option<String>,
    },

    FieldAccess {
        name: String,
        field_name: String,
    },

    FinalizingFieldAccess {
        name: String,
        field_name: String,
    },

    DoubleGreatest {
        name: String,
        fields: Vec<String>,
    },

    LongGreatest {
        name: String,
        fields: Vec<String>,
    },

    DoubleLeast {
        name: String,
        fields: Vec<String>,
    },

    LongLeast {
        name: String,
        fields: Vec<String>,
    },

    JavaScript {
        name: String,
        field_names: Vec<String>,
        function: String,
    },

    HyperUniqueCardinality {
        name: String,
        field_name: String,
    },
}

impl QueryComponent for PostAggregation {
    fn validate_type(&self) -> bool {
        match self {
            // TODO
            PostAggregation::Arithmetic { .. } => true,
            PostAggregation::FieldAccess { .. } => true,
            PostAggregation::FinalizingFieldAccess { .. } => true,
            PostAggregation::DoubleGreatest { .. } => true,
            PostAggregation::LongGreatest { .. } => true,
            PostAggregation::DoubleLeast { .. } => true,
            PostAggregation::LongLeast { .. } => true,
            PostAggregation::JavaScript { .. } => true,
            PostAggregation::HyperUniqueCardinality { .. } => true,
        }
    }
}

impl QueryComponent for Option<PostAggregation> {
    fn validate_type(&self) -> bool {
        self.clone()
            .is_none_or(|post_aggregation| post_aggregation.validate_type())
    }
}

impl QueryComponent for Option<Vec<PostAggregation>> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|vector| {
            for agg in vector {
                if !agg.validate_type() {
                    return false;
                }
            }
            true
        })
    }
}

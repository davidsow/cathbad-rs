use crate::query::Filter;
use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AggregationType {
    Count,
    JavaScript,
    Filtered,
    Grouping,

    // Combined as "Statistical"
    LongSum,
    DoubleSum,
    FloatSum,
    DoubleMin,
    DoubleMax,
    FloatMin,
    FloatMax,
    LongMin,
    LongMax,
    DoubleMean,
    DoubleFirst,
    DoubleLast,
    FloatFirst,
    FloatLast,
    LongFirst,
    LongLast,
    StringFirst,
    StringLast,
    DoubleAny,
    FloatAny,
    LongAny,
    StringAny,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Aggregation {
    Count {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
    },

    #[serde(rename_all = "camelCase")]
    Statistical {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
        field_name: String,
    },

    #[serde(rename_all = "camelCase")]
    JavaScript {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
        field_names: Vec<String>,
        fn_aggregate: String,
        fn_combine: String,
        fn_reset: String,
    },

    Filtered {
        #[serde(rename = "type")]
        type_: AggregationType,
        filter: Filter,
        aggregator: Box<Aggregation>,
    },

    Grouping {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
        groupings: Vec<String>,
    },
}

impl QueryComponent for Aggregation {
    fn validate_type(&self) -> bool {
        match self {
            Aggregation::Count { type_, .. } => *type_ == AggregationType::Count,
            Aggregation::Statistical { type_, .. } => {
                // TODO THERE HAS TO BE A BETTER WAY
                *type_ == AggregationType::LongSum
                    || *type_ == AggregationType::DoubleSum
                    || *type_ == AggregationType::FloatSum
                    || *type_ == AggregationType::DoubleMin
                    || *type_ == AggregationType::DoubleMax
                    || *type_ == AggregationType::FloatMin
                    || *type_ == AggregationType::FloatMax
                    || *type_ == AggregationType::LongMin
                    || *type_ == AggregationType::LongMax
                    || *type_ == AggregationType::DoubleMean
                    || *type_ == AggregationType::DoubleFirst
                    || *type_ == AggregationType::DoubleLast
                    || *type_ == AggregationType::FloatFirst
                    || *type_ == AggregationType::FloatLast
                    || *type_ == AggregationType::LongFirst
                    || *type_ == AggregationType::LongLast
                    || *type_ == AggregationType::StringFirst
                    || *type_ == AggregationType::StringLast
                    || *type_ == AggregationType::DoubleAny
                    || *type_ == AggregationType::FloatAny
                    || *type_ == AggregationType::LongAny
                    || *type_ == AggregationType::StringAny
            }
            Aggregation::JavaScript { type_, .. } => *type_ == AggregationType::JavaScript,
            Aggregation::Filtered { type_, .. } => *type_ == AggregationType::Filtered,
            Aggregation::Grouping { type_, .. } => *type_ == AggregationType::Grouping,
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
pub enum PostAggregation {
    Arithmetic {
        #[serde(rename = "type")]
        type_: PostAggregationType,
        name: String,
        fn_: String,
        fields: Vec<String>,
        ordering: Option<String>,
    },

    FieldAccessor {
        #[serde(rename = "type")]
        type_: PostAggregationType,
        name: String,
        field_name: String,
    },

    Bound {
        #[serde(rename = "type")]
        type_: PostAggregationType,
        name: String,
        fields: Vec<String>,
    },

    JavaScript {
        #[serde(rename = "type")]
        type_: PostAggregationType,
        name: String,
        field_names: Vec<String>,
        function: String,
    },

    HyperUniqueCardinality {
        #[serde(rename = "type")]
        type_: PostAggregationType,
        name: String,
        field_name: String,
    },
}

impl QueryComponent for PostAggregation {
    fn validate_type(&self) -> bool {
        match self {
            PostAggregation::Arithmetic { type_, .. } => *type_ == PostAggregationType::Arithmetic,
            PostAggregation::FieldAccessor { type_, .. } => {
                *type_ == PostAggregationType::FieldAccess
                    || *type_ == PostAggregationType::FinalizingFieldAccess
            }
            PostAggregation::Bound { type_, .. } => {
                *type_ == PostAggregationType::DoubleGreatest
                    || *type_ == PostAggregationType::LongGreatest
                    || *type_ == PostAggregationType::DoubleLeast
                    || *type_ == PostAggregationType::LongLeast
            }
            PostAggregation::JavaScript { type_, .. } => *type_ == PostAggregationType::JavaScript,
            PostAggregation::HyperUniqueCardinality { type_, .. } => {
                *type_ == PostAggregationType::HyperUniqueCardinality
            }
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

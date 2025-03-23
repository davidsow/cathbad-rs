use crate::query::Filter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

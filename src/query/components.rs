use serde::{Deserialize, Serialize};
use crate::query::model::{NativeQuery, IntegerNumber};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DataSource {
    String(String),

    Table {
        #[serde(rename = "type")]
        type_: String,
        name: String,
    },

    //Lookup {},

    Union {
        #[serde(rename = "type")]
        type_: String,
        data_sources: Vec<String>
    },

    Inline {
        #[serde(rename = "type")]
        type_: String,
        column_names: Vec<String>,
        rows: Vec<Vec<String>>
    },

    Query {
        #[serde(rename = "type")]
        type_: String,
        query: Box<NativeQuery>,
    },

    Join {
        #[serde(rename = "type")]
        type_: String,
        left: Box<DataSource>,
        right: Box<DataSource>,
        right_prefix: String,
        condition: Expression,
        join_type: String,
    },

    Unnest {
        #[serde(rename = "type")]
        type_: String,
        base: Box<DataSource>,
        virtual_column: VirtualColumn,
        unnest_filter: Option<String>
    }
}

type Expression = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualColumn {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    expression: Expression,
    output_type: OutputType
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="UPPERCASE")]
pub enum OutputType {
    Long,
    Float,
    Double,
    String,
    Array,
    Complex
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TopNMetricSpec {
    Numeric {
        #[serde(rename = "type")]
        _type: String,
        metric: String,
    },
    Dimension {
        #[serde(rename = "type")]
        type_: String,
        ordering: Option<String>,
        previous_stop: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Dimension {
    Default {
        #[serde(rename = "type")]
        type_: String,
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
    },

    #[serde(rename_all = "camelCase")]
    Extraction {
        #[serde(rename = "type")]
        type_: String,
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
        extraction_fn: ExtractionFunction,
    },

    #[serde(rename_all = "camelCase")]
    ListFiltered {
        #[serde(rename = "type")]
        type_: String,
        delegate: Box<Dimension>,
        values: Vec<String>,
        is_whitelist: Option<bool>,
    },

    RegexFiltered {
        #[serde(rename = "type")]
        type_: String,
        delegate: Box<Dimension>,
        pattern: String,
    },

    PrefixFiltered {
        #[serde(rename = "type")]
        type_: String,
        delegate: Box<Dimension>,
        prefix: String,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtractionFunctionType {
    Regex,
    Partial,
    SearchQuery,
    Substring,
    Strlen,
    TimeFormat,
    TimeParsing,
    JavaScript,
    Cascade,
    StringFormat,
    Upper,
    Lower,
    Bucket,
    // RegisteredLookup,
    // InlineLookup,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtractionFunction {
    #[serde(rename_all = "camelCase")]
    Regex {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        expr: String,
        index: IntegerNumber,
        replace_missing_value: bool,
        replace_missing_value_with: Option<String>,
    },
    Partial {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        expr: String,
    },
    SearchQuery {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        query: SearchQuery,
    },
    Substring {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        index: IntegerNumber,
        length: Option<IntegerNumber>,
    },
    Strlen {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
    },
    #[serde(rename_all = "camelCase")]
    TimeFormat {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        format: Option<String>,
        time_zone: Option<String>,
        locale: Option<String>,
        granularity: Option<Granularity>,
        as_millis: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    TimeParsing {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        time_format: String,
        result_format: String,
        joda: Option<bool>,
    },
    JavaScript {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        function: String,
        injective: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Cascade {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        extraction_fns: Vec<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    StringFormat {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        format: String,
        null_handling: Option<String>,
    },
    Upper {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        locale: Option<String>,
    },
    Lower {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        locale: Option<String>,
    },
    Bucket {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        size: IntegerNumber,
        offset: IntegerNumber,
    },
    //
    // I am consciously deciding not to support Lookup operations at this time
    //
    // #[serde(rename_all = "camelCase")]
    // RegisteredLookup {
    //     #[serde(rename = "type")]
    //     type_: ExtractionFunctionType,
    //     lookup: String,
    //     retain_missing_value: Option<bool>,
    //     replace_missing_value_with: Option<String>,
    // },
    // #[serde(rename_all = "camelCase")]
    // InlineLookup {
    //     #[serde(rename = "type")]
    //     type_: ExtractionFunctionType,
    //     lookup: Lookup,
    //     retain_missing_value: Option<bool>,
    //     replace_missing_value_with: Option<String>,
    //     injective: Option<bool>,
    // },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Limit {
    //TODO
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Having {
    //TODO
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum FilterType {
    Selector,
    ColumnComparison,
    Regex,
    Javascript,
    Search,
    Contains,
    InsensitiveContains,
    Fragment,
    In,
    Like,
    Bound,
    Interval,
    True,
    Expression,

    // Combined as "Logical"
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Filter {
    Selector {
        type_: FilterType,
        dimension: String,
        value: String,
    },
    ColumnComparison {
        type_: FilterType,
        dimensions: Vec<String>,
    },
    Regex {
        type_: FilterType,
        dimension: String,
        pattern: String,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
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

    #[serde(rename_all="camelCase")]
    Statistical {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
        field_name: String,
    },

    #[serde(rename_all="camelCase")]
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
        aggregator: Box<Aggregation>
    },

    Grouping {
        #[serde(rename = "type")]
        type_: AggregationType,
        name: String,
        groupings: Vec<String>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
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
        ordering: Option<String>
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="snake_case")] // ???
pub enum Granularity {
    All,
    None,
    Second,
    Minute,
    FifteenMinute,
    ThirtyMinute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum Bound {
    MaxTime,
    MinTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GranularitySpec {
    #[serde(rename = "type")]
    type_: String,
    segment_granularity: Granularity,
    query_granularity: Granularity,
    rollup: Option<bool>,
    intervals: Option<Vec<String>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SearchQuery {
    InsensitiveContains {
        #[serde(rename = "type")]
        type_: String,
        value: String,
    },
    Fragment {
        #[serde(rename = "type")]
        type_: String,
        case_sensitive: bool,
        values: Vec<String>
    },
    Contains {
        #[serde(rename = "type")]
        type_: String,
        case_sensitive: bool,
        value: String
    },
    Regex {
        #[serde(rename = "type")]
        type_: String,
        pattern: String
    }
}

// This is an insane amount of work to support for right now
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Lookup {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum ToIncludeType {
    All,
    None,
    List,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ToInclude {
    All {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    None {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    List {
        #[serde(rename = "type")]
        type_: ToIncludeType,
        columns: Vec<String>,
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
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


pub type Interval = String; // TODO

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum ResultFormat {
    List,
    CompactedList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum Order {
    Ascending,
    Descending,
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum Sort {
    Lexicographic,
    Alphanumeric,
    Strlen,
    Numeric,
}

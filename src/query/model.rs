use crate::query::components::*;
use serde::{Deserialize, Serialize};

// Query definitions in this file apply to Druid v0.22.1
// There may or may not be breaking API changes between this version and latest
// https://druid.apache.org/docs/0.22.1/querying/querying.html

// Int and Float types can probably be smaller, but I don't know for sure yet
pub type IntegerNumber = u64;
pub type FloatingPointNumber = f64;

pub trait TypeConstrainedQuery {
    fn validate_type(&self) -> bool;
    fn validate_subcomponents(&self) -> bool;
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NativeQueryType {
    Timeseries,
    TopN,
    GroupBy,
    TimeBoundary,
    SegmentMetadata,
    DatasourceMetadata,
    Scan,
    Search,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NativeQuery {
    #[serde(rename_all = "camelCase")]
    Timeseries {
        query_type: NativeQueryType,
        data_source: DataSource,
        descending: Option<bool>,
        intervals: Vec<Interval>,
        granularity: Granularity,
        filter: Option<Filter>,
        aggregations: Option<Aggregation>,
        post_aggregations: Option<Vec<PostAggregation>>,
        limit: Option<IntegerNumber>,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    TopN {
        query_type: NativeQueryType,
        data_source: DataSource,
        intervals: Vec<Interval>,
        granularity: Granularity,
        filter: Option<Filter>,
        aggregations: Option<Aggregation>,
        post_aggregations: Option<Vec<PostAggregation>>,
        dimension: DimensionSpec,
        threshold: IntegerNumber,
        metric: TopNMetricSpec,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    GroupBy {
        query_type: NativeQueryType,
        data_source: DataSource,
        dimensions: Vec<DimensionSpec>,
        limit_spec: Option<LimitSpec>,
        having: Option<Having>,
        granularity: Granularity,
        filter: Option<Filter>,
        aggregations: Option<Aggregation>,
        post_aggregations: Option<Vec<PostAggregation>>,
        intervals: Vec<String>,
        subtotals_spec: Option<Vec<Vec<String>>>, // DESGUSTANG, but that's the spec
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    TimeBoundary {
        query_type: NativeQueryType,
        data_source: DataSource,
        bound: Option<Bound>,
        filter: Option<Filter>,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    SegmentMetadata {
        query_type: NativeQueryType,
        data_source: DataSource,
        intervals: Option<Vec<Interval>>,
        to_include: Option<Vec<ToInclude>>,
        merge: Option<bool>,
        context: Option<Context>,
        analysis_types: Vec<AnalysisType>,
        lenient_aggregator_merge: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    DatasourceMetadata {
        query_type: NativeQueryType,
        data_source: DataSource,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    Scan {
        query_type: NativeQueryType,
        data_source: DataSource,
        intervals: Vec<Interval>,
        result_format: Option<ResultFormat>,
        offset: Option<IntegerNumber>,
        order: Option<Order>,
        legacy: Option<bool>,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    Search {
        query_type: NativeQueryType,
        data_source: DataSource,
        granularity: Option<Granularity>,
        filter: Option<Filter>,
        limit: Option<IntegerNumber>,
        intervals: Vec<Interval>,
        search_dimensions: Option<Vec<String>>,
        query: SearchQuery,
        sort: Option<Sort>,
        context: Option<Context>,
    },
}

impl TypeConstrainedQuery for NativeQuery {
    fn validate_type(&self) -> bool {
        match self {
            NativeQuery::Timeseries { query_type,  .. } => {
                *query_type == NativeQueryType::Timeseries
            }
            NativeQuery::TopN { query_type,  .. } => {
                *query_type == NativeQueryType::TopN
            }
            NativeQuery::GroupBy { query_type,  .. } => {
                *query_type == NativeQueryType::GroupBy
            }
            NativeQuery::TimeBoundary { query_type,  .. } => {
                *query_type == NativeQueryType::TimeBoundary
            }
            NativeQuery::SegmentMetadata { query_type,  .. } => {
                *query_type == NativeQueryType::SegmentMetadata
            }
            NativeQuery::DatasourceMetadata { query_type,  .. } => {
                *query_type == NativeQueryType::DatasourceMetadata
            }
            NativeQuery::Scan { query_type,  .. } => {
                *query_type == NativeQueryType::Scan
            }
            NativeQuery::Search { query_type,  .. } => {
                *query_type == NativeQueryType::Search
            }
        }
    }
    fn validate_subcomponents(&self) -> bool {
        match self {
            NativeQuery::Timeseries { data_source, filter, aggregations, post_aggregations,  .. } => {
                data_source.validate_type() &&
                    filter.validate_type() &&
                    aggregations.validate_type() &&
                    post_aggregations.validate_type()
            }
            NativeQuery::TopN { data_source, filter, aggregations, post_aggregations, dimension, metric, .. } => {
                data_source.validate_type() &&
                    filter.validate_type() &&
                    aggregations.validate_type() &&
                    post_aggregations.validate_type() &&
                    dimension.validate_type() &&
                    metric.validate_type()
            }
            NativeQuery::GroupBy { data_source, dimensions, limit_spec, having, filter, aggregations, post_aggregations, .. } => {
                data_source.validate_type() &&
                    dimensions.validate_type() &&
                    limit_spec.validate_type() &&
                    having.validate_type() &&
                    filter.validate_type() &&
                    aggregations.validate_type() &&
                    post_aggregations.validate_type()
            }
            NativeQuery::TimeBoundary { data_source, filter, .. } => {
                data_source.validate_type() &&
                    filter.validate_type()
            }
            NativeQuery::SegmentMetadata { data_source, to_include, .. } => {
                data_source.validate_type() &&
                    to_include.validate_type()
            }
            NativeQuery::DatasourceMetadata { data_source, .. } => {
                data_source.validate_type()
            }
            NativeQuery::Scan { data_source, .. } => {
                data_source.validate_type()
            }
            NativeQuery::Search { data_source, filter, query, .. } => {
                data_source.validate_type() &&
                    filter.validate_type() &&
                    query.validate_type()
            }
        }
    }
}


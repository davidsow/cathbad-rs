use crate::query::components::*;
use serde::{Deserialize, Serialize};

// Query definitions in this file apply to Druid v0.22.1
// There may or may not be breaking API changes between this version and latest
// https://druid.apache.org/docs/0.22.1/querying/querying.html

// Int and Float types can probably be smaller, but I don't know for sure yet
pub type IntegerNumber = u64;
pub type FloatingPointNumber = f64;

pub trait TypeConstrainedQuery: Serialize + for<'a> Deserialize<'a> {
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
#[serde(tag = "queryType", rename_all = "camelCase")]
pub enum NativeQuery {
    #[serde(rename_all = "camelCase")]
    Timeseries {
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
        data_source: DataSource,
        bound: Option<Bound>,
        filter: Option<Filter>,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    SegmentMetadata {
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
        data_source: DataSource,
        context: Option<Context>,
    },

    #[serde(rename_all = "camelCase")]
    Scan {
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
            // TODO
            NativeQuery::Timeseries { .. } => true,
            NativeQuery::TopN { .. } => true,
            NativeQuery::GroupBy { .. } => true,
            NativeQuery::TimeBoundary { .. } => true,
            NativeQuery::SegmentMetadata { .. } => true,
            NativeQuery::DatasourceMetadata { .. } => true,
            NativeQuery::Scan { .. } => true,
            NativeQuery::Search { .. } => true,
        }
    }
    fn validate_subcomponents(&self) -> bool {
        match self {
            NativeQuery::Timeseries {
                data_source,
                filter,
                aggregations,
                post_aggregations,
                ..
            } => {
                data_source.validate_type()
                    && filter.validate_type()
                    && aggregations.validate_type()
                    && post_aggregations.validate_type()
            }
            NativeQuery::TopN {
                data_source,
                filter,
                aggregations,
                post_aggregations,
                dimension,
                metric,
                ..
            } => {
                data_source.validate_type()
                    && filter.validate_type()
                    && aggregations.validate_type()
                    && post_aggregations.validate_type()
                    && dimension.validate_type()
                    && metric.validate_type()
            }
            NativeQuery::GroupBy {
                data_source,
                dimensions,
                limit_spec,
                having,
                filter,
                aggregations,
                post_aggregations,
                ..
            } => {
                data_source.validate_type()
                    && dimensions.validate_type()
                    && limit_spec.validate_type()
                    && having.validate_type()
                    && filter.validate_type()
                    && aggregations.validate_type()
                    && post_aggregations.validate_type()
            }
            NativeQuery::TimeBoundary {
                data_source,
                filter,
                ..
            } => data_source.validate_type() && filter.validate_type(),
            NativeQuery::SegmentMetadata {
                data_source,
                to_include,
                ..
            } => data_source.validate_type() && to_include.validate_type(),
            NativeQuery::DatasourceMetadata { data_source, .. } => data_source.validate_type(),
            NativeQuery::Scan { data_source, .. } => data_source.validate_type(),
            NativeQuery::Search {
                data_source,
                filter,
                query,
                ..
            } => data_source.validate_type() && filter.validate_type() && query.validate_type(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_serde() {
        let query_basic = NativeQuery::Search {
            data_source: DataSource::String("example".to_string()),
            granularity: None,
            filter: None,
            limit: None,
            intervals: vec![],
            search_dimensions: None,
            query: SearchQuery::InsensitiveContains {
                value: "DUMMY_VALUE".to_string(),
            },
            sort: None,
            context: None,
        };

        assert!(query_basic.validate_type());
        assert!(query_basic.validate_subcomponents());

        let payload = serde_json::to_string_pretty(&query_basic).unwrap();

        println!("{}", payload);

        let roundabout: NativeQuery = serde_json::from_str(&payload).unwrap();

        println!("{:?}", roundabout);
    }
}

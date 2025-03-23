use crate::query::components::*;
use serde::{Deserialize, Serialize};

// Query definitions in this file apply to Druid v0.22.1
// There may or may not be breaking API changes between this version and latest
// https://druid.apache.org/docs/0.22.1/querying/querying.html

// Int and Float types can probably be smaller, but I don't know for sure yet
pub type IntegerNumber = u64;
pub type FloatingPointNumber = f64;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
        data_source: String,
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

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    timeout: Option<IntegerNumber>,
    priority: Option<IntegerNumber>,
    lane: Option<IntegerNumber>,
    query_id: Option<String>,
    broker_service: Option<String>,
    use_cache: Option<bool>,
    populate_cache: Option<bool>,
    use_result_level_cache: Option<bool>,
    populate_result_level_cache: Option<bool>,
    by_segment: Option<bool>,
    finalize: Option<bool>,
    max_scatter_gather_bytes: Option<IntegerNumber>,
    max_queued_bytes: Option<IntegerNumber>,
    serialize_date_time_as_long: Option<bool>,
    serialize_date_time_as_long_inner: Option<bool>,
    enable_parallel_merge: Option<bool>,
    parallel_merge_parallelism: Option<IntegerNumber>,
    parallel_merge_initial_yield_rows: Option<IntegerNumber>,
    parallel_merge_small_batch_rows: Option<IntegerNumber>,
    #[serde(rename = "useFilterCNF")]
    use_filter_cnf: Option<bool>,
    secondary_partition_pruning: Option<bool>,
    enable_join_left_table_scan_direct: Option<bool>,
    debug: Option<bool>,

    // Query-Specific Parameters

    // TopN
    min_top_n_threshold: Option<IntegerNumber>,

    // Timeseries
    skip_empty_buckets: Option<bool>,

    // GroupBy
    // global
    group_by_strategy: Option<String>,
    group_by_is_single_thread: Option<bool>,
    // v2
    buffer_grouper_initial_buckets: Option<IntegerNumber>,
    buffer_grouper_max_load_factor: Option<FloatingPointNumber>,
    force_hash_aggregation: Option<bool>,
    intermediate_combine_degree: Option<IntegerNumber>,
    num_parallel_combine_threads: Option<IntegerNumber>,
    apply_limit_push_down_to_segment: Option<bool>,
    sort_by_dims_first: Option<bool>,
    force_limit_push_down: Option<bool>,
    // v1
    max_intermediate_rows: Option<IntegerNumber>,
    max_results: Option<IntegerNumber>,
    use_of_heap: Option<bool>,

    // Timeseries + GroupBy
    vectorize: Option<bool>,
    vector_size: Option<IntegerNumber>,
    vectorize_virtual_columns: Option<bool>,
}

use crate::query::{FloatingPointNumber, IntegerNumber};
use serde::{Deserialize, Serialize};

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

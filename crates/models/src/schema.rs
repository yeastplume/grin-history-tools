table! {
    headers (hash) {
        hash -> Bytea,
        version -> Int2,
        height -> Numeric,
        previous -> Bytea,
        prev_root -> Bytea,
        timestamp_utc -> Timestamp,
        output_root -> Bytea,
        output_mmr_size -> Numeric,
        range_proof_root -> Bytea,
        kernel_root -> Bytea,
        kernel_mmr_size -> Numeric,
        nonce -> Numeric,
        edge_bits -> Int2,
        cuckoo_solution -> Bytea,
        total_difficulty -> Numeric,
        secondary_scaling -> Int8,
        total_kernel_offset -> Bytea,
    }
}

table! {
    kernel_features (enum_id) {
        enum_id -> Int2,
        description -> Varchar,
    }
}

table! {
    kernels (header_hash, excess) {
        header_hash -> Bytea,
        excess -> Bytea,
        excess_sig -> Bytea,
        features -> Int2,
        fee -> Numeric,
        fee_shift -> Int2,
        lock_height -> Nullable<Numeric>,
        relative_height -> Nullable<Int8>,
    }
}

table! {
    output_features (enum_id) {
        enum_id -> Int2,
        description -> Varchar,
    }
}

table! {
    outputs (output_header_hash, commit) {
        output_header_hash -> Bytea,
        input_header_hash -> Nullable<Bytea>,
        commit -> Bytea,
        output_type -> Nullable<Int2>,
        proof -> Bytea,
    }
}

joinable!(kernels -> headers (header_hash));
joinable!(kernels -> kernel_features (features));
joinable!(outputs -> output_features (output_type));

allow_tables_to_appear_in_same_query!(
    headers,
    kernel_features,
    kernels,
    output_features,
    outputs,
);

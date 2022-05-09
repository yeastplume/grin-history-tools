table! {
    headers (id) {
        id -> Int4,
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
    inputs (id) {
        id -> Int4,
        header_id -> Int4,
        output_id -> Int4,
    }
}

table! {
    kernel_features (enum_id) {
        enum_id -> Int2,
        description -> Varchar,
    }
}

table! {
    kernels (id) {
        id -> Int4,
        header_id -> Int4,
        excess -> Bytea,
        excess_sig -> Bytea,
        features -> Int2,
        fee -> Numeric,
        fee_shift -> Int2,
        lock_height -> Numeric,
    }
}

table! {
    output_types (enum_id) {
        enum_id -> Int2,
        description -> Varchar,
    }
}

table! {
    outputs (id) {
        id -> Int4,
        header_id -> Int4,
        commit -> Bytea,
        output_type -> Int2,
        proof -> Bytea,
    }
}

joinable!(inputs -> headers (header_id));
joinable!(inputs -> outputs (output_id));
joinable!(kernels -> headers (header_id));
joinable!(kernels -> kernel_features (features));
joinable!(outputs -> headers (header_id));
joinable!(outputs -> output_types (output_type));

allow_tables_to_appear_in_same_query!(
    headers,
    inputs,
    kernel_features,
    kernels,
    output_types,
    outputs,
);

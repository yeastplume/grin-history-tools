table! {
    headers (chain_type, height, hash) {
        chain_type -> Int2,
        fork -> Int2,
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

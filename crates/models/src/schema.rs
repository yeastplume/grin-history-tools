table! {
    headers (chain_type, height, hash) {
        chain_type -> Int2,
        height -> Numeric,
        version -> Int2,
        fork -> Int2,
        hash -> Bytea,
        kernel_mmr_size -> Numeric,
        kernel_root -> Bytea,
        output_mmr_size -> Numeric,
        output_root -> Bytea,
        prev_root -> Bytea,
        previous -> Bytea,
        range_proof_root -> Bytea,
        total_kernel_offset -> Bytea,
        timestamp_utc -> Timestamp,
        edge_bits -> Int2,
        total_difficulty -> Numeric,
        nonce -> Numeric,
        secondary_scaling -> Int8,
        cuckoo_solution -> Array<Bytea>,
    }
}

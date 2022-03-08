create TABLE headers (
    chain_type SMALLINT NOT NULL,    

    height NUMERIC(20) NOT NULL,
    version SMALLINT NOT NULL, 
    fork SMALLINT NOT NULL,

    hash BYTEA NOT NULL,
    kernel_mmr_size NUMERIC(20) NOT NULL,
    kernel_root BYTEA NOT NULL,
    output_mmr_size NUMERIC(20) NOT NULL,
    output_root BYTEA NOT NULL,
    prev_root BYTEA NOT NULL,
    previous BYTEA NOT NULL,
    range_proof_root BYTEA NOT NULL,
    total_kernel_offset BYTEA NOT NULL,
    timestamp_utc TIMESTAMP NOT NULL,

    edge_bits SMALLINT NOT NULL,
    total_difficulty NUMERIC(20) NOT NULL,
    nonce NUMERIC(20) NOT NULL,
    secondary_scaling BIGINT NOT NULL,
    cuckoo_solution BYTEA[42] NOT NULL,
    PRIMARY KEY(chain_type, height, hash)
);

CREATE INDEX headers__height_idx on headers(height);
CREATE INDEX headers__hash_idx on headers(hash);

-- Block Headers

create TABLE headers (
    hash BYTEA PRIMARY KEY,
    version SMALLINT NOT NULL, 
    height NUMERIC(20) NOT NULL,
    previous BYTEA NOT NULL,
    prev_root BYTEA NOT NULL,
    timestamp_utc TIMESTAMP NOT NULL,
    output_root BYTEA NOT NULL,
    output_mmr_size NUMERIC(20) NOT NULL,
    range_proof_root BYTEA NOT NULL,
    kernel_root BYTEA NOT NULL,
    kernel_mmr_size NUMERIC(20) NOT NULL,
    nonce NUMERIC(20) NOT NULL,
    edge_bits SMALLINT NOT NULL,
    cuckoo_solution BYTEA NOT NULL,
    total_difficulty NUMERIC(20) NOT NULL,
    secondary_scaling BIGINT NOT NULL, -- u32
    total_kernel_offset BYTEA NOT NULL
);

CREATE INDEX headers__height_idx on headers(height);

-- Kernel Features
create TABLE kernel_features(
    enum_id SMALLINT PRIMARY KEY,
    description VARCHAR NOT NULL
);

insert into kernel_features (enum_id, description) VALUES 
    (0, 'Plain'),
    (1, 'Coinbase'),
    (2, 'HeightLocked'),
    (3, 'NoRecentDuplicate');

-- Kernels
create TABLE kernels (
    header_hash BYTEA references headers(hash) NOT NULL,
    excess BYTEA NOT NULL,
    excess_sig BYTEA NOT NULL,
    features SMALLINT references kernel_features(enum_id) NOT NULL,
    fee NUMERIC(20) NOT NULL,
    fee_shift SMALLINT NOT NULL, -- TODO: is this ever not 0 at the moment?
    lock_height NUMERIC(20), -- Nullable in V2 protocol
    relative_height BIGINT, -- u32, nullable in V2 protocol
    PRIMARY KEY(header_hash, excess)
);

-- Output Features
create TABLE output_features(
    enum_id SMALLINT PRIMARY KEY,
    description VARCHAR NOT NULL
);

insert into output_features (enum_id, description) VALUES 
    (0, 'Plain'),
    (1, 'Coinbase');
 
-- Outputs
-- Leave out merkle proof and MMR indices for now
-- 'proof_hash' can be derived from proof
-- 'spent' can be derived from whether input_header_hash exists
create TABLE outputs (
    output_header_hash BYTEA references headers(hash) NOT NULL, -- i.e, block in which this output was created
    input_header_hash BYTEA references headers(hash), -- i.e. block in which this output was spent
    commit BYTEA NOT NULL,
    output_type SMALLINT references output_features(enum_id),
    proof BYTEA NOT NULL,
    PRIMARY KEY(output_header_hash, commit)
);
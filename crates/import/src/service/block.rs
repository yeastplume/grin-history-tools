use diesel::prelude::*;
use grin_history_tools_models::block::model::{BlockHeaderDb, KernelDb, OutputDb, InputDb};
use grin_history_tools_models::block::{BlockHeaderPrintable, BlockPrintable, TxKernelPrintable, OutputPrintable};
use grin_history_tools_models::schema;
use grin_util::from_hex;

use std::convert::TryFrom;
use crate::DbError;

use crate::database::PooledConnection;

pub fn add_block(db: &PooledConnection, block_pr: BlockPrintable) -> Result<(), DbError> {
    db.transaction::<(), DbError, _>(|| {
        // Add header
        let header_db = create_header(db, block_pr.header)?;
        // Add kernels
        for k in block_pr.kernels {
            create_kernel(db, header_db.hash.clone(), k)?;
        }
        for o in block_pr.outputs {
            create_output(db, header_db.hash.clone(), o)?;
        }
        for i in block_pr.inputs {
            create_input(db, header_db.hash.clone(), &i)?;
        }
         Ok(())
    })
}

pub fn create_kernel(
    db: &PooledConnection,
    header_hash: Vec<u8>,
    kp: TxKernelPrintable,
) -> Result<KernelDb, DbError> {
    use schema::kernels::dsl::kernels;

    db.transaction::<KernelDb, DbError, _>(|| {
        let mut k_db = KernelDb::try_from(kp)?;
        k_db.header_hash = header_hash;
        let inserted_kernel: KernelDb = diesel::insert_into(kernels)
            .values(k_db)
            .get_result(db)?;

        Ok(inserted_kernel)
    })
}

pub fn create_output(
    db: &PooledConnection,
    header_hash: Vec<u8>,
    op: OutputPrintable,
) -> Result<OutputDb, DbError> {
    use schema::outputs::dsl::outputs;

    db.transaction::<OutputDb, DbError, _>(|| {
        let mut o_db = OutputDb::try_from(op)?;
        o_db.header_hash = header_hash;
        let inserted_output: OutputDb = diesel::insert_into(outputs)
            .values(o_db)
            .get_result(db)?;

        Ok(inserted_output)
    })
}

pub fn create_input(
    db: &PooledConnection,
    header_hash: Vec<u8>,
    commit_str: &str,
) -> Result<InputDb, DbError> {
    use schema::outputs::dsl::{outputs, commit as commit_dsl};
    use schema::inputs::dsl::inputs;

    let commit_bin = from_hex(commit_str)?;

    // Retrieve the output from the DB (Must exist for the time being)
    let output_db:OutputDb = outputs
        .filter(commit_dsl.eq(commit_bin.clone()))
        .first(db)?;

    db.transaction::<InputDb, DbError, _>(|| {
        let input_db = InputDb {
            header_hash: header_hash,
            output_header_hash: output_db.header_hash,
            commit: commit_bin 
        };
        let inserted_input: InputDb = diesel::insert_into(inputs)
            .values(input_db)
            .get_result(db)?;

        Ok(inserted_input)
    })
}


pub fn create_header(
    db: &PooledConnection,
    hp: BlockHeaderPrintable,
) -> Result<BlockHeaderDb, DbError> {
    use schema::headers::dsl::headers;

    db.transaction::<BlockHeaderDb, DbError, _>(|| {
        let inserted_header: BlockHeaderDb = diesel::insert_into(headers)
            .values(BlockHeaderDb::try_from(hp)?)
            .get_result(db)?;

        Ok(inserted_header)
    })
}


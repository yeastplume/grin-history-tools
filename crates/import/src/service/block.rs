use diesel::prelude::*;
use grin_history_tools_models::block::model::{
    BlockHeaderDbInsertable, BlockHeaderDbQueryable, InputDbInsertable, InputDbQueryable,
    KernelDbInsertable, KernelDbQueryable, OutputDbInsertable, OutputDbQueryable,
};
use grin_history_tools_models::block::{
    BlockHeaderPrintable, BlockPrintable, OutputPrintable, TxKernelPrintable,
};
use grin_history_tools_models::schema;
use grin_util::from_hex;

use crate::DbError;
use std::convert::TryFrom;

use crate::database::PooledConnection;

pub fn add_block(db: &PooledConnection, block_pr: BlockPrintable) -> Result<(), DbError> {
    db.transaction::<(), DbError, _>(|| {
        // Add header
        let header_db = create_header(db, block_pr.header)?;
        // Add kernels
        for k in block_pr.kernels {
            create_kernel(db, header_db.id, k)?;
        }
        for o in block_pr.outputs {
            create_output(db, header_db.id, o)?;
        }
        for i in block_pr.inputs {
            create_input(db, header_db.id, &i)?;
        }
        Ok(())
    })
}

pub fn create_kernel(
    db: &PooledConnection,
    header_id: i32,
    kp: TxKernelPrintable,
) -> Result<KernelDbQueryable, DbError> {
    use schema::kernels::dsl::kernels;

    db.transaction::<KernelDbQueryable, DbError, _>(|| {
        let mut k_db = KernelDbInsertable::try_from(kp)?;
        k_db.header_id = header_id;
        let inserted_kernel: KernelDbQueryable =
            diesel::insert_into(kernels).values(k_db).get_result(db)?;

        Ok(inserted_kernel)
    })
}

pub fn create_output(
    db: &PooledConnection,
    header_id: i32,
    op: OutputPrintable,
) -> Result<OutputDbQueryable, DbError> {
    use schema::outputs::dsl::outputs;

    db.transaction::<OutputDbQueryable, DbError, _>(|| {
        let mut o_db = OutputDbInsertable::try_from(op)?;
        o_db.header_id = header_id;
        let inserted_output: OutputDbQueryable =
            diesel::insert_into(outputs).values(o_db).get_result(db)?;

        Ok(inserted_output)
    })
}

pub fn create_input(
    db: &PooledConnection,
    header_id: i32,
    commit_str: &str,
) -> Result<InputDbQueryable, DbError> {
    use schema::inputs::dsl::inputs;
    use schema::outputs::dsl::{commit as commit_dsl, outputs};

    let commit_bin = from_hex(commit_str)?;

    // Retrieve the output from the DB (Must exist for the time being)
    let output_db: OutputDbQueryable = outputs
        .filter(commit_dsl.eq(commit_bin.clone()))
        .first(db)?;

    db.transaction::<InputDbQueryable, DbError, _>(|| {
        let input_db = InputDbInsertable {
            header_id: header_id,
            output_id: output_db.id,
        };
        let inserted_input: InputDbQueryable = diesel::insert_into(inputs)
            .values(input_db)
            .get_result(db)?;

        Ok(inserted_input)
    })
}

pub fn create_header(
    db: &PooledConnection,
    hp: BlockHeaderPrintable,
) -> Result<BlockHeaderDbQueryable, DbError> {
    use schema::headers::dsl::headers;

    db.transaction::<BlockHeaderDbQueryable, DbError, _>(|| {
        let inserted_header: BlockHeaderDbQueryable = diesel::insert_into(headers)
            .values(BlockHeaderDbInsertable::try_from(hp)?)
            .get_result(db)?;

        Ok(inserted_header)
    })
}

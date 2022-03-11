use crate::schema::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::*;
use grin_core::pow::Proof;
use grin_core::ser::{BinReader, Readable, ProtocolVersion, DeserializationMode};
use grin_util::{from_hex, ToHex};
use thiserror::Error;

use std::convert::TryFrom;
use std::io::Cursor;

use crate::block::BlockHeaderPrintable;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "headers"]
pub struct BlockHeaderDb {
    pub chain_type: i16,
    pub fork: i16,
    pub hash: Vec<u8>,
    pub version: i16,
    pub height: BigDecimal,
    pub previous: Vec<u8>,
    pub prev_root: Vec<u8>,
    pub timestamp_utc: NaiveDateTime,
    pub output_root: Vec<u8>,
    pub output_mmr_size: BigDecimal,
    pub range_proof_root: Vec<u8>,
    pub kernel_root: Vec<u8>,
    pub kernel_mmr_size: BigDecimal,
    pub nonce: BigDecimal,
    pub edge_bits: i16,
    pub cuckoo_solution: Vec<u8>,
    pub total_difficulty: BigDecimal,
    pub secondary_scaling: i64,
    pub total_kernel_offset: Vec<u8>,
}

impl TryFrom<BlockHeaderPrintable> for BlockHeaderDb {
    type Error = ModelError;

    fn try_from(hp: BlockHeaderPrintable) -> Result<Self, Self::Error> {
        Ok(BlockHeaderDb {
            chain_type: 0,
            fork: 0,
            hash: from_hex(&hp.hash)?,
            version: hp.version as i16,
            height: BigDecimal::from(hp.height),
            previous: from_hex(&hp.previous)?,
            prev_root: from_hex(&hp.prev_root)?,
            timestamp_utc: DateTime::parse_from_rfc3339(&hp.timestamp)?.naive_utc(),
            output_root: from_hex(&hp.output_root)?,
            output_mmr_size: BigDecimal::from(hp.output_mmr_size),
            range_proof_root: from_hex(&hp.range_proof_root)?,
            kernel_root: from_hex(&hp.kernel_root)?,
            kernel_mmr_size: BigDecimal::from(hp.kernel_mmr_size),
            nonce: BigDecimal::from(hp.nonce),
            edge_bits: hp.edge_bits as i16,
            cuckoo_solution: Proof {
                edge_bits: hp.edge_bits,
                nonces: hp.cuckoo_solution,
            }
            .pack_nonces(),
            total_difficulty: BigDecimal::from(hp.total_difficulty),
            secondary_scaling: hp.secondary_scaling as i64,
            total_kernel_offset: from_hex(&hp.total_kernel_offset)?,
        })
    }
}

impl TryFrom<BlockHeaderDb> for BlockHeaderPrintable {
    type Error = ModelError;

    fn try_from(h_db: BlockHeaderDb) -> Result<Self, Self::Error> {
        // Parse cuckoo solution
        let mut buf = Cursor::new(h_db.cuckoo_solution);
		let mut r = BinReader::new(
			&mut buf,
			ProtocolVersion::local(),
			DeserializationMode::default(),
		);
		let cuckoo_solution = match Proof::read(&mut r) {
			Err(e) => return Err(ModelError::ConversionError(
                format!("cannot read proof from db: {}", e),
            )),
			Ok(p) => p.nonces
		};
        Ok(BlockHeaderPrintable {
            hash: h_db.hash.to_hex(),
            version: h_db.version as u16,
            height: h_db.height.to_u64().ok_or(ModelError::ConversionError(
                "height from DB exceeds u64".to_string(),
            ))?,
            previous: h_db.previous.to_hex(),
            prev_root: h_db.prev_root.to_hex(),
            timestamp: DateTime::<chrono::Utc>::from_utc(h_db.timestamp_utc, chrono::Utc).to_rfc3339(),
            output_root: h_db.output_root.to_hex(),
            output_mmr_size: h_db.output_mmr_size.to_u64().ok_or(ModelError::ConversionError(
                "output_mmr_size from DB exceeds u64".to_string(),
            ))?,
            range_proof_root: h_db.range_proof_root.to_hex(),
            kernel_root: h_db.kernel_root.to_hex(),
            kernel_mmr_size: h_db.kernel_mmr_size.to_u64().ok_or(ModelError::ConversionError(
                "kernel_mmr_size from DB exceeds u64".to_string(),
            ))?,
            nonce: h_db.nonce.to_u64().ok_or(ModelError::ConversionError(
                "nonce from DB exceeds u64".to_string(),
            ))?,
            edge_bits: h_db.edge_bits as u8,
            cuckoo_solution,
            total_difficulty: h_db.total_difficulty.to_u64().ok_or(ModelError::ConversionError(
                "total difficulty from DB exceeds u64".to_string(),
            ))?,
            secondary_scaling: h_db.secondary_scaling as u32,
            total_kernel_offset: h_db.total_kernel_offset.to_hex(),
        })
    }
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model Conversion Error: {0}")]
    ConversionError(String),
}

impl From<String> for ModelError {
    fn from(e: String) -> Self {
        ModelError::ConversionError(e.into())
    }
}

impl From<chrono::ParseError> for ModelError {
    fn from(e: chrono::ParseError) -> Self {
        ModelError::ConversionError(format!("{}", e))
    }
}
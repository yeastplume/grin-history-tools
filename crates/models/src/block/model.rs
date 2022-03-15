use crate::schema::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::*;
use grin_core::pow::Proof;
use grin_core::ser::{BinReader, DeserializationMode, ProtocolVersion, Readable};
use grin_util::{from_hex, ToHex};
use thiserror::Error;

use std::convert::TryFrom;
use std::io::Cursor;

use crate::block::{BlockHeaderPrintable, OutputPrintable, OutputType, TxKernelPrintable};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "headers"]
pub struct BlockHeaderDb {
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
            Err(e) => {
                return Err(ModelError::ConversionError(format!(
                    "cannot read proof from db: {}",
                    e
                )))
            }
            Ok(p) => p.nonces,
        };
        Ok(BlockHeaderPrintable {
            hash: h_db.hash.to_hex(),
            version: h_db.version as u16,
            height: h_db.height.to_u64().ok_or(ModelError::ConversionError(
                "height from DB exceeds u64".to_string(),
            ))?,
            previous: h_db.previous.to_hex(),
            prev_root: h_db.prev_root.to_hex(),
            timestamp: DateTime::<chrono::Utc>::from_utc(h_db.timestamp_utc, chrono::Utc)
                .to_rfc3339(),
            output_root: h_db.output_root.to_hex(),
            output_mmr_size: h_db
                .output_mmr_size
                .to_u64()
                .ok_or(ModelError::ConversionError(
                    "output_mmr_size from DB exceeds u64".to_string(),
                ))?,
            range_proof_root: h_db.range_proof_root.to_hex(),
            kernel_root: h_db.kernel_root.to_hex(),
            kernel_mmr_size: h_db
                .kernel_mmr_size
                .to_u64()
                .ok_or(ModelError::ConversionError(
                    "kernel_mmr_size from DB exceeds u64".to_string(),
                ))?,
            nonce: h_db.nonce.to_u64().ok_or(ModelError::ConversionError(
                "nonce from DB exceeds u64".to_string(),
            ))?,
            edge_bits: h_db.edge_bits as u8,
            cuckoo_solution,
            total_difficulty: h_db
                .total_difficulty
                .to_u64()
                .ok_or(ModelError::ConversionError(
                    "total difficulty from DB exceeds u64".to_string(),
                ))?,
            secondary_scaling: h_db.secondary_scaling as u32,
            total_kernel_offset: h_db.total_kernel_offset.to_hex(),
        })
    }
}

#[derive(Debug, Associations, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "kernels"]
#[belongs_to(BlockHeaderDb, foreign_key = "header_hash")]
pub struct KernelDb {
    pub header_hash: Vec<u8>,
    pub excess: Vec<u8>,
    pub excess_sig: Vec<u8>,
    pub features: i16,
    pub fee: BigDecimal,
    pub fee_shift: i16,
    pub lock_height: BigDecimal,
}

fn to_kernel_feature_string(value: i16) -> String {
    match value {
        0 => "Plain".into(),
        1 => "Coinbase".into(),
        2 => "HeightLocked".into(),
        3 => "NoRecentDuplicate".into(),
        _ => "Unknown".into(),
    }
}

fn from_kernel_feature_string(value: &str) -> i16 {
    match value {
        "Plain" => 0,
        "Coinbase" => 1,
        "HeightLocked" => 2,
        "NoRecentDuplicate" => 3,
        _ => -1,
    }
}

impl TryFrom<TxKernelPrintable> for KernelDb {
    type Error = ModelError;

    fn try_from(kp: TxKernelPrintable) -> Result<Self, Self::Error> {
        Ok(KernelDb {
            header_hash: from_hex("")?, // Note this needs to be filled in manually before insertion
            excess: from_hex(&kp.excess)?,
            excess_sig: from_hex(&kp.excess_sig)?,
            features: from_kernel_feature_string(&kp.features),
            fee: BigDecimal::from(kp.fee),
            fee_shift: kp.fee_shift as i16,
            lock_height: BigDecimal::from(kp.lock_height),
        })
    }
}

impl TryFrom<KernelDb> for TxKernelPrintable {
    type Error = ModelError;

    fn try_from(k_db: KernelDb) -> Result<Self, Self::Error> {
        Ok(TxKernelPrintable {
            excess: k_db.excess.to_hex(),
            excess_sig: k_db.excess.to_hex(),
            features: to_kernel_feature_string(k_db.features),
            fee: k_db.fee.to_u64().ok_or(ModelError::ConversionError(
                "fee from DB exceeds u64".to_string(),
            ))?,
            fee_shift: k_db.fee_shift as u8,
            lock_height: k_db
                .lock_height
                .to_u64()
                .ok_or(ModelError::ConversionError(
                    "lock height from DB exceeds u64".to_string(),
                ))?,
        })
    }
}

fn _to_output_type(value: i16) -> Result<OutputType, ModelError> {
    match value {
        0 => Ok(OutputType::Coinbase),
        1 => Ok(OutputType::Transaction),
        _ => Err(ModelError::ConversionError("Unknown Output Type".into()))
    }
}

fn from_output_type(value: &OutputType) -> i16 {
    match value {
        OutputType::Coinbase => 0,
        OutputType::Transaction => 1,
    }
}

#[derive(Debug, Associations, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "outputs"]
#[belongs_to(BlockHeaderDb, foreign_key = "header_hash")]
pub struct OutputDb {
    pub header_hash: Vec<u8>,
    pub commit: Vec<u8>,
    pub output_type: i16,
    pub proof: Vec<u8>,
}

impl TryFrom<OutputPrintable> for OutputDb {
    type Error = ModelError;

    fn try_from(op: OutputPrintable) -> Result<Self, Self::Error> {
        let proof = match op.proof {
            Some(p) => p,
            None => return Err(ModelError::ConversionError(
                "proof must exist for output".to_string(),
            ))
        };
        Ok(OutputDb {
            header_hash: from_hex("")?, // Note this needs to be filled in manually before insertion
            commit: op.commit.0.to_vec(),
            output_type: from_output_type(&op.output_type),
            proof: from_hex(&proof)?,
        })
    }
}

/*
  TODO: Probably won't need this, will need a custom thing to return instead
  for viewing that contains derived data

  impl TryFrom<OutputDb> for OutputPrintable {
    type Error = ModelError;

    fn try_from(o_db: OutputDb) -> Result<Self, Self::Error> {
        let proof_bin = o_db.proof.to_hex();
        let proof_hash = RangeProof::deserialize(proof_bin).hash();
        Ok(OutputPrintable {
            commit: Commitment::from_vec(o_db.commit),
            output_type: to_output_type(o_db.output_type)?,
            proof: Some(proof_bin),
            proof_hash: proof_hash.
            block_height: 0,
            merkle_proof: None,
            mmr_index: 0,
            spent: false,
        })
    }
}*/

// No conversions here as these require references to existing outputs

#[derive(Debug, Associations, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "inputs"]
#[belongs_to(BlockHeaderDb, foreign_key = "header_hash")]
pub struct InputDb {
    pub header_hash: Vec<u8>,
    pub output_header_hash: Vec<u8>,
    pub commit: Vec<u8>,
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
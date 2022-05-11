use crate::schema::*;
use chrono::*;

use diesel::sql_types::Jsonb;
use diesel::deserialize::FromSql;
use diesel::serialize::{ToSql, Output};
use diesel::pg::Pg;
use std::io::Write;

#[derive(FromSqlRow, AsExpression, Serialize, Deserialize, Debug)]
#[sql_type = "Jsonb"]
pub struct SeedCheckResults {
    pub mainnet: Vec<SeedCheckResult>,
    pub testnet: Vec<SeedCheckResult>
}

impl Default for SeedCheckResults {
    fn default() -> Self {
        Self {
            mainnet: vec![],
            testnet: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedCheckResult {
    pub url: String,
    pub dns_resolutions_found: bool,
    pub success: bool,
    pub successful_attempts: Vec<SeedCheckConnectAttempt>, 
    pub unsuccessful_attempts: Vec<SeedCheckConnectAttempt>, 
}

impl Default for SeedCheckResult {
    fn default() -> Self {
        Self {
            url: "".into(),
            dns_resolutions_found: false,
            success: false,
            successful_attempts: vec![],
            unsuccessful_attempts: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedCheckConnectAttempt {
    pub ip_addr: String,
    pub handshake_success: bool,
    pub user_agent: Option<String>,
    pub capabilities: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "seed_check_results"]
pub struct SeedCheckResultInsertable {
    pub created_at: NaiveDateTime,
    pub results: SeedCheckResults,
}

impl FromSql<Jsonb, Pg> for SeedCheckResults {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for SeedCheckResults {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[derive(Debug, Associations, Queryable)]
#[table_name = "seed_check_results"]
pub struct SeedCheckResultQueryable {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub results: SeedCheckResults,
}






use diesel::prelude::*;

use crate::context::RequestContext;
use crate::errors::{ServiceResult, ServiceError};

use grin_history_tools_models::schema;
use grin_history_tools_models::seedcheck::model::{
    SeedCheckResponse, SeedCheckResultQueryable,
};

pub fn get_seedcheck(
        context: &RequestContext
) -> ServiceResult<SeedCheckResponse> {
    use schema::seed_check_results::dsl::{seed_check_results, created_at};

    let result:SeedCheckResultQueryable = seed_check_results
        .order(created_at.desc())
        .first(&context.db)
        .map_err(|_| ServiceError::NotFound(format!("No seed check results in database")))?;

    Ok(result.into())
} 
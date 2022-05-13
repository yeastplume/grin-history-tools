
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
    use schema::seed_check_results::dsl::seed_check_results;

    let result = seed_check_results
        .first::<SeedCheckResultQueryable>(&context.db)
        .map_err(|_| ServiceError::NotFound(format!("No seed check results in database")))?;

    Ok(result.into())
} 
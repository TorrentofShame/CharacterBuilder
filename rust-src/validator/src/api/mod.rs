//! A quick module to abstract calls to the api.
use std::fmt;

use crate::assets::{MetaData, Asset};

/// Error for API
#[derive(Debug, Clone)]
pub struct ApiError;

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something Happened With the API!")
    }
}

/// Fetches asset definition from an api
pub fn fetch_asset_definition(_ty: &str, id: &str) -> Result<Asset, ApiError> {
    // TODO: Implement
    Ok(Asset::Proficiency {
        metadata: MetaData {
            id: id.to_string(),
            name: Default::default(),
            notes: None,
            description: None,
            extra: Default::default(),
        }
    })
}

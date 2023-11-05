use thiserror::Error;

// 429 	- 	Too many requests - max allowed 400 per minutes from eash unique IP
// 200 	999 	No matching data found
// 400 	999 	Invalid query attributes or parameters
// 401 	  	Unauthorized. Missing or invalid security token
#[derive(Error, Debug)]
pub(crate) enum EntsoeError {
    #[error("{0}")]
    NoMatchingDataFound(String),
    #[error("{0}")]
    InvalidQueryAttributesOrParameters(String),
}

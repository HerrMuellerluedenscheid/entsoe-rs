// Import the module files
mod areas;
mod document_types;
mod process_types;
mod psr_types;

// Export the module files
pub use areas::*;
pub use document_types::*;
pub use process_types::*;
pub use psr_types::*;

pub trait UriElement {
    fn add_to_url(&self, params: &mut Vec<(&str, String)>);
}

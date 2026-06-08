// DOCX export temporarily disabled due to API compatibility issues
// between docx crate versions and their dependencies.
// Will be re-implemented with correct docx crate version

use std::path::Path;
use uuid::Uuid;

use crate::{Result, AnodeError};

pub fn compile_to_docx(
    _library: &Path,
    _book_id: Uuid,
    _include_snapshots: bool,
    _output_path: &Path,
) -> Result<()> {
    Err(AnodeError::msg("DOCX export is not yet implemented"))
}

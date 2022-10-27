use web_sys::Window;

use crate::errors::MediaStreamError;

pub fn window() -> Result<Window, MediaStreamError> {
    match web_sys::window() {
        Some(win) => Ok(win),
        None => Err(MediaStreamError::StructureError {
            structure: "web_sys Window".to_string(),
            error: "None".to_string(),
        }),
    }
}

use crate::magic::generated::T_zip_application;
use crate::magic::{MimeTypeChecker, MIME_MAP};
use std::io::{Cursor, Read};

pub struct ZipSpecialHandler;
#[cfg(feature = "open_zips")]
impl ZipSpecialHandler {
    pub(crate) fn check(&self, bytes: &[u8]) -> Option<&'static str> {
        // Make sure it's a zip file
        let Some(magic) = bytes.get(0..2) else {
            return None;
        };

        if magic != b"PK" {
            return None;
        }

        // Try to open the zip and read its mimetype file
        let Ok(mut zip) = zip::ZipArchive::new(Cursor::new(bytes)) else {
            return None;
        };

        let Ok(mut mimefile) = zip.by_name("mimetype") else {
            return None;
        };

        let mut mimetype = String::new();
        if mimefile.read_to_string(&mut mimetype).is_err() {
            return None;
        }

        let mimetype = mimetype.trim();

        // See if we handle the mimetype
        let Some(handlers) = MIME_MAP.get(mimetype) else {
            return None;
        };

        let Some(first) = handlers.first() else {
            return None;
        };

        Some(first.get_mime())
    }
}

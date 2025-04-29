pub struct ZipSpecialHandler;
#[cfg(feature = "open_zips")]
mod zip_impl {
    use crate::magic::{ZipSpecialHandler, MIME_MAP};
    use std::io::{Cursor, Read, Seek};

    impl ZipSpecialHandler {
        fn has_mimetype_file<R: Read + Seek>(
            &self,
            zip: &mut zip::ZipArchive<R>,
        ) -> Option<&'static str> {
            let Ok(mut mimefile) = zip.by_name("mimetype") else {
                return None;
            };

            let mut mimetype = String::new();
            if mimefile.read_to_string(&mut mimetype).is_err() {
                return None;
            }

            let mimetype = mimetype.trim();
            self.convert_to_static_str(mimetype)
        }

        fn is_ooxml<R: Read + Seek>(&self, zip: &mut zip::ZipArchive<R>) -> Option<&'static str> {
            let directory_listing = zip.file_names().collect::<Vec<_>>();

            for name in directory_listing.iter() {
                if name.starts_with("word/") {
                    return Some(
                        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                    );
                }
                if name.starts_with("ppt/") {
                    return Some(
                        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                    );
                }
                if name.starts_with("xl/") {
                    return Some(
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    );
                }
                if name.starts_with("visio/") {
                    return Some(
                        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                    );
                }
                if name.starts_with("theme/") {
                    return Some("application/vnd.openxmlformats-officedocument");
                }
                if name.starts_with("Documents/1/") {
                    return Some("application/vnd.ms-xpsdocument");
                }

                if name == &"AndroidManifest.xml" {
                    return Some("application/vnd.android.package-archive");
                }
            }

            None
        }

        fn is_apple_office_zip<R: Read + Seek>(
            &self,
            zip: &mut zip::ZipArchive<R>,
        ) -> Option<&'static str> {
            let Ok(mut mimefile) = zip.by_name("index.apxl") else {
                return None;
            };

            let mut mimetype = String::new();
            if mimefile.take(64).read_to_string(&mut mimetype).is_err() {
                return None;
            }

            if mimetype.contains("<key:") {
                return Some("application/vnd.apple.keynote");
            }

            None
        }

        fn is_sun_office_zip<R: Read + Seek>(
            &self,
            zip: &mut zip::ZipArchive<R>,
        ) -> Option<&'static str> {
            let Ok(mut mimefile) = zip.by_name("META-INF/manifest.xml") else {
                return None;
            };

            let mut mimetype = String::new();
            if mimefile.read_to_string(&mut mimetype).is_err() {
                return None;
            }

            if mimetype.contains("application/vnd.sun.xml.calc") {
                return Some("application/vnd.sun.xml.calc");
            }
            if mimetype.contains("application/vnd.sun.xml.writer") {
                return Some("application/vnd.sun.xml.writer");
            }
            if mimetype.contains("application/vnd.sun.xml.impress") {
                return Some("application/vnd.sun.xml.impress");
            }
            if mimetype.contains("application/vnd.sun.xml.draw") {
                return Some("application/vnd.sun.xml.draw");
            }

            None
        }

        fn convert_to_static_str(&self, mimetype: &str) -> Option<&'static str> {
            // See if we handle the mimetype
            let handlers = MIME_MAP.get(mimetype)?;
            let first = handlers.first()?;

            Some(first.get_mime())
        }

        pub(crate) fn check(&self, bytes: &[u8]) -> Option<&'static str> {
            // Make sure it's a zip file
            let magic = bytes.get(0..2)?;

            if magic != b"PK" {
                return None;
            }

            // Try to open the zip and read its mimetype file
            let Ok(mut zip) = zip::ZipArchive::new(Cursor::new(bytes)) else {
                return None;
            };

            // This handles most epub style zips
            if let Some(mimetype) = self.has_mimetype_file(&mut zip) {
                return Some(mimetype);
            }

            // Attempt to handle all the open-office xml zips
            if let Some(mimetype) = self.is_ooxml(&mut zip) {
                return Some(mimetype);
            }

            if let Some(mimetype) = self.is_apple_office_zip(&mut zip) {
                return Some(mimetype);
            }

            if let Some(mimetype) = self.is_sun_office_zip(&mut zip) {
                return Some(mimetype);
            }

            None
        }
    }
}

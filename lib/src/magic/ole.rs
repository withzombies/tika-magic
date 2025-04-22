pub struct OleSpecialHandler;
#[cfg(feature = "open_ole")]
mod ole_impl {
    use crate::magic::OleSpecialHandler;
    use ole::Reader;
    use std::io::Read;

    impl OleSpecialHandler {
        pub(crate) fn check(&self, bytes: &[u8]) -> Option<&'static str> {
            let magic = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
            if !bytes.starts_with(&magic) {
                return None;
            }

            let Ok(reader) = Reader::new(bytes) else {
                return None;
            };

            let names = reader.iterate().map(|e| e.name()).collect::<Vec<_>>();
            for name in names.iter() {
                match name.to_lowercase().as_str() {
                    "worddocument" => return Some("application/msword"),
                    "workbook" | "book" => return Some("application/vnd.ms-excel"),
                    "powerpoint document" => return Some("application/vnd.ms-powerpoint"),
                    "visiodocument" => return Some("application/vnd.visio"),
                    "encryptedpackage" => return Some("application/x-tika-ooxml-protected"),
                    "swdoccontentmgr" => return Some("application/sldworks"),
                    "starcalcdocument" => return Some("application/vnd.stardivision.calc"),
                    "starwriterdocument" => return Some("application/vnd.stardivision.writer"),
                    "stardrawdocument3" => return Some("application/vnd.stardivision.draw"),
                    "quill" => return Some("application/x-mspublisher"),
                    "\u{0005}hwpsummaryinformation" => return Some("application/x-hwp-v5"),
                    "nativecontent_main" => return Some("application/x-quattro-pro"),
                    _ => (),
                }

                if name.starts_with("__substg1.0_") {
                    return Some("application/vnd.ms-outlook");
                }
            }

            if names.contains(&"\u{1}CompObj") {
                if names.contains(&"Props9")
                    || names.contains(&"Props")
                    || names.contains(&"Props12")
                {
                    return Some("application/vnd.ms-project");
                }

                if names.contains(&"SPELLING") || names.contains(&"CONTENTS") {
                    return Some("application/vnd.ms-works");
                }
            }

            if names.contains(&"PerfectOffice_MAIN") {
                if names.contains(&"SlideShow") {
                    return Some("application/x-corelpresentations");
                }

                if names.contains(&"PerfectOffice_OBJECTS") {
                    return Some("application/x-quattro-pro");
                }
            }

            None
        }
    }
}

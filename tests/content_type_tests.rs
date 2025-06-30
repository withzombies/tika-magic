#[cfg(all(test, feature = "open_zips"))]
mod tests {
    use rstest::rstest;
    use std::path::{Path, PathBuf};
    use tika_magic::from_filepath;

    // Helper to extract expected MIME type from the file path
    fn expected_mime_type(path: &Path) -> String {
        // test/inputs/mime/application/zip/zip.zip -> application/zip
        let components: Vec<_> = path.components().collect();
        // components: ["test", "inputs", "mime", "application", "zip", "zip.zip"]
        let len = components.len();
        if len >= 4 {
            let top = components[len - 3].as_os_str().to_string_lossy();
            let sub = components[len - 2].as_os_str().to_string_lossy();
            format!("{top}/{sub}")
        } else {
            panic!("Unexpected path structure: {path:?}");
        }
    }

    #[rstest]
    fn test_mime_detection(#[files("tests/inputs/*/*/*.*")] path: PathBuf) {
        dbg!(&path);
        let expected_mime = expected_mime_type(&path);
        let detected_mime = from_filepath(&path).expect("Failed to detect MIME type");
        assert_eq!(
            expected_mime,
            detected_mime,
            "mkdir -p \"{}\"; mv {} \"{}/\"",
            detected_mime,
            path.to_str().unwrap(),
            detected_mime
        );
    }
}

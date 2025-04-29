//! # Example
//! ```rust
//! // Load a GIF file
//! let input: &[u8] = include_bytes!("../tests/inputs/image/gif/gif.gif");
//!
//! // Check if the MIME and the file are a match
//! let result = tika_magic::match_u8("image/gif", input);
//! assert_eq!(result, true);
//! ```

mod magic;

use crate::magic::{MIME_MAP, MIME_TYPES, PRIORITY_MIME_TYPES};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub type Mime = &'static str;

/// Checks if the given bytestream matches the given MIME type.
///
/// Returns true or false if it matches or not. If the given MIME type is not known,
/// the function will always return false.
/// If mimetype is an alias of a known MIME, the file will be checked against that MIME.
///
/// # Examples
/// ```rust
/// // Load a GIF file
/// let input: &[u8] = include_bytes!("../tests/inputs/image/gif/gif.gif");
///
/// // Check if the MIME and the file are a match
/// let result = tika_magic::match_u8("image/gif", input);
/// assert_eq!(result, true);
/// ```
pub fn match_u8(mimetype: &str, bytes: &[u8]) -> bool {
    if handle_special_files(bytes).is_some() {
        return true;
    }

    let Some(mm) = MIME_MAP.get(mimetype) else {
        return false;
    };

    for m in mm.iter() {
        if m.check(bytes) {
            return true;
        }
    }

    false
}

fn check_recursive(checker: &'static dyn magic::MimeTypeChecker, bytes: &[u8]) -> Option<Mime> {
    let matches = checker.check(bytes);
    if matches || checker.is_virtual() {
        let children = checker.get_children();
        for child in children {
            if let Some(mime) = check_recursive(*child, bytes) {
                return Some(mime);
            }
        }

        if matches {
            return Some(checker.get_mime());
        }
    }

    None
}

#[cfg(feature = "open_zips")]
fn maybe_open_zip(bytes: &[u8]) -> Option<Mime> {
    crate::magic::ZipSpecialHandler.check(bytes)
}
#[cfg(not(feature = "open_zips"))]
fn maybe_open_zip(_bytes: &[u8]) -> Option<Mime> {
    None
}

#[cfg(feature = "open_ole")]
fn maybe_open_ole(bytes: &[u8]) -> Option<Mime> {
    crate::magic::OleSpecialHandler.check(bytes)
}
#[cfg(not(feature = "open_ole"))]
fn maybe_open_ole(_bytes: &[u8]) -> Option<Mime> {
    None
}

fn handle_special_files(bytes: &[u8]) -> Option<Mime> {
    if let Some(mime) = maybe_open_zip(bytes) {
        return Some(mime);
    }

    if let Some(mime) = maybe_open_ole(bytes) {
        return Some(mime);
    }

    None
}

/// Gets the MIME from a byte stream.
///
/// Returns MIME as string.
///
/// # Examples
/// ```rust
/// // Load a GIF file
/// let input: &[u8] = include_bytes!("../tests/inputs/image/gif/gif.gif");
///
/// // Find the MIME type of the GIF
/// let result = tika_magic::from_u8(input);
/// assert_eq!(result, "image/gif");
/// ```
pub fn from_u8(bytes: &[u8]) -> Mime {
    if let Some(mime) = handle_special_files(bytes) {
        return mime;
    }

    for m in PRIORITY_MIME_TYPES {
        if let Some(mime) = check_recursive(*m, bytes) {
            return mime;
        }
    }

    for m in MIME_TYPES {
        if let Some(mime) = check_recursive(*m, bytes) {
            return mime;
        }
    }

    "application/octet-stream"
}

/// Gets the MIME types that match a byte stream.
///
/// Returns a vector of MIMEs.
///
/// # Examples
/// ```rust
/// // Load a MP4 file
/// let input: &[u8] = include_bytes!("../tests/inputs/video/mp4/mp4.mp4");
///
/// // Find the MIME type of the MP4
/// let result = tika_magic::from_u8_exhaustive(input);
/// assert_eq!(result, vec!["video/mp4", "video/quicktime"]);
/// ```
pub fn from_u8_exhaustive(bytes: &[u8]) -> Vec<Mime> {
    MIME_TYPES
        .iter()
        .filter_map(|m| match m.check(bytes) {
            true => Some(m.get_mime()),
            false => None,
        })
        .collect()
}

/// Check if the given file matches the given MIME type.
///
/// # Examples
/// ```rust
/// use std::fs::File;
///
/// // Get path to a GIF file
/// let file = File::open("./tests/inputs/image/gif/gif.gif").unwrap();
///
/// // Check if the MIME and the file are a match
/// let result = tika_magic::match_file("image/gif", &file);
/// assert_eq!(result, true);
/// ```
pub fn match_file(mimetype: &str, file: &File) -> bool {
    let mut buf = [0u8; 0x20000];
    match file.take(buf.len() as u64).read(&mut buf) {
        Ok(0) => return false,
        Err(_) => return false,
        _ => (),
    }

    match_u8(mimetype, &buf)
}

/// Check if the file at the given path matches the given MIME type.
///
/// Returns false if the file could not be read or the given MIME type is not known.
///
/// # Examples
/// ```rust
/// use std::path::Path;
///
/// // Get path to a GIF file
/// let path: &Path = Path::new("./tests/inputs/image/gif/gif.gif");
///
/// // Check if the MIME and the file are a match
/// let result = tika_magic::match_filepath("image/gif", path);
/// assert_eq!(result, true);
/// ```
pub fn match_filepath(mimetype: &str, path: &Path) -> bool {
    match File::open(path) {
        Ok(file) => match_file(mimetype, &file),
        Err(_) => false,
    }
}

/// Gets the MIME type for a file.
///
/// Does not look at file name or extension, just the contents.
///
/// # Examples
/// ```rust
/// use std::fs::File;
///
/// // Get path to a GIF file
/// let file = File::open("./tests/inputs/image/gif/gif.gif").unwrap();
///
/// // Find the MIME type of the GIF
/// let result = tika_magic::from_file(&file);
/// assert_eq!(result, Some("image/gif"));
/// ```
pub fn from_file(file: &File) -> Option<Mime> {
    let mut buf = [0u8; 0x20000];

    match file.take(buf.len() as u64).read(&mut buf) {
        Ok(0) => return None,
        Err(_) => return None,
        _ => (),
    }

    Some(from_u8(&buf))
}

/// Gets all the MIME types that match a file.
///
/// # Examples
/// ```rust
/// use std::fs::File;
///
/// // Get path to a MP4 file
/// let file = File::open("./tests/inputs/video/mp4/mp4.mp4").unwrap();
///
/// // Find the MIME type of the MP4
/// let result = tika_magic::from_file_exhaustive(&file);
/// assert_eq!(result, Some(vec!["video/mp4", "video/quicktime"]));
/// ```
pub fn from_file_exhaustive(file: &File) -> Option<Vec<Mime>> {
    let mut buf = [0u8; 0x20000];

    match file.take(buf.len() as u64).read(&mut buf) {
        Ok(0) => return None,
        Err(_) => return None,
        _ => (),
    }

    Some(from_u8_exhaustive(&buf))
}

/// Gets the MIME type for a path
///
/// Returns None if the file cannot be opened
/// or if no matching MIME type is found.
///
/// # Examples
/// ```rust
/// use std::path::Path;
///
/// // Get path to a GIF file
/// let path = Path::new("./tests/inputs/image/gif/gif.gif");
///
/// // Find the MIME type of the GIF
/// let result = tika_magic::from_filepath(path);
/// assert_eq!(result, Some("image/gif"));
/// ```
pub fn from_filepath(path: &Path) -> Option<Mime> {
    match File::open(path) {
        Ok(file) => from_file(&file),
        Err(_) => None,
    }
}

/// Gets all the MIME types that match for a path.
///
/// # Examples
/// ```rust
/// use std::path::Path;
///
/// // Get path to a MP4 file
/// let path = Path::new("./tests/inputs/video/mp4/mp4.mp4");
///
/// // Find the MIME types of the MP4
/// let result = tika_magic::from_filepath_exhaustive(path);
/// assert_eq!(result, Some(vec!["video/mp4", "video/quicktime"]));
/// ```
pub fn from_filepath_exhaustive(path: &Path) -> Option<Vec<Mime>> {
    match File::open(path) {
        Ok(file) => from_file_exhaustive(&file),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_ooxml_file() {
        let path = Path::new(
            "./tests/inputs/application/vnd.openxmlformats-officedocument.presentationml.presentation/vnd.openxmlformats-officedocument.presentationml.presentation.pptx",
        );
        assert!(path.exists());

        assert_eq!(
            from_filepath(path).unwrap(),
            "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        );
        assert!(match_filepath(
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            path
        ));
    }

    #[rstest]
    fn test_ods_file() {
        let path = Path::new(
            "./tests/inputs/application/vnd.oasis.opendocument.spreadsheet/vnd.oasis.opendocument.spreadsheet.ods",
        );
        assert!(path.exists());
        assert!(match_filepath(
            "application/vnd.oasis.opendocument.spreadsheet",
            path
        ));
        assert_eq!(
            from_filepath(path).unwrap(),
            "application/vnd.oasis.opendocument.spreadsheet"
        );
    }

    #[rstest]
    fn test_uue_file() {
        let data = include_bytes!("../tests/inputs/text/x-uu-encoded/sample-data-csv.uue");
        assert_eq!(from_u8(data), "text/x-uuencode");
        assert!(match_u8("text/x-uuencode", data));
    }
}

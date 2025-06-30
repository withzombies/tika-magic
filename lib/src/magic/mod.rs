#![allow(unused)]

use regex::bytes::Regex;
use std::cmp::{max, min};
use std::sync::Arc;

mod generated;
mod ole;
mod zip;

pub use generated::{EXT_MAP, MIME_MAP, MIME_TYPES};
pub use ole::OleSpecialHandler;
pub use zip::ZipSpecialHandler;

use generated::*;

pub static PRIORITY_MIME_TYPES: &[&'static dyn MimeTypeChecker] = &[
    &T_png_image,
    &T_gif_image,
    &T_jpeg_image,
    &T_zip_application,
    &T_pdf_application,
    &T_x_dosexec_application,
];

struct MimeType {
    mime: String,
    ext: String,
    children: Vec<Arc<MimeType>>,
    detector: dyn Fn(&[u8]) -> bool,
}

pub trait MimeTypeChecker: Send + Sync {
    fn check(&self, bytes: &[u8]) -> bool;
    fn get_mime(&self) -> &'static str;
    fn get_ext(&self) -> &[&'static str];
    fn get_children(&self) -> &[&'static dyn MimeTypeChecker];
    fn is_virtual(&self) -> bool;
}

// Detector helpers
pub(crate) fn prefix(bytes: &[u8], needle: &[u8]) -> bool {
    bytes.starts_with(needle)
}

pub(crate) fn offset(bytes: &[u8], start: usize, needle: &[u8]) -> bool {
    if let Some(slice) = &bytes.get(start..) {
        prefix(slice, needle)
    } else {
        false
    }
}

pub(crate) fn offset_range(bytes: &[u8], start: usize, end: usize, needle: &[u8]) -> bool {
    let end = std::cmp::min(end, bytes.len().saturating_sub(1));
    if end < start || start >= bytes.len() {
        return false;
    }

    if let Some(slice) = &bytes.get(start..=end) {
        slice.windows(needle.len()).any(|window| window == needle)
    } else {
        false
    }
}

pub(crate) fn offset_mask(bytes: &[u8], start: usize, needle: &[u8], mask: &[u8]) -> bool {
    if let Some(slice) = &bytes.get(start..) {
        slice
            .iter()
            .zip(needle.iter().zip(mask.iter()))
            .all(|(s, (n, m))| (s & m) == (n & m))
    } else {
        false
    }
}

pub(crate) fn offset_mask_range(
    bytes: &[u8],
    start: usize,
    end: usize,
    needle: &[u8],
    mask: &[u8],
) -> bool {
    let end = std::cmp::min(end, bytes.len().saturating_sub(1));
    if end < start || start >= bytes.len() {
        return false;
    }

    if let Some(slice) = &bytes.get(start..=end) {
        let masked_needle = needle
            .iter()
            .zip(mask.iter())
            .map(|(n, m)| n & m)
            .collect::<Vec<_>>();

        slice.windows(needle.len()).any(|window| {
            window
                .iter()
                .zip(mask.iter())
                .map(|(w, m)| w & m)
                .zip(masked_needle.iter())
                .all(|(w, m)| w == *m)
        })
    } else {
        false
    }
}

pub(crate) fn prefix_case_insensitive(bytes: &[u8], needle: &[u8]) -> bool {
    let bytes_iter = bytes.iter();
    let needle_iter = needle.iter();

    for (b, p) in bytes_iter.zip(needle_iter) {
        if !b.eq_ignore_ascii_case(p) {
            return false;
        }
    }

    true
}

pub(crate) fn offset_case_insensitive(bytes: &[u8], start: usize, needle: &[u8]) -> bool {
    if let Some(slice) = &bytes.get(start..) {
        prefix_case_insensitive(slice, needle)
    } else {
        false
    }
}

pub(crate) fn offset_range_case_insensitive(
    bytes: &[u8],
    start: usize,
    end: usize,
    needle: &[u8],
) -> bool {
    let end = std::cmp::min(end, bytes.len().saturating_sub(1));
    if end < start || start >= bytes.len() {
        return false;
    }

    if let Some(slice) = &bytes.get(start..=end) {
        slice
            .windows(needle.len())
            .any(|window| window.eq_ignore_ascii_case(needle))
    } else {
        false
    }
}

pub(crate) fn prefix_string(bytes: &[u8], needle: &str) -> bool {
    prefix(bytes, needle.as_bytes())
}

pub(crate) fn prefix_case_insensitive_string(bytes: &[u8], needle: &str) -> bool {
    prefix_case_insensitive(bytes, needle.as_bytes())
}

pub(crate) fn offset_string(bytes: &[u8], start: usize, needle: &str) -> bool {
    offset(bytes, start, needle.as_bytes())
}

pub(crate) fn offset_case_insensitive_string(bytes: &[u8], start: usize, needle: &str) -> bool {
    offset_case_insensitive(bytes, start, needle.as_bytes())
}

pub(crate) fn regex(bytes: &[u8], start: usize, needle: &Regex) -> bool {
    if let Some(slice) = &bytes.get(start..) {
        needle.is_match(slice)
    } else {
        false
    }
}

pub(crate) fn regex_range(bytes: &[u8], start: usize, end: usize, needle: &Regex) -> bool {
    if let Some(slice) = &bytes.get(start..=end) {
        needle.is_match(slice)
    } else {
        false
    }
}

pub(crate) fn little32(bytes: &[u8], start: usize, needle: u32) -> bool {
    if let Some(slice) = bytes.get(start..) {
        if slice.len() >= 4 {
            let sized_slice = [slice[0], slice[1], slice[2], slice[3]];
            u32::from_le_bytes(sized_slice) == needle
        } else {
            false
        }
    } else {
        false
    }
}

pub(crate) fn little16(bytes: &[u8], start: usize, needle: u16) -> bool {
    if let Some(slice) = bytes.get(start..) {
        if slice.len() >= 2 {
            let sized_slice = [slice[0], slice[1]];
            u16::from_le_bytes(sized_slice) == needle
        } else {
            false
        }
    } else {
        false
    }
}

// Host16 is little endian on all reasonable platforms running rust
pub(crate) fn host16(bytes: &[u8], start: usize, needle: u16) -> bool {
    little16(bytes, start, needle)
}

pub(crate) fn unicode_le(bytes: &[u8], start: usize, needle: &[u8]) -> bool {
    if let Some(slice) = bytes.get(start..) {
        offset(bytes, start, needle)
    } else {
        false
    }
}

pub(crate) fn unicode_le_range(bytes: &[u8], start: usize, end: usize, needle: &[u8]) -> bool {
    if let Some(slice) = bytes.get(start..=end) {
        offset_range(bytes, start, end, needle)
    } else {
        false
    }
}

pub fn file_is_text(bytes: &[u8]) -> bool {
    // Let's check to make sure everything is printable
    if let Some(slice) = bytes.get(0..bytes.len().clamp(4, 128)) {
        // Slice must be ascii but can start with byte order marks
        if !slice.is_ascii()
            && (slice[0] != 0x00 && slice[1] != 0x00 && slice[2] != 0xfe && slice[3] != 0xff)
            && (slice[0] != 0xff && slice[1] != 0xfe && slice[2] != 0x00 && slice[3] != 0x00)
            && (slice[0] != 0xef
                && slice[1] != 0xbb
                && slice[2] != 0xbf
                && slice[3].is_ascii_whitespace())
        {
            return false;
        }

        match slice.get(4..=min(8, slice.len())) {
            Some(slice) => slice.iter().all(|b| *b > 0 && *b < 0x80),
            None => bytes
                .iter()
                .all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace()),
        }
    } else {
        bytes.is_ascii()
    }
}

pub(crate) fn rootxml(bytes: &[u8], local_name: &str, namespace_uri: &str) -> bool {
    let local = rootxml_local(bytes, local_name);
    let ns = rootxml_namespace(bytes, namespace_uri);

    local && ns
}

pub(crate) fn rootxml_local(bytes: &[u8], local_name: &str) -> bool {
    let local_name_tag = format!("<{local_name} ");
    let local_name_bytes = local_name_tag.as_bytes();
    let local_name_with_namespace = format!("{local_name}:{local_name} ");
    let local_name_with_namespace_bytes = local_name_with_namespace.as_bytes();

    let localname_followed_by_namspace = format!(":{local_name} xmlns");
    let localname_followed_by_namspace_bytes = localname_followed_by_namspace.as_bytes();

    file_is_text(bytes)
        && (offset_range_case_insensitive(bytes, 0, 2048, local_name_bytes)
            || offset_range_case_insensitive(bytes, 0, 2048, local_name_with_namespace_bytes)
            || offset_range_case_insensitive(bytes, 0, 2048, localname_followed_by_namspace_bytes))
}

pub(crate) fn rootxml_namespace(bytes: &[u8], namespace_uri: &str) -> bool {
    file_is_text(bytes) && offset_range_case_insensitive(bytes, 0, 2048, namespace_uri.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le32() {
        assert!(little32(b"\xFD\x2F\xB5\x28", 0, 0x28B52FFD));
    }

    #[test]
    fn test_file_is_text() {
        assert!(file_is_text(b"Hello, world!"));
        assert!(file_is_text(b"\xff\xfe\x00\x00lollol"));
        assert!(!file_is_text(b"\x00\x00\x00\x00"));
    }
}

#![allow(unused)]

use regex::bytes::Regex;
use std::sync::Arc;

mod generated;

pub use generated::{EXT_MAP, MIME_MAP, MIME_TYPES};

use generated::*;

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
}

// Detector helpers
pub(crate) fn prefix(bytes: &[u8], needle: &[u8]) -> bool {
    bytes.starts_with(needle)
}

pub(crate) fn offset(bytes: &[u8], start: usize, needle: &[u8]) -> bool {
    let Some(slice) = &bytes.get(start..) else {
        return false;
    };

    prefix(slice, needle)
}

pub(crate) fn offset_range(bytes: &[u8], start: usize, end: usize, needle: &[u8]) -> bool {
    let Some(slice) = &bytes.get(start..=end) else {
        return false;
    };

    slice.windows(needle.len()).any(|window| window == needle)
}

pub(crate) fn offset_mask(bytes: &[u8], start: usize, needle: &[u8], mask: &[u8]) -> bool {
    let Some(slice) = &bytes.get(start..) else {
        return false;
    };

    slice
        .iter()
        .zip(needle.iter().zip(mask.iter()))
        .all(|(s, (n, m))| (s & m) == (n & m))
}

pub(crate) fn offset_mask_range(
    bytes: &[u8],
    start: usize,
    end: usize,
    needle: &[u8],
    mask: &[u8],
) -> bool {
    let Some(slice) = &bytes.get(start..=end) else {
        return false;
    };

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
    let Some(slice) = &bytes.get(start..) else {
        return false;
    };

    prefix_case_insensitive(slice, needle)
}

pub(crate) fn offset_range_case_insensitive(
    bytes: &[u8],
    start: usize,
    end: usize,
    needle: &[u8],
) -> bool {
    let Some(slice) = &bytes.get(start..=end) else {
        return false;
    };

    slice
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
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
    let Some(slice) = &bytes.get(start..) else {
        return false;
    };

    needle.is_match(slice)
}

pub(crate) fn regex_range(bytes: &[u8], start: usize, end: usize, needle: &Regex) -> bool {
    let Some(slice) = &bytes.get(start..=end) else {
        return false;
    };

    needle.is_match(slice)
}

pub(crate) fn little32(bytes: &[u8], start: usize, needle: u32) -> bool {
    let Some(slice) = bytes.get(start..) else {
        return false;
    };

    let Ok(sized_slice) = slice.try_into() as Result<[u8; 4], _> else {
        return false;
    };
    u32::from_le_bytes(sized_slice) == needle
}

pub(crate) fn little16(bytes: &[u8], start: usize, needle: u16) -> bool {
    let Some(slice) = bytes.get(start..) else {
        return false;
    };

    let Ok(sized_slice) = slice.try_into() as Result<[u8; 2], _> else {
        return false;
    };

    u16::from_le_bytes(sized_slice) == needle
}

// Host16 is little endian on all reasonable platforms running rust
pub(crate) fn host16(bytes: &[u8], start: usize, needle: u16) -> bool {
    little16(bytes, start, needle)
}

pub(crate) fn unicode_le(bytes: &[u8], start: usize, needle: &[u8]) -> bool {
    let Some(slice) = bytes.get(start..) else {
        return false;
    };

    offset(bytes, start, needle)
}

pub(crate) fn unicode_le_range(bytes: &[u8], start: usize, end: usize, needle: &[u8]) -> bool {
    let Some(slice) = bytes.get(start..=end) else {
        return false;
    };

    offset_range(bytes, start, end, needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le32() {
        assert!(little32(b"\xFD\x2F\xB5\x28", 0, 0x28B52FFD));
    }
}

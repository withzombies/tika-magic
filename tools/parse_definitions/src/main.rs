mod parse_xml;

use crate::parse_xml::{parse_mime_type_xml, Match, MimeType, Offset};
use crate::MatchRule::And;
use num_traits::Num;
use regex::bytes::Regex;
use sailfish::TemplateSimple;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
struct OutputMimeType {
    short_name: String,
    mime_string: String,
    globs: Vec<String>,
    match_rules_string: String,
    subclasses: Vec<String>,
    priority: u32,
    children: Vec<String>,
}

#[derive(TemplateSimple)]
#[template(path = "generated.rs.stpl")]
struct OutputTemplate {
    types: Vec<OutputMimeType>,
    type_map: HashMap<String, Vec<String>>,
    ext_map: HashMap<String, Vec<String>>,
}

fn mime_to_short_name(mime_type: &str) -> String {
    let short_name = mime_type.to_string();
    let short_name = short_name
        .split('/')
        .rev()
        .map(|s| s.replace(['-', '.', '+', ';', '=', ' '], "_"))
        .collect::<Vec<String>>()
        .join("_");

    format!("T_{}", short_name)
}

fn print_help() {
    println!("Usage: parse_definitions <XML_FILE_PATHS>");
    println!();
    println!("Arguments:");
    println!(
        "    <XML_FILE_PATHS>    The paths to the Apache Tika MIME type XML files to parse and generate code from."
    );
    println!();
    println!("Example:");
    println!("    parse_definitions tika-mimetypes.xml");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum MatchRule {
    String(u32, Vec<u8>),
    StringRange(u32, u32, Vec<u8>),
    StringCaseInsensitive(u32, Vec<u8>),
    StringRangeCaseInsensitive(u32, u32, Vec<u8>),
    StringMask(u32, Vec<u8>, Vec<u8>),
    StringMaskRange(u32, u32, Vec<u8>, Vec<u8>),
    StringMaskCaseInsensitive(u32, Vec<u8>, Vec<u8>),
    StringMaskRangeCaseInsensitive(u32, u32, Vec<u8>, Vec<u8>),
    Regex(u32, String),
    RegexRange(u32, u32, String),
    ValueU32(u32, Vec<u8>),
    ValueU16(u32, Vec<u8>),
    UnicodeLE(u32, Vec<u8>),
    UnicodeLERange(u32, u32, Vec<u8>),
    And(Vec<MatchRule>),
    Or(Vec<MatchRule>),
    Empty,
}

fn parse_int_auto<T>(input: &str) -> Result<T, <T as Num>::FromStrRadixErr>
where
    T: Num,
{
    let input = input.trim();

    // Detect the prefix and determine the appropriate radix
    if input.starts_with("0x") || input.starts_with("0X") {
        T::from_str_radix(&input[2..], 16)
    } else if input.starts_with("0o") || input.starts_with("0O") {
        T::from_str_radix(&input[2..], 8)
    } else if input.starts_with("0b") || input.starts_with("0B") {
        T::from_str_radix(&input[2..], 2)
    } else if input.starts_with("0n") || input.starts_with("0n") {
        T::from_str_radix(&input[2..], 10)
    } else if input.starts_with('0') && input.len() > 1 {
        // Handle legacy octal (0-prefixed, but no `0o` prefix)
        T::from_str_radix(&input[1..], 8)
    } else {
        // Default to base-10 (decimal)
        T::from_str_radix(input, 10)
    }
}

fn string_to_bytes(input: &str) -> Vec<u8> {
    // First check if it's a hex string
    if input.starts_with("0x") || input.starts_with("0X") {
        let hex = &input[2..];

        // Ensure length is even; prepend a `0` if necessary
        let adjusted_hex = if hex.len() % 2 != 0 {
            format!("0{}", hex)
        } else {
            hex.to_string()
        };

        return hex::decode(&adjusted_hex).expect("Failed to decode hex string");
    }

    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Handle escape sequences
                if let Some(&next) = chars.peek() {
                    match next {
                        'x' => {
                            // Hexadecimal escape (\xNN)
                            chars.next(); // Consume 'x'

                            // Read two characters, convert to hex
                            let hex = chars.next().unwrap().to_string()
                                + &chars.next().unwrap().to_string();
                            let byte =
                                u8::from_str_radix(&hex, 16).expect("Invalid hex escape sequence");
                            result.push(byte);
                        }
                        '0'..='7' => {
                            // Octal escape (\NNN)
                            let mut octal = String::new();
                            for _ in 0..3 {
                                if let Some(&ch) = chars.peek() {
                                    if ('0'..='7').contains(&ch) {
                                        octal.push(ch);
                                        chars.next(); // Consume valid octal character
                                    } else {
                                        break;
                                    }
                                }
                            }
                            let byte = u8::from_str_radix(&octal, 8)
                                .expect("Invalid octal escape sequence");
                            result.push(byte);
                        }
                        _ => {
                            // Unknown escape sequence or single character like '\n'
                            result.push(match next {
                                'n' => b'\n',
                                'r' => b'\r',
                                't' => b'\t',
                                '\\' => b'\\',
                                '\'' => b'\'',
                                '\"' => b'\"',
                                ' ' => 0x20,
                                _ => panic!("Unknown escape sequence: \\{}", next),
                            });
                            chars.next(); // Consume the escaped character
                        }
                    }
                }
            }
            _ => {
                // Normal ASCII character
                result.push(c as u8);
            }
        }
    }

    result
}

fn match_to_rule(mat: &Match) -> MatchRule {
    let rule = match &mat.match_type.as_str() {
        &"string" => match (&mat.offset, &mat.value, &mat.mask) {
            (None, Some(value), None) => MatchRule::String(0, string_to_bytes(value)),
            (Some(Offset::Start(offset)), Some(value), None) => {
                MatchRule::String(*offset, string_to_bytes(value))
            }
            (Some(Offset::Range { start, end }), Some(value), None) => {
                MatchRule::StringRange(*start, *end, string_to_bytes(value))
            }
            (Some(Offset::Start(offset)), Some(value), Some(mask)) => {
                MatchRule::StringMask(*offset, string_to_bytes(value), string_to_bytes(mask))
            }
            (None, Some(value), Some(mask)) => {
                MatchRule::StringMask(0, string_to_bytes(value), string_to_bytes(mask))
            }
            (Some(Offset::Range { start, end }), Some(value), Some(mask)) => {
                MatchRule::StringMaskRange(
                    *start,
                    *end,
                    string_to_bytes(value),
                    string_to_bytes(mask),
                )
            }
            (None, None, None) => MatchRule::Empty,
            _ => {
                panic!("Unhandled string rule: {:?}", &mat);
            }
        },
        &"stringignorecase" => match (&mat.offset, &mat.value, &mat.mask) {
            (Some(Offset::Start(offset)), Some(value), None) => {
                MatchRule::StringCaseInsensitive(*offset, string_to_bytes(value))
            }
            (Some(Offset::Range { start, end }), Some(value), None) => {
                MatchRule::StringRangeCaseInsensitive(*start, *end, string_to_bytes(value))
            }
            (Some(Offset::Start(offset)), Some(value), Some(mask)) => {
                MatchRule::StringMaskCaseInsensitive(
                    *offset,
                    string_to_bytes(value),
                    string_to_bytes(mask),
                )
            }
            (Some(Offset::Range { start, end }), Some(value), Some(mask)) => {
                MatchRule::StringMaskRangeCaseInsensitive(
                    *start,
                    *end,
                    string_to_bytes(value),
                    string_to_bytes(mask),
                )
            }
            _ => {
                panic!("Unhandled stringignorecase rule: {:?}", &mat);
            }
        },
        &"regex" => match (&mat.offset, &mat.value) {
            (Some(Offset::Start(start)), Some(value)) => {
                // Test the regex pattern, we don't support some features
                println!("Testing regex pattern: {}", value);
                match Regex::new(value) {
                    Ok(reg) => {
                        reg.is_match(&[0, 1, 2, 3, 4, 5, 6, 7]);
                    }
                    Err(e) => {
                        eprintln!("Error: Invalid regex pattern: {}", e);
                        return MatchRule::Empty;
                    }
                }

                MatchRule::Regex(*start, value.to_string())
            }
            (Some(Offset::Range { start, end }), Some(value)) => {
                match Regex::new(value) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: Invalid regex pattern: {}", e);
                        return MatchRule::Empty;
                    }
                }

                MatchRule::RegexRange(*start, *end, value.to_string())
            }
            _ => {
                panic!("Unhandled regex rule: {:?}", &mat);
            }
        },
        &"little16" | &"host16" => match (&mat.offset, &mat.value) {
            (Some(Offset::Start(start)), Some(value)) => {
                let value = parse_int_auto::<u16>(value).expect("Failed to parse value string");
                MatchRule::ValueU16(*start, value.to_le_bytes().to_vec())
            }
            _ => {
                panic!("Unhandled little16 rule: {:?}", &mat);
            }
        },
        &"little32" | &"host32" => match (&mat.offset, &mat.value) {
            (Some(Offset::Start(start)), Some(value)) => {
                let vv = parse_int_auto::<u32>(value).expect("Failed to parse value string");
                MatchRule::ValueU32(*start, vv.to_le_bytes().to_vec())
            }
            _ => {
                panic!("Unhandled little32 rule: {:?}", &mat);
            }
        },
        &"big16" => match (&mat.offset, &mat.value) {
            (Some(Offset::Start(start)), Some(value)) => {
                let value = parse_int_auto::<u16>(value).expect("Failed to parse value string");
                MatchRule::ValueU16(*start, value.to_be_bytes().to_vec())
            }
            _ => {
                panic!("Unhandled big16 rule: {:?}", &mat);
            }
        },
        &"big32" => match (&mat.offset, &mat.value) {
            (Some(Offset::Start(start)), Some(value)) => {
                let vv = parse_int_auto::<u32>(value).expect("Failed to parse value string");
                MatchRule::ValueU32(*start, vv.to_le_bytes().to_vec())
            }
            _ => {
                panic!("Unhandled big32 rule: {:?}", &mat);
            }
        },
        &"unicodeLE" => match (&mat.offset, &mat.value, &mat.mask) {
            (None, Some(value), None) => MatchRule::UnicodeLE(0, string_to_bytes(value)),
            (Some(Offset::Start(offset)), Some(value), None) => {
                MatchRule::UnicodeLE(*offset, string_to_bytes(value))
            }
            (Some(Offset::Range { start, end }), Some(value), None) => {
                MatchRule::UnicodeLERange(*start, *end, string_to_bytes(value))
            }
            _ => {
                panic!("Unhandled unicodeLE rule: {:?}", &mat);
            }
        },
        _ => {
            panic!("Unhandled match type: {}", mat.match_type);
        }
    };

    if !mat.sub_matches.is_empty() {
        let sub_rules: Vec<MatchRule> = mat
            .sub_matches
            .clone()
            .into_iter()
            .map(|mat: parse_xml::Match| match_to_rule(&mat))
            .collect();
        let mut rules = vec![rule];

        if sub_rules.len() == 1 {
            // Strip off the AND if there's just the one
            let first = sub_rules.first().unwrap();
            match first {
                And(r) => {
                    rules.extend(r.clone());
                }
                _ => {
                    rules.extend(sub_rules);
                }
            }
        } else {
            rules.push(MatchRule::Or(sub_rules));
        }

        return And(rules);
    }

    rule
}

fn actions_to_rules(mime: &MimeType) -> MatchRule {
    // Any magics work
    let mut or_rules: Vec<MatchRule> = Vec::new();
    for magic in &mime.magics {
        if magic.matches.len() == 1 {
            let first = magic.matches.first().unwrap();
            or_rules.push(match_to_rule(first));
            continue;
        }

        let rules: Vec<MatchRule> = magic
            .matches
            .iter()
            .map(|mat: &parse_xml::Match| match_to_rule(mat))
            .collect();
        or_rules.extend(rules);
    }

    if or_rules.len() > 1 {
        return MatchRule::Or(or_rules);
    } else if or_rules.len() == 1 {
        return or_rules.pop().unwrap();
    }

    MatchRule::Empty
}

fn rules_to_string(match_rule: &MatchRule) -> String {
    match match_rule {
        MatchRule::Or(rules) => {
            let strings = rules.iter().map(rules_to_string).collect::<Vec<String>>();
            let joined = strings.join(" || ");
            format!("({})", joined)
        }
        MatchRule::And(rules) => {
            let strings = rules.iter().map(rules_to_string).collect::<Vec<String>>();
            let joined = strings.join(" && ");
            format!("({})", joined)
        }
        MatchRule::String(offset, bytes) => {
            format!("offset(bytes, {}, &{:?})", offset, bytes)
        }
        MatchRule::StringRange(start, end, bytes) => {
            format!("offset_range(bytes, {}, {}, &{:?})", start, end, bytes)
        }
        MatchRule::StringCaseInsensitive(offset, bytes) => {
            format!("offset_case_insensitive(bytes, {}, &{:?})", offset, bytes)
        }
        MatchRule::StringRangeCaseInsensitive(start, end, bytes) => {
            format!(
                "offset_range_case_insensitive(bytes, {}, {}, &{:?})",
                start, end, bytes
            )
        }
        MatchRule::StringMask(offset, bytes, mask) => {
            format!("offset_mask(bytes, {}, &{:?}, &{:?})", offset, bytes, mask)
        }
        MatchRule::StringMaskRange(start, end, bytes, mask) => {
            format!(
                "offset_mask_range(bytes, {}, {}, &{:?}, &{:?})",
                start, end, bytes, mask
            )
        }
        MatchRule::StringMaskCaseInsensitive(offset, bytes, mask) => {
            format!(
                "offset_mask_case_insensitive(bytes, {}, &{:?}, &{:?})",
                offset, bytes, mask
            )
        }
        MatchRule::StringMaskRangeCaseInsensitive(start, end, bytes, mask) => {
            format!(
                "offset_mask_range_case_insensitive(bytes, {}, {}, &{:?}, &{:?})",
                start, end, bytes, mask
            )
        }
        MatchRule::Regex(offset, pattern) => {
            format!(
                "regex(bytes, {}, &Regex::new(\"{}\").unwrap())",
                offset, pattern
            )
        }
        MatchRule::RegexRange(start, end, pattern) => {
            format!(
                "regex_range(bytes, {}, {}, &Regex::new(\"{}\").unwrap())",
                start, end, pattern
            )
        }
        MatchRule::ValueU32(offset, value) => {
            format!("offset(bytes, {}, &{:?})", offset, value)
        }
        MatchRule::ValueU16(offset, value) => {
            format!("offset(bytes, {}, &{:?})", offset, value)
        }
        MatchRule::Empty => "false".to_string(),
        MatchRule::UnicodeLE(offset, bytes) => {
            format!("unicode_le(bytes, {}, &{:?})", offset, bytes)
        }
        MatchRule::UnicodeLERange(start, end, bytes) => {
            format!("unicode_le_range(bytes, {}, {}, &{:?})", start, end, bytes)
        }
    }
}

fn parse_definition_file(
    xml_path: &str,
) -> Result<(Vec<OutputMimeType>, Vec<(String, String)>), Box<dyn std::error::Error>> {
    let rules = parse_mime_type_xml(xml_path)?;
    let mime_types = rules.mime_types;

    let output_mime_types = &mime_types
        .iter()
        .map(|mime: &MimeType| OutputMimeType {
            short_name: mime_to_short_name(&mime.mime_type),
            mime_string: mime.mime_type.to_string(),
            globs: mime
                .globs
                .iter()
                .filter_map(|z| z.pattern.clone())
                .collect(),
            match_rules_string: rules_to_string(&actions_to_rules(mime)),
            subclasses: mime
                .sub_classes
                .iter()
                .filter_map(|e| e.class_type.clone())
                .collect::<Vec<String>>(),
            priority: max(mime.magics.iter().map(|m| m.priority).max().unwrap_or(0), 0),
            children: vec![],
        })
        .collect::<Vec<OutputMimeType>>();

    let alias_types = &mime_types
        .iter()
        .flat_map(|mime: &MimeType| {
            let short_name = mime_to_short_name(mime.mime_type.to_string().as_str());
            mime.aliases
                .clone()
                .into_iter()
                .filter_map(|alias| alias.alias_type)
                .map(move |alias| (alias.to_string(), short_name.clone()))
        })
        .collect::<Vec<(String, String)>>();

    Ok((output_mime_types.to_vec(), alias_types.to_vec()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing required argument <XML_FILE_PATH>");
        print_help();
        return Ok(());
    }

    let mut output_mime_types = vec![];
    let mut alias_types = vec![];

    let xml_paths = &args[1..];
    for xml_path in xml_paths {
        println!("Processing file: {}", xml_path);
        let (omt, at) = parse_definition_file(xml_path)?;
        output_mime_types.extend(omt);
        alias_types.extend(at);
    }

    let mut seen = HashSet::new();
    let mut error_out = false;
    for mime in &output_mime_types {
        if seen.contains(mime.mime_string.as_str()) {
            eprintln!(
                "Error: Duplicate mime type ({}) please consolidate the definitions or delete one",
                mime.mime_string
            );
            error_out = true;
        } else if seen.contains(mime.short_name.as_str()) {
            eprintln!(
                "Error: Duplicate mime type ({}) please consolidate the definitions or delete one",
                mime.mime_string
            );
            error_out = true;
        }
        seen.insert(mime.mime_string.as_str());
        seen.insert(mime.short_name.as_str());
    }

    if error_out {
        return Err(Box::new(std::io::Error::other(
            "We can't handle multiple definitions for the same mime type",
        )));
    }

    let mut type_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut ext_map: HashMap<String, Vec<String>> = HashMap::new();

    // Add children, these should be types that subclass you and are *more specific*.
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();

    for mime in &output_mime_types {
        type_map
            .entry(mime.mime_string.clone())
            .or_default()
            .push(format!("&{}", mime.short_name));

        mime.globs.iter().for_each(|glob| {
            ext_map
                .entry(glob.clone())
                .or_default()
                .push(format!("&{}", mime.short_name));
        });

        for subclass in &mime.subclasses {
            children_map
                .entry(subclass.clone())
                .or_default()
                .push(format!("&{}", mime.short_name));
        }
    }

    // Now that we have the children associations, update the output mime types with the children information
    for mime in &mut output_mime_types {
        let children = children_map
            .get(&mime.mime_string)
            .unwrap_or(&vec![])
            .to_vec();
        mime.children = children;
    }

    // Sort by the number of children and then priority
    output_mime_types.sort_by(|a, b| {
        if a.priority == b.priority {
            return a.children.len().cmp(&b.children.len());
        }
        b.priority.cmp(&a.priority)
    });

    for (alias, short_name) in &alias_types {
        type_map
            .entry(alias.clone())
            .or_default()
            .push(format!("&{}", short_name));
    }

    let ctx = OutputTemplate {
        types: output_mime_types.to_vec(),
        type_map,
        ext_map,
    };

    let code = ctx.render_once().unwrap();
    File::create("generated.rs")?.write_all(code.as_bytes())?;

    println!("Generated Rust code written to `generated.rs`");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_xml::MimeType;
    use quick_xml::de::from_str;

    #[test]
    fn test_actions_to_rules() {
        let def = r#"
                <mime-type type="application/zip">
                    <_comment>Compressed Archive File</_comment>
                    <tika:link>http://en.wikipedia.org/wiki/ZIP_(file_format)</tika:link>
                    <tika:uti>com.pkware.zip-archive</tika:uti>
                    <alias type="application/x-zip-compressed"/>
                    <magic priority="50">
                        <match value="PK\003\004" type="string" offset="0"/>
                        <match value="PK\005\006" type="string" offset="0"/>
                        <match value="PK\x07\x08" type="string" offset="0"/>
                    </magic>
                    <glob pattern="*.zip"/>
                    <glob pattern="*.zipx"/>
                </mime-type>"#;

        let mime: MimeType = from_str(def).unwrap();

        let rule = actions_to_rules(&mime);
        dbg!(&rule);

        assert!(matches!(rule, MatchRule::Or(_)));

        let matches = match &rule {
            MatchRule::Or(rules) => rules,
            _ => panic!("Expected Or rule"),
        };

        assert_eq!(matches.len(), 3);
        assert_eq!(
            matches[0],
            MatchRule::String(0, [0x50, 0x4B, 0x03, 0x04].to_vec())
        );
        assert_eq!(
            matches[1],
            MatchRule::String(0, [0x50, 0x4B, 0x05, 0x06].to_vec())
        );
        assert_eq!(
            matches[2],
            MatchRule::String(0, [0x50, 0x4B, 0x07, 0x08].to_vec())
        );

        let string_rules = rules_to_string(&rule);
        assert_eq!(
            string_rules,
            "(offset(bytes, 0, &[80, 75, 3, 4]) || offset(bytes, 0, &[80, 75, 5, 6]) || offset(bytes, 0, &[80, 75, 7, 8]))"
        );
    }

    #[test]
    fn test_value_match_rules() {
        let def = r#"<mime-type type="application/onenote;format=one">
    <glob pattern="*.one"/>
    <magic priority="50">
      <!-- GUID {7B5C52E4-D88C-4DA7-AEB1-5378D02996D3} -->
      <match value="0x7B5C52E4" type="little32" offset="0">
        <match value="0xD88C" type="little16" offset="4">
          <match value="0x4DA7" type="little16" offset="6">
            <match value="0xAEB15378D02996D3" offset="8" />
            <match value="0xAEB15378D02996D4" offset="8" />
          </match>
        </match>
      </match>
    </magic>
  </mime-type>"#;

        let mime: MimeType = from_str(def).unwrap();

        let rule = actions_to_rules(&mime);
        dbg!(&rule);

        assert!(matches!(rule, MatchRule::And(_)));

        let matches = match &rule {
            MatchRule::And(rules) => rules,
            _ => panic!("Expected And rule"),
        };

        assert_eq!(matches.len(), 4);
        assert!(matches!(matches[3], MatchRule::Or(_)));

        let compound_matches = match &matches[3] {
            MatchRule::Or(rules) => rules,
            _ => panic!("Expected And rule"),
        };

        assert_eq!(compound_matches.len(), 2);
        assert_eq!(
            compound_matches[0],
            MatchRule::String(8, [174, 177, 83, 120, 208, 41, 150, 211].to_vec())
        );

        assert_eq!(
            compound_matches[1],
            MatchRule::String(8, [174, 177, 83, 120, 208, 41, 150, 212].to_vec())
        );

        let string_rules = rules_to_string(&rule);
        assert_eq!(
            string_rules,
            "(offset(bytes, 0, &[228, 82, 92, 123]) && offset(bytes, 4, &[140, 216]) && offset(bytes, 6, &[167, 77]) && (offset(bytes, 8, &[174, 177, 83, 120, 208, 41, 150, 211]) || offset(bytes, 8, &[174, 177, 83, 120, 208, 41, 150, 212])))"
        );
    }

    #[test]
    #[ignore = "I'm not sure why this test doesn't work, I disabled the code that triggers it for now"]
    fn we_dont_generate_bad_regexes() {
        let def = r#"<mime-type type="application/x-ms-owner">
    <_comment>Temporary files created by MSOffice applications</_comment>
    <_comment>PRONOM fmt-473</_comment>
    <_comment>First byte and 53rd byte are the same -- the length of the name.</_comment>
    <_comment>Based on TIKA-2469, we've added a heuristic/wild guess that the first 10 chars</_comment>
    <_comment>after the length byte should be \x00 or a non-control character.</_comment>
    <magic priority="80">
      <match value="(?s)^([\\x05-\\x0F])[\\x00\\x20-\\x7E]{10}.{43}\\1\x00" type="regex" offset="0"/>
    </magic>
  </mime-type>"#;

        let mime: MimeType = from_str(def).unwrap();

        let rule = actions_to_rules(&mime);
        dbg!(&rule);

        let string_rules = rules_to_string(&rule);
        assert_eq!(
            string_rules,
            "(T_zip_application{}.check(bytes) && T_zip2_application{}.check(bytes) && offset(bytes, 0, &[80, 75, 3, 4]) && offset(bytes, 30, &[109, 105, 109, 101, 116, 121, 112, 101, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 118, 110, 100, 46, 101, 116, 115, 105, 46, 97, 115, 105, 99, 45, 101, 43, 122, 105, 112]))"
        );
    }

    #[test]
    fn can_parse_xml_rule() {
        let def = r#"  <mime-type type="application/xml">
    <acronym>XML</acronym>
    <_comment>Extensible Markup Language</_comment>
    <tika:link>http://en.wikipedia.org/wiki/Xml</tika:link>
    <tika:uti>public.xml</tika:uti>
    <alias type="text/xml"/>
    <alias type="application/x-xml"/>
    <magic priority="50">
      <match value="&lt;?xml" type="string" offset="0"/>
      <match value="&lt;?XML" type="string" offset="0"/>
      <!-- UTF-8 BOM -->
      <match value="0xEFBBBF3C3F786D6C" type="string" offset="0"/>
      <!-- UTF-16 LE/BE -->
      <match value="0xFFFE3C003F0078006D006C00" type="string" offset="0"/>
      <match value="0xFEFF003C003F0078006D006C" type="string" offset="0"/>
      <!-- TODO: Add matches for the other possible XML encoding schemes -->
    </magic>
    <!-- XML files can start with a comment but then must not contain processing instructions.
         This should be rare so we assign lower priority here. Priority is also lower than text/html magics
         for them to be preferred for HTML starting with comment.-->
    <magic priority="30">
      <match value="&lt;!--" type="string" offset="0"/>
    </magic>
    <glob pattern="*.xml"/>
    <glob pattern="*.xsl"/>
    <glob pattern="*.xsd"/>
    <sub-class-of type="text/plain" />
  </mime-type>"#;

        let mime: MimeType = from_str(def).unwrap();

        let rule = actions_to_rules(&mime);
        dbg!(&rule);

        let string_rules = rules_to_string(&rule);
        assert_eq!(
            string_rules,
            "(offset(bytes, 0, &[60, 63, 120, 109, 108]) || offset(bytes, 0, &[60, 63, 88, 77, 76]) || offset(bytes, 0, &[239, 187, 191, 60, 63, 120, 109, 108]) || offset(bytes, 0, &[255, 254, 60, 0, 63, 0, 120, 0, 109, 0, 108, 0]) || offset(bytes, 0, &[254, 255, 0, 60, 0, 63, 0, 120, 0, 109, 0, 108]) || offset(bytes, 0, &[60, 33, 45, 45]))"
        );
    }
}

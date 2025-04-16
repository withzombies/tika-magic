use quick_xml::de::from_str;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Offset {
    Start(u32),
    Range { start: u32, end: u32 },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct MimeInfo {
    #[serde(rename = "mime-type")]
    pub mime_types: Vec<MimeType>,
}

#[derive(Debug, Clone)]
pub struct MimeType {
    pub mime_type: String,
    pub globs: Vec<Glob>,
    pub magics: Vec<Magic>,
    pub aliases: Vec<Alias>,
    pub sub_classes: Vec<SubClass>,
    pub root_xml: Vec<RootXml>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Glob {
    #[serde(rename = "@pattern")]
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Magic {
    #[serde(rename = "@priority", default = "default_priority")]
    pub priority: u32,
    #[serde(rename = "match", default)]
    pub matches: Vec<Match>,
}

fn default_priority() -> u32 {
    0
}

fn default_type() -> String {
    "string".to_string()
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Match {
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@type", default = "default_type")]
    pub match_type: String,
    #[serde(rename = "@offset", default, deserialize_with = "parse_offset")]
    pub offset: Option<Offset>,
    #[serde(rename = "@mask", default)]
    pub mask: Option<String>,
    #[serde(rename = "match", default)]
    pub sub_matches: Vec<Match>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Alias {
    #[serde(rename = "@type")]
    pub alias_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SubClass {
    #[serde(rename = "@type")]
    pub class_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RootXml {
    #[serde(rename = "@localName")]
    pub local_name: Option<String>,
    #[serde(rename = "@namespaceURI")]
    pub namespace_uri: Option<String>,
}

// Define custom deserialization for MimeType that can handle interleaved fields
impl<'de> Deserialize<'de> for MimeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MimeTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for MimeTypeVisitor {
            type Value = MimeType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a MIME type structure with interleaved fields")
            }

            fn visit_map<M>(self, mut map: M) -> Result<MimeType, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut mime_type = None;
                let mut globs = Vec::new();
                let mut magics = Vec::new();
                let mut aliases = Vec::new();
                let mut sub_classes = Vec::new();
                let mut root_xml = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "@type" => {
                            mime_type = Some(map.next_value()?);
                        }
                        "glob" => {
                            globs.push(map.next_value()?);
                        }
                        "magic" => {
                            magics.push(map.next_value()?);
                        }
                        "alias" => {
                            aliases.push(map.next_value()?);
                        }
                        "sub-class-of" => {
                            sub_classes.push(map.next_value()?);
                        }
                        "root-XML" => {
                            root_xml.push(map.next_value()?);
                        }
                        _ => {
                            // just eat the rest
                            let _: String = map.next_value()?;
                        }
                    }
                }

                Ok(MimeType {
                    mime_type: mime_type.ok_or_else(|| de::Error::missing_field("@type"))?,
                    globs,
                    magics,
                    aliases,
                    sub_classes,
                    root_xml,
                })
            }
        }

        deserializer.deserialize_map(MimeTypeVisitor)
    }
}

// Custom deserializer for the Offset enum.
fn parse_offset<'de, D>(deserializer: D) -> Result<Option<Offset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Offset::from_str(&s).ok())
}

impl FromStr for Offset {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = input.split_once(':') {
            let start: u32 = start
                .parse()
                .map_err(|_| format!("Invalid start value: {}", start))?;
            let end: u32 = end
                .parse()
                .map_err(|_| format!("Invalid end value: {}", end))?;
            Ok(Offset::Range { start, end })
        } else {
            let start: u32 = input
                .parse()
                .map_err(|_| format!("Invalid single offset: {}", input))?;
            Ok(Offset::Start(start))
        }
    }
}

pub fn parse_mime_type_xml(file_path: &str) -> Result<MimeInfo, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut xml_input = String::new();
    let _ = reader.read_to_string(&mut xml_input);

    from_str(&xml_input).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn can_parse_zip_definition() {
        let def = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <mime-info xmlns:tika="https://tika.apache.org/">
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
                </mime-type>
            </mime-info>
        </xml>"#;

        let mime_types: MimeInfo = from_str(def).unwrap();
        assert_eq!(mime_types.mime_types.len(), 1);
        assert_eq!(mime_types.mime_types[0].mime_type, "application/zip");
        assert_eq!(mime_types.mime_types[0].globs.len(), 2);
        assert_eq!(mime_types.mime_types[0].magics.len(), 1);

        let matches = &mime_types.mime_types[0].magics.first().unwrap().matches;
        assert_eq!(matches.len(), 3);

        dbg!(&matches[0]);

        assert_eq!(matches[0].value, Some("PK\\003\\004".to_string()));
        assert_eq!(matches[1].value, Some("PK\\005\\006".to_string()));
        assert_eq!(matches[2].value, Some("PK\\x07\\x08".to_string()));

        dbg!(&mime_types);

        let globs = &mime_types.mime_types[0].globs;
        assert_eq!(globs[0].pattern, Some("*.zip".to_string()));
        assert_eq!(globs[1].pattern, Some("*.zipx".to_string()));
    }

    #[test]
    fn can_match_coral_draw_submatches() {
        let def = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <mime-info xmlns:tika="https://tika.apache.org/">
                  <mime-type type="application/coreldraw">
     <alias type="application/x-coreldraw"/>
     <alias type="application/x-cdr"/>
     <alias type="application/cdr"/>
     <alias type="image/x-cdr"/>
     <alias type="image/cdr"/>
     <_comment>CorelDraw</_comment>
     <_comment>cdr: CorelDraw</_comment>
     <_comment>des: CorelDraw X4 and newer</_comment>
     <magic priority="60">
        <match value="RIFF" type="string" offset="0">
           <match value="CDR" type="string" />
           <match value="cdr" type="string" offset="8" />
           <match value="DES" type="string" offset="8" />
           <match value="des" type="string" offset="8" />
        </match>
     </magic>
     <glob pattern="*.cdr"/>
  </mime-type>
  </mime-info>
  </xml>"#;

        let mime_types: MimeInfo = from_str(def).unwrap();
        assert_eq!(mime_types.mime_types.len(), 1);
        assert_eq!(mime_types.mime_types[0].mime_type, "application/coreldraw");

        let aliases = &mime_types.mime_types[0].aliases;
        assert_eq!(
            aliases.first().unwrap().alias_type.clone().unwrap(),
            "application/x-coreldraw"
        );

        let magic = mime_types.mime_types[0].magics.first().unwrap();
        let matches = &magic.matches;
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].value, Some("RIFF".to_string()));
        assert_eq!(matches[0].match_type, "string");
        assert_eq!(matches[0].offset, Some(Offset::Start(0)));

        assert_eq!(matches[0].sub_matches.len(), 4);

        dbg!(&mime_types);
    }

    #[test]
    fn can_handle_html() {
        let def = r#"
        <mime-type type="text/html">
    <_comment>HyperText Markup Language</_comment>
    <acronym>HTML</acronym>
    <tika:uti>public.html</tika:uti>
     <!-- TIKA-327: if you encounter tags in the HTML
          with no declared namespace, it's not XHTML, it's just
          bad HTML, unfortunately.
     -->
    <root-XML localName="html"/>
    <root-XML localName="HTML"/>
    <root-XML localName="link"/>
    <root-XML localName="LINK"/>
    <root-XML localName="body"/>
    <root-XML localName="BODY"/>
    <root-XML localName="p"/>
    <root-XML localName="P"/>
    <root-XML localName="script"/>
    <root-XML localName="SCRIPT"/>
    <root-XML localName="frameset"/>
    <root-XML localName="FRAMESET"/>
    <root-XML localName="iframe"/>
    <root-XML localName="IFRAME"/>
    <magic priority="60">
      <match value="(?i)&lt;(html|head|body|title|div)[ >]" type="regex" offset="0"/>
      <match value="(?i)&lt;h[123][ >]" type="regex" offset="0"/>
    </magic>
    <!-- The magic priority needs to be lower than that of -->
    <!--  files which contain HTML within them, eg mime emails -->
    <magic priority="40">
      <match value="&lt;!DOCTYPE HTML" type="string" offset="0:64"/>
      <match value="&lt;!DOCTYPE html" type="string" offset="0:64"/>
      <match value="&lt;!doctype HTML" type="string" offset="0:64"/>
      <match value="&lt;!doctype html" type="string" offset="0:64"/>
      <match value="&lt;HEAD" type="string" offset="0:64"/>
      <match value="&lt;head" type="string" offset="0:64"/>
      <match value="&lt;TITLE" type="string" offset="0:64"/>
      <match value="&lt;title" type="string" offset="0:64"/>
      <match value="&lt;HTML" type="string" offset="0:64"/>
      <match value="&lt;html" type="string" offset="0:128"/>
    </magic>
    <magic priority="20">
      <!-- Lower priority match for <html anywhere near the top of the file -->
      <!-- note on the offset value here: this can only be as big as
           MimeTypes#getMinLength(). If you set the offset value to larger
           than that size, the magic will only be compared to up to
           MimeTypes#getMinLength() bytes. It should also only start after
           the higher priority "start of file" one above
       -->
      <match value="&lt;html" type="string" offset="128:8192"/>
    </magic>
    <glob pattern="*.html"/>
    <glob pattern="*.htm"/>
  </mime-type>
        "#;

        let mime: MimeType = from_str(def).unwrap();
        assert_eq!(mime.mime_type, "text/html");

        let magic = &mime.magics;
        assert_eq!(magic.len(), 3);

        dbg!(&mime);

        let rootxmls = mime.root_xml;
        assert_eq!(rootxmls.len(), 14);

        dbg!(&rootxmls[0]);
    }

    #[test]
    fn can_handle_when_fields_are_interleaved() {
        let xml = r#"
        <mime-info xmlns:tika="https://tika.apache.org/">
            <mime-type type="application/vnd.ms-cab-compressed">
                <magic priority="50">
                    <match value="MSCF\000\000\000\000" type="string" offset="0" />
                </magic>
                <glob pattern="*.cab"/>
                <magic priority="50">
                    <match value="MSCF" type="string" offset="0" />
                </magic>
            </mime-type>
        </mime-info>
    "#;

        let mime_info: MimeInfo = from_str(xml).unwrap();
        assert_eq!(mime_info.mime_types.len(), 1);

        let magics = &mime_info.mime_types[0].magics;
        assert_eq!(magics.len(), 2);
    }

    #[test]
    fn can_handle_no_match_type_also_match_masks() {
        let def = r#"<mime-type type="application/pkcs7-signature">
    <glob pattern="*.p7s"/>
    <magic priority="50">
      <!-- PEM encoded -->
      <match value="-----BEGIN PKCS7" type="string" offset="0"/>
      <!-- DER encoded, sequence+length, object=pkcs7-signedData -->
      <match value="0x3080" offset="0">
         <match value="0x06092a864886f70d0107FFa0" type="string"
                 mask="0xFFFFFFFFFFFFFFFFFFFF00FF" offset="2"/>
      </match>
      <match value="0x3081" offset="0">
         <match value="0x06092a864886f70d0107FFa0" type="string"
                 mask="0xFFFFFFFFFFFFFFFFFFFF00FF" offset="3"/>
      </match>
      <match value="0x3082" offset="0">
         <match value="0x06092a864886f70d0107FFa0" type="string"
                 mask="0xFFFFFFFFFFFFFFFFFFFF00FF" offset="4"/>
      </match>
      <match value="0x3083" offset="0">
         <match value="0x06092a864886f70d0107FFa0" type="string"
                 mask="0xFFFFFFFFFFFFFFFFFFFF00FF" offset="5"/>
      </match>
      <match value="0x3084" offset="0">
         <match value="0x06092a864886f70d0107FFa0" type="string"
                 mask="0xFFFFFFFFFFFFFFFFFFFF00FF" offset="6"/>
      </match>
    </magic>
  </mime-type>"#;

        let mime_info: MimeType = from_str(def).unwrap();

        assert_eq!(mime_info.mime_type, "application/pkcs7-signature");
        let magics = mime_info.magics;
        let magic = &magics[0];
        let matches = &magic.matches;
        let match_1 = &matches[1];
        let sub_matches = &match_1.sub_matches;
        let sub_match_0 = &sub_matches[0];
        assert_eq!(sub_match_0.match_type, "string");
        assert_eq!(sub_match_0.offset, Some(Offset::Start(2)));
        assert_eq!(
            sub_match_0.mask,
            Some("0xFFFFFFFFFFFFFFFFFFFF00FF".to_string())
        );
    }
}

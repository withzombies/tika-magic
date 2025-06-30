mod tests {
    use rstest::rstest;
    use tika_magic::from_u8;

    #[rstest]
    #[case("3gpp2", b"\x00\x00\x00\x18ftyp3g24", "video/3gpp2")]
    #[case(
        "3gpp2 without ftyp",
        b"\x00\x00\x00\x18mtyp3g24",
        "application/octet-stream"
    )]
    #[case("3gp", b"\x00\x00\x00\x18ftyp3gp1", "video/3gpp")]
    #[case("3mf",b"<?xml version=\"1.0\"?><model xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">", "application/vnd.ms-package.3dmanufacturing-3dmodel+xml")]
    #[case("7z", b"\x37\x7A\xBC\xAF\x27\x1C", "application/x-7z-compressed")]
    #[case("a", b"\x21\x3C\x61\x72\x63\x68\x3E", "application/x-archive")]
    #[case("aac 1", b"\xff\xf9", "audio/x-aac")]
    #[case("aac 2", b"\xff\xf1", "audio/x-aac")]
    #[case(
        "aiff",
        b"\x46\x4F\x52\x4D\x00\x00\x00\x00\x41\x49\x46\x46\x00",
        "audio/x-aiff"
    )]
    #[case("amf", b"<?xml version=\"1.0\"?><amf >", "application/x-amf")]
    #[case("amr", b"\x23\x21\x41\x4D\x52", "audio/amr")]
    #[case(
        "ape",
        b"\x4D\x41\x43\x20\x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3",
        "audio/ape"
    )]
    #[case(
        "asf",
        b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C",
        "video/x-ms-asf"
    )]
    #[case(
        "atom",
        b"<?xml version=\"1.0\"?><feed xmlns=\"http://www.w3.org/2005/Atom\">",
        "application/atom+xml"
    )]
    #[case("au", b"\x2E\x73\x6E\x64\x00\x00\x00", "audio/basic")]
    #[case("avi", b"RIFF\x00\x00\x00\x00AVI LIST\x00", "video/x-msvideo")]
    #[case("avif", b"\x00\x00\x00\x18ftypavif", "image/avif")]
    #[case("avis", b"\x00\x00\x00\x18ftypavis", "image/avif")]
    #[case("bmp", b"\x42\x4d\x1a\x58\x00\x00\x00\x00\x00\x00\x36\x00\x00\x00\x28\x00\x00\x00\x64\x00\x00\x00\x4b\x00\x00\x00\x01\x00\x18\x00\x00\x00\x00\x00\xe4\x57\x00\x00\x13\x0b\x00\x00\x13\x0b\x00\x00\x00\x00", "image/bmp")]
    #[case("bpg", b"\x42\x50\x47\xFB", "image/x-bpg")]
    #[case("bz2", b"\x42\x5A\x681", "application/x-bzip2")]
    #[case("cab", b"MSCF\x00\x00\x00\x00", "application/vnd.ms-cab-compressed")]
    #[case("cab.is", b"ISc(\x00\x00\x00\x01", "application/x-installshield")]
    #[case("class", b"\xCA\xFE\xBA\xBE\x00\x00\x00\xFF", "application/java-vm")]
    //#[case("csv", b"1,2,3,4\n5,6,7,8\na,b,c,d", "text/csv")]
    #[case("cpio 7", b"070707", "application/x-cpio")]
    #[case("cpio 1", b"070701", "application/x-cpio")]
    #[case("cpio 2", b"070702", "application/x-cpio")]
    #[case(
        "dae",
        b"<?xml version=\"1.0\"?><COLLADA xmlns=\"http://www.collada.org/2005/11/COLLADASchema\">",
        "model/vnd.collada+xml"
    )]
    //#[case("dbf", b"\x03\x5f\x07\x1a\x96\x0f\x00\x00\xc1\x00\xa3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x6f\x73\x6d\x5f\x69\x64\x00\x00\x00\x00\x00\x43\x00\x00\x00\x00\x0a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x63\x6f\x64\x65", "application/x-dbf")]
    #[case(
        "deb",
        b"\x21\x3c\x61\x72\x63\x68\x3e\x0a\x64\x65\x62\x69\x61\x6e\x2d\x62\x69\x6e\x61\x72\x79",
        "application/x-debian-package"
    )]
    #[case(
        "djvu",
        b"\x41\x54\x26\x54\x46\x4F\x52\x4D\x00\x00\x00\x00DJVU",
        "image/vnd.djvu"
    )]
    #[case(
        "djvuM",
        b"\x41\x54\x26\x54\x46\x4F\x52\x4D\x00\x00\x00\x00DJVM",
        "image/vnd.djvu"
    )]
    #[case(
        "djvuI",
        b"\x41\x54\x26\x54\x46\x4F\x52\x4D\x00\x00\x00\x00DJVI",
        "image/vnd.djvu"
    )]
    #[case(
        "djvuTHUM",
        b"\x41\x54\x26\x54\x46\x4F\x52\x4D\x00\x00\x00\x00THUM",
        "image/vnd.djvu"
    )]
    #[case("rpm 1", b"\xed\xab\xee\xdb", "application/x-rpm")]
    #[case("rpm 2", b"drpm", "application/x-rpm")]
    #[case("dwg", b"\x41\x43\x31\x30\x32\x34", "image/vnd.dwg")]
    #[case("eot", b"\xbe\x45\x00\x00\xfa\x44\x00\x00\x02\x00\x02\x00\x04\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x90\x01\x00\x00\x00\x00\x4c\x50", "application/vnd.ms-fontobject")]
    #[case("fdf", b"%FDF-", "application/vnd.fdf")]
    #[case("fits", b"\x53\x49\x4d\x50\x4c\x45\x20\x20\x3d\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x54", "image/fits")]
    #[case("flac", b"\x66\x4C\x61\x43\x00\x00\x00\x22", "audio/x-flac")]
    #[case("flv", b"\x46\x4C\x56\x01", "video/x-flv")]
    #[case("gif 87", b"GIF87a", "image/gif")]
    #[case("gif 89", b"GIF89a", "image/gif")]
    #[case("glb 1", b"\x67\x6C\x54\x46\x02\x00\x00\x00", "model/gltf-binary")]
    #[case("glb 2", b"\x67\x6C\x54\x46\x01\x00\x00\x00", "model/gltf-binary")]
    #[case(
        "gml",
        b"<?xml version=\"1.0\"?><any xmlns:gml=\"http://www.opengis.net/gml\">",
        "application/gml+xml"
    )]
    #[case(
        "gml3.2",
        b"<?xml version=\"1.0\"?><any xmlns:gml=\"http://www.opengis.net/gml/3.2\">",
        "application/gml+xml"
    )]
    #[case(
        "gml3.3",
        b"<?xml version=\"1.0\"?><any xmlns:gml=\"http://www.opengis.net/gml/3.3/exr\">",
        "application/gml+xml"
    )]
    #[case(
        "gpx",
        b"<?xml version=\"1.0\"?><gpx xmlns=\"http://www.topografix.com/GPX/1/1\">",
        "application/gpx+xml"
    )]
    #[case("gz", b"\x1F\x8B", "application/gzip")]
    #[case("hdr", b"#?RADIANCE\n", "image/vnd.radiance")]
    #[case("heic", b"\x00\x00\x00\x18ftypheic", "image/heic")]
    #[case("heix", b"\x00\x00\x00\x18ftypheix", "image/heic")]
    #[case("heif mif1", b"\x00\x00\x00\x18ftypmif1", "image/heif")]
    #[case("heif heim", b"\x00\x00\x00\x18ftypheim", "image/heif")]
    #[case("heif heis", b"\x00\x00\x00\x18ftypheis", "image/heif")]
    #[case("heif avic", b"\x00\x00\x00\x18ftypavic", "image/heif")]
    #[case("html", b"<HtMl><bOdY>blah blah blah</body></html>", "text/html")]
    #[case("html empty", b"<HTML></HTML>", "text/html")]
    #[case("html just header", b"   <!DOCTYPE HTML>...", "text/html")]
    #[case("line ending before html", b"\r\n<html>...", "text/html")]
    #[case(
        "html with encoding",
        b"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=iso-8859-1\">",
        "text/html"
    )]
    #[case("ico 01", b"\x00\x00\x01\x00", "image/vnd.microsoft.icon")]
    #[case("ico 02", b"\x00\x00\x02\x00", "image/vnd.microsoft.icon")]
    #[case("ics", b"BEGIN:VCALENDAR\nVERSION:2.0", "text/calendar")]
    #[case("ics dos", b"BEGIN:VCALENDAR\r\nVERSION:2.0", "text/calendar")]
    #[case("jp2", b"\x00\x00\x00\x0c\x6a\x50\x20\x20\x0d\x0a\x87\x0a\x00\x00\x00\x14\x66\x74\x79\x70\x6a\x70\x32\x20", "image/jp2")]
    #[case("jpf", b"\x00\x00\x00\x0c\x6a\x50\x20\x20\x0d\x0a\x87\x0a\x00\x00\x00\x1c\x66\x74\x79\x70\x6a\x70\x78\x20", "image/jpx")]
    #[case("jpg", b"\xFF\xD8\xFF", "image/jpeg")]
    #[case("jpm", b"\x00\x00\x00\x0c\x6a\x50\x20\x20\x0d\x0a\x87\x0a\x00\x00\x00\x14\x66\x74\x79\x70\x6a\x70\x6d\x20", "image/jpm")]
    #[case("jxl 1", b"\xFF\x0A", "image/jxl")]
    #[case("jxl 2", b"\x00\x00\x00\x0cJXL\x20\x0d\x0a\x87\x0a", "image/jxl")]
    #[case("jxr", b"\x49\x49\xBC\x01", "image/jxr")]
    #[case("xpm", b"\x2F\x2A\x20\x58\x50\x4D\x20\x2A\x2F", "image/x-xpixmap")]
    #[case("js", b"#!/bin/node ", "text/javascript")]
    #[case(
        "kml 2.2",
        b"<?xml version=\"1.0\"?><kml xmlns=\"http://www.opengis.net/kml/2.2\">",
        "application/vnd.google-earth.kml+xml"
    )]
    #[case(
        "kml 2.0",
        b"<?xml version=\"1.0\"?><kml xmlns=\"http://earth.google.com/kml/2.0\">",
        "application/vnd.google-earth.kml+xml"
    )]
    #[case(
        "kml 2.1",
        b"<?xml version=\"1.0\"?><kml xmlns=\"http://earth.google.com/kml/2.1\">",
        "application/vnd.google-earth.kml+xml"
    )]
    #[case(
        "kml 2.2",
        b"<?xml version=\"1.0\"?><kml xmlns=\"http://earth.google.com/kml/2.2\">",
        "application/vnd.google-earth.kml+xml"
    )]
    #[case("lit", b"ITOLITLS", "application/x-ms-reader")]
    #[case("lua", b"#!/usr/bin/lua", "text/x-lua")]
    #[case("lua space", b"#! /usr/bin/lua", "text/x-lua")]
    #[case("lz", b"\x4c\x5a\x49\x50", "application/x-lzip")]
    #[case("m3u", b"#EXTM3U", "application/vnd.apple.mpegurl")]
    #[case("m4a", b"\x00\x00\x00\x18ftypM4A ", "audio/mp4")]
    #[case("audio mp4 F4A", b"\x00\x00\x00\x18ftypF4A ", "audio/mp4")]
    #[case("audio mp4 F4B", b"\x00\x00\x00\x18ftypF4B ", "audio/mp4")]
    #[case("audio mp4 M4B", b"\x00\x00\x00\x18ftypM4B ", "audio/mp4")]
    #[case("audio mp4 M4P", b"\x00\x00\x00\x18ftypM4P ", "audio/mp4")]
    #[case("audio mp4 MSNV", b"\x00\x00\x00\x18ftypMSNV", "audio/mp4")]
    #[case("audio mp4 NDAS", b"\x00\x00\x00\x18ftypNDAS", "audio/mp4")]
    #[case(
        "lnk",
        b"\x4C\x00\x00\x00\x01\x14\x02\x00",
        "application/x-ms-shortcut"
    )]
    #[case("midi", b"\x4D\x54\x68\x64", "audio/midi")]
    #[case("mkv", b"\x1a\x45\xdf\xa3\x01\x00\x00\x00\x00\x00\x00\x23\x42\x86\x81\x01\x42\xf7\x81\x01\x42\xf2\x81\x04\x42\xf3\x81\x08\x42\x82\x88\x6d\x61\x74\x72\x6f\x73\x6b\x61", "application/x-matroska")]
    #[case(
        "mov",
        b"\x00\x00\x00\x14\x66\x74\x79\x70\x71\x74\x20\x20",
        "video/quicktime"
    )]
    #[case("mp3", b"\x49\x44\x33\x03", "audio/mpeg")]
    #[case("mp3 v1 notag", b"\xff\xfb\xc8\x00", "audio/mpeg")]
    #[case("mp3 v2.5 notag", b"\xff\xe3\x18\xc4", "audio/mpeg")]
    #[case("mp3 v2 notag", b"\xff\xf3\x82\xc4", "audio/mpeg")]
    #[case("mp4 1", b"\x00\x00\x00\x18ftyp0000", "video/mp4")]
    #[case("mpc", b"MPCK", "audio/musepack")]
    #[case("mpeg", b"\x00\x00\x01\xba", "video/mpeg")]
    #[case("mqv", b"\x00\x00\x00\x18ftypmqt ", "video/quicktime")]
    #[case("nes", b"NES\x1a", "application/x-nesrom")]
    #[case(
        "elfobject",
        b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00",
        "application/x-object"
    )]
    #[case("sxc", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xbb\x03\x5eGE\xbc\x13\x94\x1c\x00\x00\x00\x1c\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.sun.xml.calc", "application/vnd.sun.xml.calc")]
    #[case("odg", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xcbY\xa8N\x9f\x03.\xc4\x2b\x00\x00\x00\x2b\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.graphics", "application/vnd.oasis.opendocument.graphics")]
    #[case("odp", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xbdX\xa8N3&\xac\xa8/\x00\x00\x00/\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.presentation", "application/vnd.oasis.opendocument.presentation")]
    #[case("ods", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\x14V\xa8N\x85l9\x8a.\x00\x00\x00.\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.spreadsheet", "application/vnd.oasis.opendocument.spreadsheet")]
    #[case("odt", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xbbP\xa8N\x5e\xc62\n'\x00\x00\x00'\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.text", "application/vnd.oasis.opendocument.text")]
    #[case("ogg", b"OggS\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\xce\xc6AI\x00\x00\x00\x00py\xf3\x3d\x01\x1e\x01vorbis\x00\x00", "audio/vorbis")]
    #[case("ogg", b"OggS\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x80\xbc\x81_\x00\x00\x00\x00\xd0\xfbP\x84\x01@fishead\x00\x03", "video/ogg")]
    #[case("ogg spx oga", b"OggS\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\xc7w\xaa\x15\x00\x00\x00\x00V&\x88\x89\x01PSpeex   1", "audio/speex")]
    #[case("otf", b"OTTO\x00", "application/x-font-otf")]
    #[case("otg", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xd1Y\xa8N\xdf%\xad\xe94\x00\x00\x004\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.graphics-template", "application/vnd.oasis.opendocument.graphics-template")]
    #[case("otp", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xc4X\xa8N\xef\n\x14:8\x00\x00\x008\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.presentation-template", "application/vnd.oasis.opendocument.presentation-template")]
    #[case("ots", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\x1bV\xa8N{\x96\xa3N7\x00\x00\x007\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.spreadsheet-template", "application/vnd.oasis.opendocument.spreadsheet-template")]
    #[case("ott", b"PK\x03\x04\x14\x00\x00\x08\x00\x00\xcfP\xa8N\xe4\x11\x92)0\x00\x00\x000\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.text-template", "application/vnd.oasis.opendocument.text-template")]
    #[case("odc", b"PK\x03\x04\x14\x00\x00\x08\x00\x00zp2R\xab\xb8\xb2l(\x00\x00\x00(\x00\x00\x00\x08\x00\x00\x00mimetypeapplication/vnd.oasis.opendocument.chart", "application/vnd.oasis.opendocument.chart")]
    #[case(
        "owl",
        b"<?xml version=\"1.0\"?><Ontology xmlns=\"http://www.w3.org/2002/07/owl#\">",
        "application/owl+xml"
    )]
    #[case(
        "pat",
        b"\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03GPAT",
        "image/x-gimp-pat"
    )]
    #[case("pdf", b"%PDF-", "application/pdf")]
    #[case("php", b"#!/usr/bin/env php", "text/x-php")]
    #[case("pl", b"#!/usr/bin/perl", "text/x-perl")]
    #[case("png", b"\x89PNG\x0d\x0a\x1a\x0a", "image/png")]
    #[case("ps", b"%!PS-Adobe-", "application/postscript")]
    #[case("psd", b"8BPS\x00\x01", "image/vnd.adobe.photoshop")]
    #[case("p7s_pem", b"-----BEGIN PKCS7", "application/pkcs7-signature")]
    #[case(
        "p7s_der",
        b"\x30\x82\x01\x26\x06\x09\x2a\x86\x48\x86\xf7\x0d\x01\x07\x02\xa0\x82\x01\x17\x30",
        "application/pkcs7-signature"
    )]
    #[case("py", b"#!/usr/bin/python", "text/x-python")]
    #[case("qcp", b"RIFF\xc0\xcf\x00\x00QLCMf", "audio/qcelp")]
    #[case(
        "rar",
        b"Rar!\x1a\x07\x01\x00",
        "application/x-rar-compressed;version=5"
    )]
    #[case("rmvb", b".RMF", "application/vnd.rn-realmedia")]
    #[case("rpm", b"\xed\xab\xee\xdb", "application/x-rpm")]
    #[case(
        "rss",
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss ",
        "application/rss+xml"
    )]
    #[case("rtf", b"{\\rtf", "application/rtf")]
    #[case(
        "so",
        b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00",
        "application/x-sharedlib"
    )]
    #[case("sqlite", b"SQLite format 3\x00", "application/x-sqlite3")]
    #[case(
        "srt",
        b"1\n00:02:16,612 --\x3e 00:02:19,376\nS",
        "application/x-subrip"
    )]
    #[case("svg", b"<svg", "image/svg+xml")]
    #[case("swf", b"CWS", "application/x-shockwave-flash")]
    #[case("tcl", b"#!/usr/bin/tcl", "text/x-tcl")]
    #[case("tcx", b"<?xml version=\"1.0\"?><TrainingCenterDatabase xmlns=\"http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2\">", "application/vnd.garmin.tcx+xml")]
    #[case("tiff", b"II*\x00", "image/tiff")]
    #[case("ttc", b"ttcf\x00\x01\x00\x00", "font/collection")]
    #[case("ttf", b"\x00\x01\x00\x00", "application/x-font-ttf")]
    #[case(
        "utf16bebom txt",
        b"\xfe\xff\x00\x74\x00\x68\x00\x69\x00\x73",
        "text/plain"
    )]
    #[case(
        "utf16lebom txt",
        b"\xff\xfe\x74\x00\x68\x00\x69\x00\x73\x00",
        "text/plain"
    )]
    #[case(
        "utf32lebom txt",
        b"\xff\xfe\x00\x00\x74\x00\x00\x00\x68\x00\x00\x00\x69\x00\x00\x00\x73\x00\x00\x00",
        "text/plain"
    )]
    #[case(
        "utf8ctrlchars",
        b"\xef\xbf\xbd\xef\xbf\xbd\xef\xbf\xbd\xef\xbf\xbd\xef\xbf\xbd\x10",
        "application/octet-stream"
    )]
    #[case("vcf", b"BEGIN:VCARD\nV", "text/x-vcard")]
    #[case("vcf dos", b"BEGIN:VCARD\r\nV", "text/x-vcard")]
    #[case("voc", b"Creative Voice File", "audio/x-unknown")]
    #[case("vtt", b"WEBVTT\n", "text/vtt")]
    #[case("warc", b"WARC/1.1", "application/warc")]
    #[case("wasm", b"\x00asm", "application/wasm")]
    #[case("wav", b"RIFF\xba\xa5\x04\x00WAVEf", "audio/vnd.wave")]
    #[case("webm", b"\x1aE\xdf\xa3\x01\x00\x00\x00\x00\x00\x00\x1fB\x86\x81\x01B\xf7\x81\x01B\xf2\x81\x04B\xf3\x81\x08B\x82\x84webm", "video/webm")]
    #[case("webp", b"RIFFhv\x00\x00WEBPV", "image/webp")]
    #[case("woff", b"wOFF", "font/woff")]
    #[case("woff2", b"wOF2", "font/woff2")]
    #[case(
        "x3d",
        b"<?xml version=\"1.0\"?><X3D xmlns:xsd=\"http://www.w3.org/2001/XMLSchema-instance\">",
        "model/x3d+xml"
    )]
    #[case("xar", b"xar!", "application/vnd.xara")]
    #[case("xcf", b"gimp xcf ", "image/x-xcf")]
    #[case(
        "xfdf",
        b"<?xml version=\"1.0\"?><xfdf xmlns=\"http://ns.adobe.com/xfdf/\">",
        "application/vnd.adobe.xfdf"
    )]
    #[case(
        "xlf",
        b"<?xml version=\"1.0\"?><xliff xmlns=\"urn:oasis:names:tc:xliff:document:1.2\">",
        "application/x-xliff+xml"
    )]
    #[case("xml", b"<?xml ", "application/xml")]
    #[case("xml withbr", b"\x0D\x0A<?xml ", "application/xml")]
    #[case("xz", b"\xfd7zXZ\x00", "application/x-xz")]
    #[case("zip", b"PK\x03\x04", "application/zip")]
    #[case("zst", b"(\xb5/\xfd", "application/zstd")]
    #[case("zst skippable frame", b"\x50\x2A\x4D\x18", "application/zstd")]
    fn test_from_u8_cases(#[case] name: &str, #[case] bytes: &[u8], #[case] expected_mime: &str) {
        //assert!(match_u8(expected_mime, bytes));

        let actual_mime = from_u8(bytes);
        assert_eq!(
            actual_mime, expected_mime,
            "Test:{name} failed Input: {bytes:?}, Expected: {expected_mime}, Got: {actual_mime}" // Added debug info on failure
        );
    }
}

/*
("accdb", offset(4, "Standard ACE DB"), "application/x-msaccess"), // false because accdb and mdb share the same MIME
("apng", b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"# + offset(29, "acTL"), "image/vnd.mozilla.apng"),
("crx", r#"Cr24\x00\x00\x00\x00\x01\x00\x00\x00\x0F\x00\x00\x00"# + offset(16, "") + b"\x50\x4B\x03\x04"#, "application/x-chrome-extension",},
("dcm", offset(128, b"\x44\x49\x43\x4D"#), "application/dicom"),
("doc", fromDisk("doc.doc"), "application/msword"),
("docx", fromDisk("docx.docx"), "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
("epub", b"\x50\x4B\x03\x04"# + offset(26, "mimetypeapplication/epub+zip"), "application/epub+zip"),
("gbr", offset(20, "GIMP"), "image/x-gimp-gbr"),
("jar", fromDisk("jar.jar"), "application/jar"),
("mdb", offset(4, "Standard Jet DB"), "application/x-msaccess"),
("mobi", offset(60, "BOOKMOBI"), "application/x-mobipocket-ebook"),
("msi", fromDisk("msi.msi"), "application/x-ms-installer"),
("msg", fromDisk("msg.msg"), "application/vnd.ms-outlook"),
("ppt", fromDisk("ppt.ppt"), "application/vnd.ms-powerpoint"),
("pptx", fromDisk("pptx.pptx"), "application/vnd.openxmlformats-officedocument.presentationml.presentation"),
("pub", fromDisk("pub.pub"), "application/vnd.ms-publisher"),
("shp", fromDisk("shp.shp"), "application/vnd.shp"),
("tar", fromDisk("tar.tar"), "application/x-tar"),
("tzfile", fromDisk("tzfile"), "application/tzif"),
("utf8 txt", fromDisk("utf8.txt"), "text/plain; charset=utf-8"),
("xls", fromDisk("xls.xls"), "application/vnd.ms-excel"),
("xlsx", fromDisk("xlsx.xlsx"), "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),

 */

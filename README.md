[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Status][test-action-image]][test-action-link]
[![Apache 2.0 Licensed][license-apache-image]][license-apache-link]

# tika-magic

tika-magic is a Rust library that determines the MIME type of a file or byte array. tika-magic is meant to be an API 
compatible with the fantastic [tree_magic_mini](https://github.com/mbrubeck/tree_magic/) crate, but without a dependency
on the system magic file database (which is GPL).

tika-magic uses the [Apache Tika](http://tika.apache.org) mimetypes library to provide an Apache 2.0 licensed MIME 
detection library.

## About tika-magic

`tika-magic` was created due to system differences in the system magic database causing inconsistency in down-stream 
software. Unfortunately, the `libmagic` magic database is licensed GPL which prevents many developers from being able
to use it or distribute software using it. It's not a great UX to require your users to keep their magic file updated
to keep your application working smoothly!

Several other projects have gone down this route, most famously the Ruby on Rails project had to 
[remove](https://www.theregister.com/2021/03/25/ruby_rails_code/) and rewrite their mime type handling code because of 
the license conflict. They created the [Marcel](https://github.com/rails/marcel) library, also based on Apache Tika's rule
definitions to replace the dependency on libmagic. Go has a similar mime detection library called 
[go-mimetype](https://github.com/gabriel-vasile/mimetype). I've taken some design inspiration from them as well as taking
their test inputs.

## Using tika-magic

### API Examples
tika-magic provides several ways to detect MIME types from files or byte arrays:
``` rust
use std::fs::File;
use std::path::Path;
use tika_magic;

// Detect MIME type from a byte array
let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG file signature
let mime_type = tika_magic::from_u8(&data);
assert_eq!(mime_type, "image/png");

// Check if bytes match a specific MIME type
let is_png = tika_magic::match_u8("image/png", &data);
assert!(is_png);

// Get all possible MIME types (ordered by confidence)
let mime_types = tika_magic::from_u8_exhaustive(&data);
println!("Possible MIME types: {:?}", mime_types);

// File-based detection
let file = File::open("example.png").unwrap();
let mime_type = tika_magic::from_file(&file).unwrap();
assert_eq!(mime_type, "image/png");

// Path-based detection
let mime_type = tika_magic::from_filepath(Path::new("example.pdf")).unwrap();
assert_eq!(mime_type, "application/pdf");

// Check if a file matches a specific MIME type
let is_pdf = tika_magic::match_filepath("application/pdf", Path::new("example.pdf"));
assert!(is_pdf);
```
## Installation
Add tika-magic to your `Cargo.toml`:
``` toml
[dependencies]
tika-magic = "0.1.0"
```
Then include it in your Rust project:
``` rust
use tika_magic;
```
The library has minimal dependencies and doesn't require any system libraries or external resources to work - all the MIME detection rules are bundled with the crate.
## License
tika-magic is licensed under the Apache License, Version 2.0. See the LICENSE file for the full license text.
``` 
Copyright 2023 [Your Name or Organization]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

The MIME type detection rules are derived from the [Apache Tika](http://tika.apache.org) project, which is also licensed under the Apache License 2.0.

## Speed

`tika-magic` is slower than `tree_magic_mini`, as `tree_magic_mini` is specifically optimized for quick parsing.

```
test tika-magic::from_u8::application_zip           ... bench:   3,088,086 ns/iter (+/- 340,938)
test tika-magic::from_u8::image_gif                 ... bench:     441,894 ns/iter (+/- 36,948)
test tika-magic::from_u8::image_png                 ... bench:     424,299 ns/iter (+/- 26,686)
test tika-magic::from_u8::text_plain                ... bench:   3,587,062 ns/iter (+/- 535,857)
test tika-magic::match_u8::application_zip          ... bench:          14 ns/iter (+/- 2)
test tika-magic::match_u8::image_gif                ... bench:          14 ns/iter (+/- 1)
test tika-magic::match_u8::image_png                ... bench:          14 ns/iter (+/- 0)
test tika-magic::match_u8::text_plain               ... bench:          15 ns/iter (+/- 0)


test tree_magic_mini::from_u8::application_zip      ... bench:       5,364 ns/iter (+/- 524)
test tree_magic_mini::from_u8::image_gif            ... bench:       1,567 ns/iter (+/- 90)
test tree_magic_mini::from_u8::image_png            ... bench:       1,848 ns/iter (+/- 73)
test tree_magic_mini::from_u8::text_plain           ... bench:      27,507 ns/iter (+/- 2,296)
test tree_magic_mini::match_u8::application_zip     ... bench:          37 ns/iter (+/- 2)
test tree_magic_mini::match_u8::image_gif           ... bench:          28 ns/iter (+/- 1)
test tree_magic_mini::match_u8::image_png           ... bench:          27 ns/iter (+/- 1)
test tree_magic_mini::match_u8::text_plain          ... bench:          16 ns/iter (+/- 1)
```

If you can afford to use the system magic database or to distribute GPL software, `tree_magic_mini` is significantly 
faster. Something for `tika-magic` to improve on!

[//]: # (links)

[crate-image]: https://img.shields.io/crates/v/tika-magic.svg

[crate-link]: https://crates.io/crates/tika-magic

[docs-image]: https://docs.rs/tika-magic/badge.svg

[docs-link]: https://docs.rs/tika-magic/

[test-action-image]: https://github.com/withzombies/tika-magic/workflows/CI/badge.svg

[test-action-link]: https://github.com/withzombies/tika-magic/actions?query=workflow:Test

[license-apache-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg

[license-apache-link]: http://www.apache.org/licenses/LICENSE-2.0
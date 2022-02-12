# social-image

[![Crates.io](https://img.shields.io/crates/v/social-image.svg)](https://crates.io/crates/social-image)
[![CI](https://github.com/clord/social-image/workflows/CI/badge.svg)](https://github.com/clord/social-image/actions)

Post SVGs and then request renderings in other formats (currently only PNG)

This is a very small helper tool that renders SVG to PNG,
including resources like images and fonts. The resulting PNG
can be used for social images or other live content.

Updating the SVG or any of its resources will trigger an update to the PNG.

## Usage

- `GET /` → help content
- `POST /images` → POST SVG for render (see help content above for instructions)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* Run `cargo install social-image`

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

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

## Environment Variables

- `APP_ADDRESS` IP address to serve on (default 127.0.0.1)
- `APP_CLI_COLORS` Whether to use colors and emoji when logging. (default true)
- `APP_IDENT` If and how to identify via the Server header.
- `APP_KEEP_ALIVE` Keep-alive timeout seconds; disabled when 0.(default 5)
- `APP_KEY` is the secret required to use API
- `APP_LOG_LEVEL` one of `critical`, `support`, `normal`, `debug`, `off`
  (default `critical`)
- `APP_PORT` Port to serve on (default 8000)
- `APP_TEMP_PATH` is path to where work temporary files will be kept. (default /tmp)
- `APP_WORKERS` Number of threads to use (default CPU core count)

## Installation

### Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- Run `cargo install social-image`

## License

At your option licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

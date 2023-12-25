# xmc4300

> THIS IS A WORK IN PROGRESS AND MUCH IS UNTESTED

[![crates.io](https://img.shields.io/crates/v/xmc4300.svg)](https://crates.io/crates/xmc4300)
[![rust](https://github.com/xmc-rs/xmc4300/workflows/Rust/badge.svg)](https://github.com/xmc-rs/xmc4300/workflows/Rust/badge.svg)

This is a 'peripheral access crate' for interfacing to the XMC4300 series of microcontrollers for embedded support in Rust that is generated using [svd2rust](https://docs.rs/svd2rust) and an SVD file provided by Infineon.

All API's and usage (besides what registers exist) are defined by [svd2rust](https://docs.rs/svd2rust)

## Generate Crate from SVD

```bash
# Necessary 3rd-party tools
cargo install svd2rust
cargo install form
rustup component add rustfmt

svd.sh # Generates code from crate and formats to rustfmt
```

The inclusion of EtherCAT has been removed from `src/lib.rs`. This is due to an issue either with the SVD file or the generation of the code using svd2rust. There is no support at this time until a fix can be made. This has not been tested on hardware to ensure that nothing else has been impacted.

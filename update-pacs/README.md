# RA PACs

Rust PACs for the Renesas RA Microcontrollers devices with FSP support for runtime.

Fork of https://github.com/ra-rs/ra/commit/16bafa98f2ed5c4dfa3fb7cf973e58e7a471ef9f.

How to run (argument is a version to give to generated PACs):

```bash
cargo run -- 0.1.0
```

## License

By using this software, you agree to the additonal terms and conditions found at: http://www.renesas.com/disclaimer.

Contents of `svd/` and `pac/` directories are licensed by [Renesas](https://github.com/renesas/fsp/blob/master/LICENSE.md).

All Rust source code except `pac/` directory is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.

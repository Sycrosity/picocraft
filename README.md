`picocraft`
==================

picocraft ~~is~~ (hopefully will become) a no_std lightweight minecraft server implementated from scratch, with simplified mechanics and gameplay targeting embedded devices. The server generates miniature (e.g. 256x256, 128x128) worlds with a number of small biomes.

-------

## Current Project Status

As of right now, `picocraft` can handle Status requests from clients, with support for `std` targets - the underlying `picocraft_proto` and `picocraft_core` are fully `no_std`.

## Usage

To run `picocraft`, you must have rust installed via [rustup](https://rustup.rs).

To run the server on default port 25565 (std targets only for now), run this command:

```bash
$ cargo run --release
```

You can also install the server binary from <https://crates.io/crates/picocraft>:

```bash
$ cargo install picocraft --locked
```

Note: this may not be up to date, and installing from this repo's main branch may be more featureful:
```bash
$ cargo install --git https://github.com/Sycrosity/picocraft --locked
```

-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo fmt`.

-------

## Credits

This project wouldn't be possible without learning from the amazing work put in by the following projects:
* [PicoLimbo](https://picolimbo.quozul.dev/) - An ultra-lightweight, multi-version Minecraft limbo server written in Rust, actively maintained and updated to the latest version
* [valence](https://github.com/valence-rs/valence) - A Rust framework for building Minecraft: Java Edition servers. Valence hasn't had a major update for a while now.
* [minecraft.wiki](https://minecraft.wiki/) - The best source of information for minecraft mechanics and inner workings (and especially packets, since the [~~wiki.vg~~](https://wiki.vg/) merge)

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

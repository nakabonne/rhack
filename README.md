# cargo-rhack

[![Version info](https://img.shields.io/crates/v/cargo-rhack.svg)](https://crates.io/crates/cargo-rhack)

## Description

You want to quickly put a sneaky macro kind of like `dbg!` into external crates
to find out how some internal data structure works? If so `cargo-rhack` is for
you!

`cargo-rhack` makes it easier to edit external crates code that your project
depends on.

## Usage

Let's say you want to modify the `reqwest` crate.

```toml
[dependencies]
reqwest = "0.11"
```

Run the following:

```sh
cargo rhack edit reqwest
```

This will make a copy of the crate into `$HOME/.rhack/reqwest-0.11.1` and add
its path to the
[[patch] section](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html)
in your Cargo.toml whose path is automatically detected:

```toml
[patch.crates-io]
reqwest = { path = "/home/you/.rhack/reqwest-0.11.1" }
```

Now your package uses the locally checked out copy instead of one from
crates.io. You can now open the files (typically by leveraging the "Jump to
Definition" feature) and then feel free to modify the source code.

### Undoing

Simply run the `undo` command then you can undo all of the changes to your
Cargo.toml:

```sh
cargo rhack undo
```

Keep in mind that this command doesn't remove any copy of crates.

### Settings

It uses `$HOME/.rhack` as the destination to copy the source code of the
external crates by default. You can change it by setting and exposing the
`$RHACK_DIR` environment variable.

## Installation

For MacOS, Linux, and Windows, prebuilt binaries are available through
[here](https://github.com/simonsan/cargo-rhack/releases).

<!-- ### Debian/Ubuntu

```sh
wget https://github.com/nakabonne/rhack/releases/download/v0.1.0/rhack_linux_amd64.deb
apt install ./rhack_linux_amd64.deb
```

### Arch Linux

Currently there are no
[AUR packages](https://aur.archlinux.org/packages/cargo-rhack/) available. -->

### Cargo

```sh
cargo install cargo-rhack
```
<!-- 
### From source

```sh
git clone https://github.com/simonsan/cargo-rhack.git
cargo build --release
sudo install -m755 target/release/cargo-rhack /usr/local/bin/cargo-rhack
```

If you want to generate the man page, you can install it with `scdoc`.

```sh
sudo mkdir -p /usr/local/share/man/man1
scdoc < rhack.1.scd > rhack.1
sudo install -m644 rhack.1 /usr/local/share/man/man1/rhack.1
``` -->

## Contributing

Found a bug? [Open an issue!](https://github.com/simonsan/cargo-rhack/issues/new/choose)

Got an idea for an improvement? Don't keep it to yourself!

- [Contribute fixes](https://github.com/simonsan/cargo-rhack/contribute) or new features
  via pull requests!

## Code of Conduct

Please review and abide by the general
[Rust Community Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
when contributing to this project. In the future, we might create our own Code
of Conduct and supplement it at this location.

## Minimum Rust version policy

This crate's minimum supported `rustc` version is `1.65.0`.

The current policy is that the minimum Rust version required to use this crate
can be increased in minor version updates. For example, if `crate 1.0` requires
Rust 1.20.0, then `crate 1.0.z` for all values of `z` will also require Rust
1.20.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum
version of Rust.

In general, this crate will be conservative with respect to the minimum
supported version of Rust.

## Acknowledgements

The original author of this tool is [nakabonne](https://github.com/nakabonne),
I revived it and fixed some things. I'm grateful for the original author's
efforts and contributions to the Rust community. I hope this tool will be
useful for many people in the Rust community going forward.

### Original inspiration

This tool is highly inspired by [gohack](https://github.com/rogpeppe/gohack). It
clearly stimulated an incentive to creation. A big "thank you!" goes out to
them.

## License

**BSD-3-Clause license**; see [LICENSE](./LICENSE).

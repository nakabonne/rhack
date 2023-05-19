# cargo-rhack

[![Version info](https://img.shields.io/crates/v/rhack.svg)](https://crates.io/crates/cargo-rhack)

You want to quickly put a sneaky macro kind of like `dbg!` into external crates to find out how some internal data structure works? If so `cargo-rhack` is for you!

`cargo-rhack` makes it easier to edit external crates code that your project depends on.

## Usage

Let's say you want to modify the `reqwest` crate.

```toml
[dependencies]
reqwest = "0.11"
```

Run the following:

```
cargo rhack edit reqwest
```

This will make a copy of the crate into `$HOME/.rhack/reqwest-0.11.1` and add its path to the [[patch] section](<https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html>) in your Cargo.toml whose path is automatically detected:

```toml
[patch.crates-io]
reqwest = { path = "/home/you/.rhack/reqwest-0.11.1" }
```

Now your package uses the locally checked out copy instead of one from crates.io. You can now open the files (typically by leveraging the "Jump to Definition" feature) and then feel free to modify the source code.

### Undoing

Simply run the `undo` command then you can undo all of the changes to your Cargo.toml:

```
cargo rhack undo
```

Keep in mind that this command doesn't remove any copy of crates.

### Settings

It uses `$HOME/.rhack` as the destination to copy the source code of the external crates by default. You can change it by setting and exposing the `$RHACK_DIR` environment variable.

## Installation

For MacOS, Linux, and Windows, prebuilt binaries are available through [here](https://github.com/nakabonne/rhack/releases).

### Debian/Ubuntu

```
wget https://github.com/nakabonne/rhack/releases/download/v0.1.0/rhack_linux_amd64.deb
apt install ./rhack_linux_amd64.deb
```

### Arch Linux

An [AUR package](https://aur.archlinux.org/packages/rhack/) is available.

```
yay rhack
```

### Cargo

```
cargo install rhack
```

### From source

```
git clone https://github.com/nakabonne/rhack.git
cargo build --release
sudo install -m755 target/release/rhack /usr/local/bin/rhack
```

If you want to generate the man page, you can install it with `scdoc`.

```
sudo mkdir -p /usr/local/share/man/man1
scdoc < rhack.1.scd > rhack.1
sudo install -m644 rhack.1 /usr/local/share/man/man1/rhack.1
```

## Acknowledgements

This tool is highly inspired by [gohack](https://github.com/rogpeppe/gohack). It clearly stimulated an incentive to creation. A big "thank you!" goes out to them.

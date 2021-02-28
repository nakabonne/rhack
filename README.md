# rhack
Would you like to put a sneaky `dbg!` macro into external crates to find out how some internal data structure works? If so `rhack` is for you!

## Usage

Let's say you want to modify the `reqwest` crate.

```toml
[dependencies]
reqwest = "0.11"
```

Run the following:

```
$ rhack edit reqwest
```

This will make a copy of the crate into `$HOME/.rhack/reqwest-0.11.1` and add its path to the [[patch] section](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html) in your Cargo.toml:

```toml
[patch.crates-io]
reqwest = { path = "/Users/nakabonne/.rhack/reqwest-0.11.1", version = "0.11" }
```

Now your package uses the locally checked out copy instead of one from crates.io. You can now put the `dbg!` macro or try to fix the bug!

### Undoing

```
$ rhack undo reqwest
```

## Inspired by
- [gohack](https://github.com/rogpeppe/gohack)
# rhack
Are you want to put a sneaky `dbg!` macro into external crates to find out how some internal data structure works? If so `rhack` is for you!

## Usage

Let's say you want to modify the `reqwest` crate.

```toml
[dependencies]
reqwest = "0.11"
```

Run `rhack edit`:

```
$ rhack edit reqwest
```

This will make a copy of the crate into $HOME/.rhack/reqwest-0.11.1 and add its path to your Cargo.toml:

```toml
[dependencies]
reqwest = { path = "/Users/nakabonne/.rhack/reqwest-0.11.1", version = "0.11" }
```

### Undoing

```
$ rhack undo reqwest
```
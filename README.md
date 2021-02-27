# rhack

## Usage

Let's say you want to modify reqwest crate.

```toml
[dependencies]
reqwest = "0.11"
```

Run `rhack get`:

```
$ rhack get reqwest
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
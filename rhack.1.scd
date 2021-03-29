rhack(1)

# NAME

rhack - easily edit external crates that your Rust project depends on.

# SYNOPSIS

*rhack* <package name>

# EDITING

To check out a local copy of your dependency run:

	*rhack* <package name>

This will make a copy of the crate in your _RHACK_DIR_, and patch your
*Cargo.toml*.

For example running *rhack reqwest* might result in a local copy
at _~/.rhack/reqwest-0.11.1_, and an amendment to your *Cargo.toml* like this:

	\[patch.crates-io\]++
reqwest = { path = "/home/you/.rhack/reqwest-0.11.1" }

# UNDO

To remove changes made by *rhack* to *Cargo.toml*, run:

	*rhack undo*

This will _not_ remove any copies of crates in your _RHACK_DIR_.

# CONFIGURATION

rhack stores copies of crates in the directory specified by the _RHACK_DIR_
environment variable.

Where this is unset, the default is _$HOME/.rhack_

# AUTHORS

Ryo Nakao <ryo@nakao.dev>

# PROJECT HOMEPAGE

Sources can be found, and bugs or patches submitted at https://github.com/nakabonne/rhack

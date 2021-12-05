mod cmd;
mod error;

use crate::cmd::{App, Cmd};
use crate::error::SilentExit;

use clap::Parser;

use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(e) = App::parse().run() {
        match e.downcast::<SilentExit>() {
            Ok(SilentExit { code }) => process::exit(code),
            Err(e) => {
                let _ = writeln!(io::stderr(), "rhack: {:?}", e);
                process::exit(1);
            }
        }
    }
}

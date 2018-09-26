#![deny(unused)]

extern crate clap;
#[macro_use]
extern crate failure;
extern crate reqwest;

use clap::{App, Arg};
use failure::Error;

fn get_sha(path: &str) -> Result<String, Error> {
    let mut resp = reqwest::get(&format!("{}.sha256", path))?;
    assert!(resp.status().is_success());

    resp.text()?
        .split_whitespace()
        .next()
        .map(|s| s.into())
        .ok_or_else(|| format_err!("Failed to get sha"))
}

fn main() -> Result<(), Error> {
    let matches = App::new("Update flatpak json")
        .version("1.0")
        .author("Daniel García <danigm@wadobo.com>")
        .about("Prints the sha of Rustc release tarballs")
        .arg(
            Arg::with_name("VERSION")
                .help("the rustc version")
                .required(true)
                .index(1),
        ).get_matches();

    let version = matches.value_of("VERSION").unwrap();
    let sources = vec![
        (
            "source tarball",
            format!(
                "https://static.rust-lang.org/dist/rustc-{}-src.tar.gz",
                version
            ),
        ),
        (
            "x86_64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-x86_64-unknown-linux-gnu.tar.gz",
                version
            ),
        ),
        (
            "i686",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-i686-unknown-linux-gnu.tar.gz",
                version
            ),
        ),
        (
            "armv7hf",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-aarch64-unknown-linux-gnu.tar.gz",
                version
            ),
        ),
        (
            "aarch64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-armv7-unknown-linux-gnueabihf.tar.gz",
                version
            ),
        ),
    ];

    for (arch, url) in sources {
        let sha = get_sha(&url)?;
        println!("{}: {}", arch, sha);
    }

    Ok(())
}

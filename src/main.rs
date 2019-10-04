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

#[test]
fn test_get_sha() -> Result<(), Error> {
    let source = "4d0d64c60561ea8acc01b866f48e42c906b60215c43ff7d7b088ee21bdcd813e";
    let x86_64 = "6e38e51cc5cc2a6e66401ed88571fb044792e0694b2af8edd9b12fbe87c4e1f9";
    let i686 = "16107f767fe874fc870e46ffc74cfaeae670d5ff4b9b620f9867c87c26fb58f2";
    let aarch64 = "bb8664f0aa5a5c310f0ec51bd6520539ede94a39aebdb13fd81f8658c18eec2c";
    let armv7hf = "2cfab068b16c0e4e9450abb1826d82c55c92c0438e3c82596458884863bf2857";

    let s = "https://static.rust-lang.org/dist/rustc-1.29.1-src.tar.xz";
    let x = "https://static.rust-lang.org/dist/rust-1.29.1-x86_64-unknown-linux-gnu.tar.xz";
    let i = "https://static.rust-lang.org/dist/rust-1.29.1-i686-unknown-linux-gnu.tar.xz";
    let aa = "https://static.rust-lang.org/dist/rust-1.29.1-aarch64-unknown-linux-gnu.tar.xz";
    let arm = "https://static.rust-lang.org/dist/rust-1.29.1-armv7-unknown-linux-gnueabihf.tar.xz";

    assert_eq!(get_sha(s)?, source);
    assert_eq!(get_sha(x)?, x86_64);
    assert_eq!(get_sha(i)?, i686);
    assert_eq!(get_sha(arm)?, armv7hf);
    assert_eq!(get_sha(aa)?, aarch64);
    Ok(())
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
                "https://static.rust-lang.org/dist/rustc-{}-src.tar.xz",
                version
            ),
        ),
        (
            "x86_64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-x86_64-unknown-linux-gnu.tar.xz",
                version
            ),
        ),
        (
            "i686",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-i686-unknown-linux-gnu.tar.xz",
                version
            ),
        ),
        (
            "armv7hf",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-armv7-unknown-linux-gnueabihf.tar.xz",
                version
            ),
        ),
        (
            "aarch64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-aarch64-unknown-linux-gnu.tar.xz",
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

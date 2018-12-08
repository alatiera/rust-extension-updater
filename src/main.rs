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
    let source = "f1b0728b66ce6bce6d72bbe5ea9e3a24ea22a045665da2ed8fcdfad14f61a349";
    let x86_64 = "b36998aea6d58525f25d89f1813b6bfd4cad6ff467e27bd11e761a20dde43745";
    let i686 = "05e2880beca45e7319074d2268fd79a70c7aade2fb14dbcbf39585b5560f2048";
    let aarch64 = "2685224f67b2ef951e0e8b48829f786cbfed95e19448ba292ac33af719843dbe";
    let armv7hf = "2cae2ecc366914707d6b753a96505c727df69df8bcbc1f8d14fbd66fca005239";

    let s = "https://static.rust-lang.org/dist/rustc-1.29.1-src.tar.gz";
    let x = "https://static.rust-lang.org/dist/rust-1.29.1-x86_64-unknown-linux-gnu.tar.gz";
    let i = "https://static.rust-lang.org/dist/rust-1.29.1-i686-unknown-linux-gnu.tar.gz";
    let aa = "https://static.rust-lang.org/dist/rust-1.29.1-aarch64-unknown-linux-gnu.tar.gz";
    let arm = "https://static.rust-lang.org/dist/rust-1.29.1-armv7-unknown-linux-gnueabihf.tar.gz";

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
                "https://static.rust-lang.org/dist/rust-{}-armv7-unknown-linux-gnueabihf.tar.gz",
                version
            ),
        ),
        (
            "aarch64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-aarch64-unknown-linux-gnu.tar.gz",
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

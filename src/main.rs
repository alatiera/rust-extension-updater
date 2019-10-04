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
    let source = "10abffac50a729cf74cef6dd03193a2f4647541bd19ee9281be9e5b12ca8cdfd";
    let x86_64 = "b498a84947012064363607c29cdd2ba6fab9a5260212ec4a7151dafd0d079081";
    let i686 = "f99daecd13c716b0c285b1c26225e71df2c7ca520a00285ba4c26ff767680ecb";
    let aarch64 = "aed553dbf457d8239eb3b01d4a2f422672772a0114aa43647294eca1b358d219";
    let armv7hf = "d4ed33497b3a3b89f5910fa140bbc9f4865ebf1735898bbe06266ef5944f6fca";
    let powerpc64le = "eecebeaa2100950196e604d5680f9a9e7b6e97c535c246011a2247fe6ce629fc";

    let s = "https://static.rust-lang.org/dist/rustc-1.37.0-src.tar.xz";
    let x = "https://static.rust-lang.org/dist/rust-1.37.0-x86_64-unknown-linux-gnu.tar.xz";
    let i = "https://static.rust-lang.org/dist/rust-1.37.0-i686-unknown-linux-gnu.tar.xz";
    let aa = "https://static.rust-lang.org/dist/rust-1.37.0-aarch64-unknown-linux-gnu.tar.xz";
    let arm = "https://static.rust-lang.org/dist/rust-1.37.0-armv7-unknown-linux-gnueabihf.tar.xz";
    let p = "https://static.rust-lang.org/dist/rust-1.37.0-powerpc64le-unknown-linux-gnu.tar.xz";

    assert_eq!(get_sha(s)?, source);
    assert_eq!(get_sha(x)?, x86_64);
    assert_eq!(get_sha(i)?, i686);
    assert_eq!(get_sha(aa)?, aarch64);
    assert_eq!(get_sha(arm)?, armv7hf);
    assert_eq!(get_sha(p)?, powerpc64le);
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
            "aarch64",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-aarch64-unknown-linux-gnu.tar.xz",
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
            "powerpc64le",
            format!(
                "https://static.rust-lang.org/dist/rust-{}-powerpc64le-unknown-linux-gnu.tar.xz",
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

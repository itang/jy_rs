use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;
use structopt::StructOpt;
use toml;
use toml::Value;

#[derive(StructOpt, Debug)]
#[structopt(name = "jy_rs")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,
    // The number of occurrences of the `v/verbose` flag
    //// Verbose mode (-v, -vv, -vvv, etc.)
    //#[structopt(short, long, parse(from_occurrences))]
    //verbose: u8,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let path = match opt.config {
        Some(p) => p,
        None => Path::new(&env::var("HOME")?)
            .join("bin")
            .join("jiayou.toml"),
    };

    println!("Read config from {:?}", path);

    let content = fs::read_to_string(path)?;
    let r = content.parse::<Value>()?;

    let mut index = 0;
    for urls in r["urls"].as_array() {
        for url_value in urls {
            for url in url_value.as_str() {
                println!("{:4}: open {}", index, url);
                browser(url)?;
                index += 1;
            }
        }
    }

    Ok(())
}

fn browser(url: &str) -> Result<()> {
    Command::new("x-www-browser")
        .arg(url)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}

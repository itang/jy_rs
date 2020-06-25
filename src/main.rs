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

    let config_path = get_config_path(&opt)?;
    println!("Read config from {:?}", config_path);

    let content = fs::read_to_string(config_path)?;
    let config = content.parse::<Value>()?;

    browser_batch(config)
}

fn get_config_path(opt: &Opt) -> Result<PathBuf> {
    Ok(match &opt.config {
        Some(p) => p.clone(),
        None => Path::new(&env::var("HOME")?)
            .join("bin")
            .join("jiayou.toml"),
    })
}

fn browser_batch(config: Value) -> Result<()> {
    let urls = config["urls"]
        .as_array()
        .into_iter()
        .flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()));
    for (index, url) in urls.enumerate() {
        println!("{:4}: open {}", index, url);
        browser_single_url(url)?;
    }

    /*
    let mut index = 0;
    for urls in r["urls"].as_array() {
        for url_value in urls {
            for url in url_value.as_str() {
                println!("{:4}: open {}", index, url);
                browser(url)?;
                index += 1;
            }
        }
    }*/

    Ok(())
}

fn browser_single_url(url: &str) -> Result<()> {
    Command::new("x-www-browser")
        .arg(url)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}

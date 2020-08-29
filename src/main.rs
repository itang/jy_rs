use std::fs;
use std::process::{Command, Stdio};

use anyhow::Result;
use structopt::StructOpt;
use toml::{self, Value};

use opt::{IConfigPath, Opt};

mod opt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if opt.dry_run {
        println!("dry run. exit!");
        Ok(())
    } else {
        let config_path = opt.get_config_path()?;
        println!("Read config from {:?}", config_path);

        let content = fs::read_to_string(config_path)?;
        let config = content.parse::<Value>()?;

        browser_batch(config)
    }
}

fn browser_batch(config: Value) -> Result<()> {
    let urls = config["urls"]
        .as_array()
        .into_iter()
        .flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()));
    for (index, url) in urls.enumerate() {
        println!("{:4}: open {}", index + 1, url);
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
    //TODO: Cross-system support
    Command::new("x-www-browser")
        .arg(url)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}

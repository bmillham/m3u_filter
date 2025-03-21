use m3u_parser::M3uParser;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{fs::File, io::Write};
use clap::Parser;

#[derive(Deserialize, Debug)]
struct Config {
    urls: Vec<String>,
    template: Option<String>,
    channels: Vec<String>,
    all_channels: Option<String>,
    new_channels: Option<String>,
    ignore_url: Vec<String>,
    ignore_title: Vec<String>,
    countries: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
	#[arg(short, long, default_value="m3u_filter_config.toml")]
	config_file: String,
	#[arg(short, long, default_value=".")]
	output_dir: String
}

#[tokio::main]
async fn main() {
	let args = Args::parse();
	
    let config_contents = match std::fs::read_to_string(&args.config_file) {
        Ok(c) => c,
        Err(e) => panic!("Error {e:?} reading {}", args.config_file),
    };

    let config: Config = match toml::from_str(&config_contents) {
        Ok(c) => c,
        Err(e) => panic!("Error {e:?} parsing {}", args.config_file),
    };

    let channels: Vec<&str> = config.channels.iter().map(|s| &**s).collect();
    let ignore_url: Vec<&str> = config.ignore_url.iter().map(|s| &**s).collect();
    let ignore_title: Vec<&str> = config.ignore_title.iter().map(|s| &**s).collect();
    let countries: Vec<&str> = config.countries.iter().map(|s| &**s).collect();
    let template = match config.template {
        Some(s) => s,
        _ => String::from("default"),
    };

    let mut count = 1;
    for url in config.urls {
        println!("Downloading/parsing {url}");
        let mut parser = M3uParser::new(None);
        parser.parse_m3u(&url, false, true).await;
        println!("Number of streams found: {}", parser.streams_info.len());
        parser.sort_by("title", "", true, false);
        parser.filter_by("url", ignore_url.clone(), "", false, false);
        println!(
            "Number of streams after filtering {} urls: {}",
            ignore_url.len(),
            parser.streams_info.len()
        );
        parser.filter_by("title", ignore_title.clone(), "", false, false);
        println!(
            "Number of streams after filtering {} titles: {}",
            ignore_title.len(),
            parser.streams_info.len()
        );
        if !countries.is_empty() {
            parser.filter_by("title", countries.clone(), "", true, false);
            println!(
                "Number of streams in {} countries: {}",
                countries.len(),
                parser.streams_info.len()
            );
        }

        if count == 1 {
            // Save all streams to a file
            match config.all_channels {
                Some(ref s) => {
                    let mut output = match File::create(args.output_dir.to_owned() + "/" + s) {
                        Ok(f) => f,
                        Err(e) => {
                            panic!("Error creating {s}: {e:?}");
                        }
                    };
                    let js = match parser.get_json(false) {
                        Ok(j) => j,
                        Err(e) => {
                            panic!("Unable to get json: {e:?}");
                        }
                    };
                    let v: Vec<Value> = serde_json::from_str(&js).expect("None");
                    for j in v {
                        writeln!(output, "{}", j["title"]).expect("Error");
                    }
                    println!("Saved full filtered channel list to {s}");
                }
                _ => println!("Not saving full list"),
            }
        }

        parser.filter_by("title", channels.clone(), "", true, false);
        println!(
            "Number of streams after selecting {} channels: {}",
            channels.len(),
            parser.streams_info.len()
        );
        parser.to_file(&format!("{}/{template}-{count}", args.output_dir), "m3u");

        count += 1;
    }
}

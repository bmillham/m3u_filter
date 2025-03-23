use m3u_parser::M3uParser;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{fs::{File, read_to_string}, io::Write, path::Path};
use clap::Parser;
use similar::{ChangeTag, TextDiff};
//#![feature(os_str_display)]
//use std::ffi::OsStr;


#[derive(Deserialize, Debug)]
struct Config {
    urls: Vec<String>,
    template: Option<String>,
    channels: Vec<String>,
    all_channels: Option<String>,
    ignore_url: Vec<String>,
    ignore_title: Vec<String>,
    countries: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
	#[arg(short, long, default_value="m3u_filter_config.toml")]
	config_file: String,
	#[arg(short, long)]
	output_dir: Option<String>,
	#[arg(short, long)]
	input_file: Option<String>,
	#[arg(short, long)]
	template: Option<String>,
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

	let urls = match args.input_file {
		Some(s) => vec![s],
		_ => config.urls.iter().map(|s| s.to_string()).collect(),
	};
    let channels: Vec<&str> = config.channels.iter().map(|s| &**s).collect();
    let ignore_url: Vec<&str> = config.ignore_url.iter().map(|s| &**s).collect();
    let ignore_title: Vec<&str> = config.ignore_title.iter().map(|s| &**s).collect();
    let countries: Vec<&str> = config.countries.iter().map(|s| &**s).collect();
	let template = match args.template {
		Some(s) => s,
		_ => match config.template {
			Some(s) => s,
			_ => String::from("default"),
		},
	};
	let output_dir = match args.output_dir {
		Some(ref s) => {
			if s == "." {
				Path::new("")
			} else {
				Path::new(s)
			}
		},
		_ => Path::new(""),
	};

    let mut count = 1;

	for url in urls {
        println!("Downloading/parsing {url}");
        let mut parser = M3uParser::new(None);
        parser.parse_m3u(&url, false, true).await;
		if parser.streams_info.len() == 0 {
			continue;
		}
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
					let of = output_dir.join(s);
					let fs = of.file_stem().unwrap();
					let ext = match of.extension() {
						Some(s) => {
							String::from(format!(".{}", s.to_string_lossy()))
						},
						_ => String::new(),
					};
					let diff_file = output_dir.join(format!("{}_diff{}", fs.to_str().unwrap(), ext));
					let original_contents = match read_to_string(&of) {
						Ok(s) => s,
						Err(_) => String::new(),
					};
					
                    let mut output = match File::create(&of) {
                        Ok(f) => f,
                        Err(e) => {
                            panic!("Error creating {of:?}: {e:?}");
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
                    println!("Saved full filtered channel list to {of:?}");
					let new_contents = read_to_string(&of).unwrap();
					let cdiff = TextDiff::from_lines(&original_contents, &new_contents);
					let mut diff_output = match File::create(&diff_file) {
                        Ok(f) => f,
                        Err(e) => {
                            panic!("Error creating {diff_file:?}: {e:?}");
                        }
                    };
					let mut changes = 0;
					let mut inserted = 0;
					let mut deleted = 0;
					for change in cdiff.iter_all_changes() {
						let sign = match change.tag() {
							ChangeTag::Delete => {
								deleted += 1;
								"-"
							},
							ChangeTag::Insert => {
								inserted += 1;
								"+"
							},
							ChangeTag::Equal => " ",
						};
						if sign != " " {
							write!(diff_output, "{sign} {change}").expect("Failed to write diff info");
							changes += 1;
						}
					}
					if changes > 0 {
						println!("Found {inserted} new streams.");
						println!("{deleted} streams are no longer available.");
						println!("Saved {changes} changes to {diff_file:?}");
					} else {
						println!("No streams were added or deleted");
					}
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

		parser.to_file(&output_dir.join(format!("{template}-{count}")).display().to_string(), "m3u");

        count += 1;
    }
}

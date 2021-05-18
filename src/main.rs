mod templates;
use std::{
    fs::{self, create_dir_all, remove_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
extern crate yaml_rust;
use clap::{App, Arg};
use yaml_rust::{Yaml, YamlLoader};

fn main() -> std::io::Result<()> {
    let matches = App::new("Theme file gen")
        .version("0.1.0")
        .author("Snazzie")
        .about("Generates style files from css")
        .arg(
            Arg::with_name("Input")
                .short("i")
                .long("input")
                .index(1)
                .required(true)
                .value_name("FILE")
                .takes_value(true)
                .help("Input css path"),
        )
        .arg(
            Arg::with_name("Output")
                .short("o")
                .required(false)
                .help("Output path"),
        )
        .arg(
            Arg::with_name("Config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .index(2)
                .required(true)
                .help("Config file"),
        )
        .get_matches();

    let input_css: String;
    match matches.value_of("Input") {
        Some(i) => {
            let mut file = File::open(i)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            input_css = contents;
        }
        None => {
            panic!();
        }
    }

    let config_content: Vec<Yaml>;

    match matches.value_of("Config") {
        Some(i) => {
            let contents = fs::read_to_string(i).ok().unwrap();

            config_content = YamlLoader::load_from_str(&contents).unwrap();
        }
        None => {
            panic!();
        }
    }
    let theme_name: String =
        String::from(config_content[0]["themeName"].as_str().unwrap().to_string());
    let namespace: String =
        String::from(config_content[0]["namespace"].as_str().unwrap().to_string());
    let url_regex: String = String::from(config_content[0]["regex"].as_str().unwrap().to_string());
    let description: String = String::from(
        config_content[0]["description"]
            .as_str()
            .unwrap()
            .to_string(),
    );
    let author: String = String::from(config_content[0]["author"].as_str().unwrap().to_string());
    let homepage_url: String = String::from(
        config_content[0]["homepageUrl"]
            .as_str()
            .unwrap()
            .to_string(),
    );
    let support_url: String = String::from(
        config_content[0]["supportUrl"]
            .as_str()
            .unwrap()
            .to_string(),
    );
    let update_url: String =
        String::from(config_content[0]["updateUrl"].as_str().unwrap().to_string());
    let theme_css: String = input_css;
    let version: String;

    match matches.value_of("version") {
        Some(version_arg) => {
            version = version_arg.to_string();
        }
        None => {
            println!("Version not provided as an argument, will use version from file");
            version = String::from(config_content[0]["version"].as_str().unwrap().to_string());
        }
    }

    let output_path: PathBuf =
        Path::new(&matches.value_of("Output").unwrap_or("./Output")).to_owned();
    println!(
        "{}",
        &output_path
            .to_owned()
            .into_os_string()
            .into_string()
            .unwrap()
    );

    create_dir_all(&output_path)?;

    File::create(output_path.join("github.user.styl"))?.write_all(
        templates::new::stylus(
            theme_css.to_owned(),
            theme_name.to_owned(),
            namespace.to_owned(),
            version.to_owned(),
            description.to_owned(),
            author.to_owned(),
            url_regex.to_owned(),
            support_url.to_owned(),
            update_url.to_owned(),
            url_regex.to_owned(),
        )
        .as_bytes(),
    )?;

    File::create(output_path.join("github.user.js"))?.write_all(
        templates::new::user_script(
            theme_css.to_owned(),
            theme_name.to_owned(),
            namespace.to_owned(),
            version.to_owned(),
            description.to_owned(),
            author.to_owned(),
            url_regex.to_owned(),
            support_url.to_owned(),
            update_url.to_owned(),
            url_regex.to_owned(),
        )
        .as_bytes(),
    )?;

    Ok(())
}

use ansi_term::Colour;
use clap::{ArgEnum, Parser, Subcommand};
use regex::Regex;
use reqwest;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use tokio;

#[derive(Deserialize)]
struct Package {
    #[serde(deserialize_with = "parse_null_strings")]
    Description: String,
    FirstSubmitted: i32,
    ID: i32,
    #[serde(deserialize_with = "parse_null_integers")]
    LastModified: i32,
    #[serde(deserialize_with = "parse_null_strings")]
    Maintainer: String,
    Name: String,
    NumVotes: i8,
    #[serde(deserialize_with = "parse_null_strings")]
    PackageBase: String,
    PackageBaseID: i32,
    URL: String,
    URLPath: String,
    Version: String,
    #[serde(deserialize_with = "parse_null_integers")]
    OutOfDate: i32,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Install { name: String },
    Search { name: String },
}

fn parse_null_integers<'de, D>(d: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(0))
}

fn parse_null_strings<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("N/A".to_string()))
}

#[derive(Deserialize)]
struct AurResponse {
    resultcount: i32,
    results: Vec<Package>,
}

async fn query_aur(keyword: String) {
    let url = "https://aur.archlinux.org/rpc/?v=5&type=search&by=name&arg=".to_owned()
        + &keyword.to_string();
    println!("Fetching from: {}", url);
    let response: AurResponse = reqwest::get(url).await.unwrap().json().await.unwrap();
    println!(
        "Found {count} results",
        count = Colour::Green.paint(response.resultcount.to_string())
    );
    for package in response.results {
        println!(
            "
            {name}
            Description: {description}
            Maintainer: {maintainer}
            Votes: {votes}
            URL: {url}
            Download URL: https://aur.archlinux.org/{name}.git
            ",
            name = Colour::Red.paint(package.Name),
            description = package.Description,
            maintainer = package.Maintainer,
            votes = package.NumVotes,
            url = package.URL,
        );
    }
}

async fn install_package(package_name: String) {
    if cfg!(target_os = "windows") {
        panic!("Windows is not supported!");
    } else {
        let folder_name = format!("{name}", name = package_name);
        let mut git_process_child = Command::new("git")
            .arg("clone")
            .arg(format!(
                "https://aur.archlinux.org/{name}.git",
                name = package_name
            ))
            .spawn()
            .unwrap();

        let result = git_process_child.wait().unwrap();

        println!("{}", result);
        let mut make_install_package_child = Command::new("makepkg")
            .current_dir(&folder_name)
            .arg("-si")
            .spawn()
            .unwrap();

        let make_install_result = make_install_package_child.wait().unwrap();

        println!("{}", make_install_result);

        let mut remove_tmp_child = Command::new("rm")
            .arg("-rf")
            .arg(&folder_name)
            .spawn()
            .unwrap();

        let remove_tmp_result = remove_tmp_child.wait().unwrap();

        println!("{}", remove_tmp_result);
    };
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        SubCommands::Install { name } => {
            println!("Installing {:?}", name);
            let package_name = format!("{:?}", name);
            install_package(name.to_owned()).await;
        }
        SubCommands::Search { name } => {
            println!("Searching for {:?}", name);
            let package_name = format!("{:?}", name);
            query_aur(name.to_owned()).await;
        }
    }
}

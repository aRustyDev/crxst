// use std::process::Command;
// use error_chain::error_chain;
use std::io::{copy, Read, Write};
use std::fs::File;
// use tempfile::Builder;
// use serde::Deserialize;
// use clap::{Parser, Command, Subcommand, Arg};
use clap::{Command, Arg};
use clap;
use serde_yaml;
use serde_yaml::Value;
use libc::pid_t;
// use reqwest;
use std::any::type_name;
// error_chain! {
//      foreign_links {
//          Io(std::io::Error);
//          HttpRequest(reqwest::Error);
//      }
// }

const DEFAULT_CRX_URL: &str     = "https://clients2.google.com/service/update2/crx";
const DEFAULT_CRX_CONFIG: &str  = "./crx.yaml";

struct Chrome<'a> {
    pid: pid_t,
    crx: Vec<String>,
    url: &'a str,
}

impl Chrome<'_> {
    fn new() -> Chrome<'static> { // TODO: plumb URL option & crx read from yaml
        Chrome {
            pid: 0,
            crx: Vec::new(),
            url: "https://clients2.google.com/service/update2/crx",
        }
    }

    // fn download(&self) -> Result<(), E> {
    //     let tmp_dir = Builder::new().prefix("example").tempdir()?;
    //     let response = reqwest::get(self.url).await?;
    
    //     let mut dest = {
    //         let fname = response
    //             .url()
    //             .path_segments()
    //             .and_then(|segments| segments.last())
    //             .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //             .unwrap_or("tmp.bin");
    
    //         println!("file to download: '{}'", fname);
    //         let fname = tmp_dir.path().join(fname);
    //         println!("will be located under: '{:?}'", fname);
    //         File::create(fname)?
    //     };
    //     let content =  response.text().await?;
    //     copy(&mut content.as_bytes(), &mut dest)?;
    //     Ok(())
    
    //     let crx = Command::new("wget")
    //                       .arg(self.url)
    //                       .output().unwrap();
    //     println!("CRX: {}", String::from_utf8_lossy(&crx.stdout));
    // }

    // fn start(&self) -> Result<(), E> { // TODO return chrome process
    
    //     // Windows: $chromePath = "C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" # Path to Chrome executable
    //     // Linux: $chromePath = "/usr/bin/google-chrome" # Path to Chrome executable
    //     // Mac: $chromePath = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" # Path to Chrome executable
    
    //     // Path to Chrome application (adjust if necessary)
    //     let chrome_path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
    
    //     // Command to launch Chrome with remote debugging
    //     let mut command = Command::new(chrome_path);
    //     command.arg("--remote-debugging-port=9222");
    
    //     // Attempt to spawn the process
    //     let process = match command.spawn() {
    //         Ok(child) => child,
    //         Err(err) => {
    //             eprintln!("Error spawning Chrome process: {}", err);
    //             return;
    //         }
    //     };
    
    //     // Get the process ID
    //     let self.pid = unsafe { process.id() } as pid_t;
    
    //     // Print success message and PID
    //     println!("Chrome launched in remote debugging mode with PID: {}", self.pid);
    //     Ok(())
    // }
    
    // fn kill(&self) -> Result<(), E> {
    //     unsafe {
    //         libc::kill(self.pid, libc::SIGTERM);
    //     }
    //     println!("Chrome process terminated (PID: {})", self.pid);
    //     // Safety note: This function is not implemented as directly killing a process
    //     // can be dangerous. You should implement proper logic with error handling
    //     // based on your specific needs. 
    // }

}

// fn unzip_crx(crx: &str) {
//     let unzip = Command::new("unzip")
//                         .arg(crx)
//                         .output().unwrap();
//     println!("UNZIP: {}", String::from_utf8_lossy(&unzip.stdout));
// }

// fn connect_to_chrome() {
//     let connect = Command::new("nc")
//                           .arg("localhost")
//                           .arg("9222")
//                           .output().unwrap();
//     println!("CONNECT: {}", String::from_utf8_lossy(&connect.stdout));
// }

// fn install_extension() {
//     let install = Command::new("curl")
//                           .arg("http://localhost:9222/json")
//                           .output().unwrap();
//     println!("INSTALL: {}", String::from_utf8_lossy(&install.stdout));
// }

// fn read_extensions() {
//     let extensions = Command::new("cat")
//                              .arg("extensions.json")
//                              .output().unwrap();
//     println!("EXTENSIONS: {}", String::from_utf8_lossy(&extensions.stdout));
// }

// fn pack_extension() {
//     let pack = Command::new("zip")
//                        .arg("-r")
//                        .arg("extension-id.zip")
//                        .arg("extension-id")
//                        .output().unwrap();
//     println!("PACK: {}", String::from_utf8_lossy(&pack.stdout));
// }

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_config(path: &str) -> Vec<String> {
    // Open the file in read-only mode with buffer.
    // let f = std::fs::File::open(path).unwrap();
    let f = std::fs::read_to_string(path).unwrap();
    // Deserialize the YAML file into a String
    let d: Value = serde_yaml::from_str::<Value>(&f).unwrap();
    let mut ids: Vec<String> = Vec::<String>::new();

    for (key, value) in d.get("crx").unwrap().as_mapping().unwrap() {
        println!("{} / {}", key, value);
        ids.push(value);
    }
    
    // println!("Type: {:?}", type_of());
    // println!("Deserialized: {:?}", ids);

    // // Path to your YAML file
    // let file_path = "config.yaml";

    // // Open the file
    // let mut file = File::open(file_path);
    // // Read the contents of the file
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // Deserialize the YAML content into a Vec<String>
    let data_array: Vec<String> = Vec::<String>::new();
    return data_array;
}

fn cmd_install(cmd: &clap::ArgMatches) {
    println!("You ran '{:#?}'!", "install");
    let config = cmd.get_occurrences::<String>("config").unwrap().map(Iterator::collect::<Vec<_>>).collect::<Vec<_>>()[0][0];
    println!("Config File : {:#?}", read_config(&config));
    // 1. Read Config CRX.yaml
}

fn cmd_export(cmd: &clap::ArgMatches){
    // println!("You ran '{:#?}'!", cmd)
    println!("You ran '{:#?}'!", "export")
    // let export = Command::new("crx")
    //                      .arg("export")
    //                      .output().unwrap();
    // println!("EXPORT: {}", String::from_utf8_lossy(&export.stdout));
}

fn cmd_update(cmd: &clap::ArgMatches){
    // println!("You ran '{:#?}'!", cmd)
    println!("You ran '{:#?}'!", "update")
    // let export = Command::new("crx")
    //                      .arg("export")
    //                      .output().unwrap();
    // println!("EXPORT: {}", String::from_utf8_lossy(&export.stdout));
}

fn cmd_cleanup(cmd: &clap::ArgMatches){
    // println!("You ran '{:#?}'!", cmd)
    println!("You ran '{:#?}'!", "cleanup")
    // let export = Command::new("crx")
    //                      .arg("export")
    //                      .output().unwrap();
    // println!("EXPORT: {}", String::from_utf8_lossy(&export.stdout));
}

fn main() -> Result<(), String> {
    // let styles = styling::Styles::styled()
    //     .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    //     .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    //     .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
    //     .placeholder(styling::AnsiColor::Cyan.on_default());
    let cmd = clap::Command::new("crx")
        // .bin_name("cargo")
        .author("Adam Smith, adam.smith@associates.gwe.cisa.dhs.gov")
        .version("0.1.0")
        .about("Provides a CLI method to download Chome Extensions and install them in a headless Chrome instance")
        .after_help("CRX is designed to deploy a Chrome browser instance in remote debugging mode \
                    a side effect of this is that the current browser will need to be shutdown/killed first. \
                    This is expected behavior.")
        .subcommand_required(true)
        .subcommands([
            Command::new("install")
            // .styles(styles)
            .args([
                Arg::new("debug")
                    .short('d')
                    .help("turns on debugging mode"),
                Arg::new("packed")
                    .short('p')
                    .long("packed")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PACKING")
                    .default_value("true")
                    .help("Enable packing of extensions during export process"),
                Arg::new("path")
                    .short('o')
                    .long("output-path")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PATH")
                    .default_value("./")
                    .help("Output path for exported extensions"),
                Arg::new("config")
                    .short('c')
                    .long("config")
                    // .env("CRX_CONFIG")
                    .default_value(DEFAULT_CRX_CONFIG)
                    .help("Give path to alternate config file"),
            ]),
            Command::new("export")
            // .styles(styles)
            .args([
                Arg::new("debug")
                    .short('d')
                    .help("turns on debugging mode"),
                Arg::new("packed")
                    .short('p')
                    .long("packed")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PACKING")
                    .default_value("true")
                    .help("Enable packing of extensions during export process"),
                Arg::new("path")
                    .short('o')
                    .long("output-path")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PATH")
                    .default_value("./")
                    .help("Output path for exported extensions"),
                Arg::new("config")
                    .short('c')
                    .long("config")
                    // .env("CRX_CONFIG")
                    .default_value(DEFAULT_CRX_CONFIG)
                    .help("Give path to alternate config file"),
            ]),
            Command::new("update")
            // .styles(styles)
            .args([
                Arg::new("debug")
                    .short('d')
                    .help("turns on debugging mode"),
                Arg::new("packed")
                    .short('p')
                    .long("packed")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PACKING")
                    .default_value("true")
                    .help("Enable packing of extensions during export process"),
                Arg::new("path")
                    .short('o')
                    .long("output-path")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PATH")
                    .default_value("./")
                    .help("Output path for exported extensions"),
                Arg::new("config")
                    .short('c')
                    .long("config")
                    // .env("CRX_CONFIG")
                    .default_value(DEFAULT_CRX_CONFIG)
                    .help("Give path to alternate config file"),
            ]),
            Command::new("cleanup") // TODO: implement; remove all extensions & artifacts
            // .styles(styles)
            .args([
                Arg::new("debug")
                    .short('d')
                    .help("turns on debugging mode"),
                Arg::new("packed")
                    .short('p')
                    .long("packed")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PACKING")
                    .default_value("true")
                    .help("Enable packing of extensions during export process"),
                Arg::new("path")
                    .short('o')
                    .long("output-path")
                    .ignore_case(true)
                    // .env("CRX_EXPORT_PATH")
                    .default_value("./")
                    .help("Output path for exported extensions"),
                Arg::new("config")
                    .short('c')
                    .long("config")
                    // .env("CRX_CONFIG")
                    .default_value(DEFAULT_CRX_CONFIG)
                    .help("Give path to alternate config file"),
            ]),
        ]);
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() { // TODO: plumb matches to functions
        Some(("install", matches))  => cmd_install(matches),
        Some(("export", matches))   => cmd_export(matches),
        Some(("update", matches))   => cmd_update(matches),
        Some(("cleanup", matches))  => cmd_cleanup(matches),
        _ => unreachable!("clap should ensure we don't get here"),
    };
    // let url = "https://chrome.google.com/webstore/detail/extension-id";

    // wget_crx(url);
    
    // unzip_crx(crx);
    
    let chrome = Chrome::new();
    // chrome.start();
    // connect_to_chrome();
    // install_extension();
    // chrome.kill();

    Ok(())
}
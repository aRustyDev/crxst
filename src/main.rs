// use std::process::Command;
use std::io::copy;
use std::fs::File;
use std::fs;
use std::collections::HashMap;
use std::io::Error;
use serde::{Serialize, Deserialize};
use clap::Parser;
use serde_yaml;
use libc::pid_t;
// use reqwest;
use std::any::type_name;
use tempfile::Builder;

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

#[derive(Debug, Serialize, Deserialize)]
struct CrxYml {
    uri: String,
    // path: String,
    crx:  HashMap<String, String>,
}

impl<'a> CrxYml {
    fn new() -> CrxYml {
        CrxYml {
            uri: String::new(),
            crx: HashMap::new()
        }
    }

    fn init(&self, path: &str) {
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");
        self = &serde_yaml::from_str::<CrxYml>(&contents).unwrap();
    }

    async fn download_crx(&self, crx_id: &str, tmp: &str) -> Result<(), Box<dyn std::error::Error>>{

        // Create a file to write the CRX to
        let mut fname = File::create(format!("{}{}", tmp, crx_id))?;

        // Download the file
        let target = format!("{}{}", self.uri, crx_id);
        let response = reqwest::get(target).await?;
        let content =  response.text().await?;

        // Write the file to disk
        copy(&mut content.as_bytes(), &mut fname)?;

        Ok(())
    }

    // Assumes CrxYml is
    fn install(&self, path: &str) -> Result<(), Error> {
        // 1. Read Config CRX.yaml @ path
        // self.crx = read_config(path);
        self.init(path);
        // let f = std::fs::read_to_string(path)
        // self.crx = serde_yaml::from_str(&f)?;

        // 2. Download CRX from Chrome Web Store
        let map = self.crx.clone();
        let tmp = Builder::new().prefix("example").tempdir()?;
        for (name, id) in map.into_iter() {
            println!("Downloading CRX: {}", name);
            // self.download_crx(id, tmp);
            // map.remove(name);
        }

        // 3. Unzip CRX
        // 4. Check if Chrome Remote Debugging is available already.
        //      - N: Alert user that this will Stop Chrome; Query for confirmation
        //          - N: Halt
        //          - Y: Restart Chrome in Remote Debugging Mode
        //      - Y: Continue
        // 5. Connect to Chrome
        // 6. Install CRX files
        Ok(())
    }

    fn export(&self, path: &str) -> Result<(), Error> {
        println!("You ran '{:#?}'!", "export");
        println!("Not Implemented Yet!");
        Ok(())
        // 1. Connect to Chrome
        // 2. Read Extensions
        // 3. Pack Extension
    }

    fn load(&self, path: &str) -> Result<(), Error> {
        println!("You ran '{:#?}'!", "export");
        println!("Not Implemented Yet!");
        Ok(())
        // 1. Connect to Chrome
        // 2. Read path
        // 3. unPack Extensions
        // 4. Install Extensions
    }

    fn cleanup(&self, path: &str) -> Result<(), Error> {
        println!("You ran '{:#?}'!", "export");
        println!("Not Implemented Yet!");
        Ok(())
        // 1. Cleanup CRX files
    }
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

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// fn read_config(path: &str) -> Vec<String> {
//     // Open the file in read-only mode with buffer.
//     // let f = std::fs::File::open(path).unwrap();
//     let f = std::fs::read_to_string(path).unwrap();
//     // Deserialize the YAML file into a String
//     let d: Value = serde_yaml::from_str::<Value>(&f).unwrap();
//     let mut ids: Vec<String> = Vec::<String>::new();

//     for (key, value) in d.get("crx").unwrap().as_mapping().unwrap() {
//         println!("{} / {}", key, value);
//         ids.push(value);
//     }

//     // println!("Type: {:?}", type_of());
//     // println!("Deserialized: {:?}", ids);

//     // // Path to your YAML file
//     // let file_path = "config.yaml";

//     // // Open the file
//     // let mut file = File::open(file_path);
//     // // Read the contents of the file
//     // let mut contents = String::new();
//     // file.read_to_string(&mut contents)?;

//     // Deserialize the YAML content into a Vec<String>
//     let data_array: Vec<String> = Vec::<String>::new();
//     return data_array;
// }

fn query_user() {
    // let mut input = String::new();
    // println!("This will stop chrome Do you want to continue? (y/n)");
    // std::io::stdin().read_line(&mut input).unwrap();
    // match input.trim() {
    //     "y" => println!("Continuing..."),
    //     "n" => println!("Exiting..."),
    //     _ => println!("Invalid input! Exiting..."),
    // }
}

// let helpmsg = r#"CRX is designed to deploy a Chrome browser instance in remote debugging mode \
//                     a side effect of this is that the current browser will need to be shutdown/killed first. \
//                     This is expected behavior."#;

/// CLI tool to install & administer Chrome Extensions via a headless Chrome instance.
#[derive(Clone, Parser, Debug)]
#[command(version, about, author, after_help = "", long_about = None)]
struct Args<'a> {

    /// Install new Chrome extensions described in the CRX.yaml.
    #[arg(short, long)]
    install: bool,

    /// Cleanup artifacts from previous runs.
    #[arg(short, long)]
    cleanup: bool,

    /// Import the specified Zipped CRX.
    #[arg(short, long)]
    load: bool,

    /// Export currently installed Chrome extensions as a Zipped CRX file.
    #[arg(short, long)]
    export: bool,

    /// Export currently installed Chrome extensions as a Zipped CRX file.
    #[arg(short, long)]
    update: bool,

    /// Export currently installed Chrome extensions as a Zipped CRX file
    #[arg(short, long, value_parser, default_value_t = "../data/crx.yaml")]
    path: &'a str,
}

fn main() -> Result<(), Error> {
    // let styles = styling::Styles::styled()
    //     .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    //     .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    //     .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
    //     .placeholder(styling::AnsiColor::Cyan.on_default());
    let args = Args::parse();
    let crx_yml = CrxYml::new();

    match args {
        Args { install: true, .. } => crx_yml.install(&args.path),
        // Args { cleanup: true, .. } => crx_yml.cleanup(&args.path),
        // Args { load: _, .. } => crx_yml.load(&args.path),
        // Args { export: true, .. } => crx_yml.export(&args.path),
        // Args { update: true, .. } => crx_yml.install(&args.path),
        _ => panic!("No valid command was given! Exiting..."),
    };

    Ok(())
}

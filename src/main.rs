use log::{info, error};
use std::env;
use std::process::{Command, exit};
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use spinners::{Spinner, Spinners};
use std::path::Path;
use regex::Regex;

fn exists(path: &str) -> bool {
    Path::new(&path).exists()
}

fn download_installer() {
    let mut spinner = Spinner::new(Spinners::Arc, "OK from bun installer! downloading...".into());
    let url = "https://bun.sh/install";

    let body = get(url)
    .expect("Failed to download")
    .text()
    .expect("Failed to read body");

    let mut file = File::create("install.sh").unwrap();
    file.write_all(body.as_bytes()).unwrap();
    spinner.stop();
}

fn download_bun() -> String {
    if !exists("./install.sh") {
        error!("Failed to execute install.sh. Please check network status.");
        exit(1);
    }

    let home = env::var("HOME").unwrap_or("".to_string());
    let bun_bin = format!("{}/.bun/bin/bun", home);

    if exists(&bun_bin) {
        return "exists".to_string();
    }

    let executer = Command::new("bash")
    .arg("./install.sh")
    .output()
    .expect("Failed to execute installer. TIP: bash is installed?");

    let information = match String::from_utf8(executer.stdout) {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid UTF-8: {}", e);
            exit(1);
        }
    };

    if information.contains("successfully to") {
        let re = Regex::new(r"successfully to (\S+)").unwrap();
        if let Some(caps) = re.captures(&information) {
            return caps[1].to_string();
        }
    }

    String::new() // empty
}

fn is_executeable() -> String {
    let home = env::var("HOME").unwrap_or("".to_string());
    let bun_bin = format!("{}/.bun/bin/bun", home);

    let executor = Command::new(bun_bin)
    .arg("--version")
    .output()
    .expect("NOT executeable");

    let out = match String::from_utf8(executor.stdout) {
        Ok(v) => v,
        Err(e) => {
            error!("{:?}", e);
            exit(1);
        }
    };

    return out
}

fn main() {
    colog::init();

    info!("The bun installer for termux!");

    // 1. CLI args
    let args: Vec<String> = env::args().collect();
    let no_os_check = args.iter().any(|a| a == "--no-os-check");

    // 2. uname -r
    let output = Command::new("uname")
    .arg("-r")
    .output()
    .expect("Failed to execute process");

    // 3. to UTF-8
    let os_str = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid UTF-8: {}", e);
            exit(1);
        }
    };

    let is_android = os_str.to_lowercase().contains("android");

    if is_android {
        info!("Detected Android kernel.");
    } else if no_os_check {
        info!("Skipping OS check because --no-os-check was used 🔧");
    } else {
        error!("Not Android.\nUse '--no-os-check' to override.");
        exit(1);
    }

    info!("Checking bun installer status...");

    download_installer();
    println!();
    info!("Main bun installer download was succeed!");
    info!("Calling bun installer...");

    let bun_path = match download_bun().as_str() {
        "exists" => "~/.bun/bin/bun [exists]".to_string(),
        other => other.to_string(),  // 그 외는 그냥 본문 반환
    };

    let real_path = match bun_path.as_str() {
        "~/.bun/bin/bun [exists]" => "~/.bun/bin/bun",
        other => other,
    };

    info!("Found bun path: {}\nReal path: {}", bun_path, real_path);
    info!("Checking it is executeable...");

    let out = is_executeable();
    let out_trim = out.trim();

    info!("{:?}", out_trim)
}

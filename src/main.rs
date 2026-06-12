use std::process::{self, Command};
use std::thread;
use std::time::Duration;

use v2x::{Katana, Output};

const CREATIVE_PROCS: &[&str] = &[
    "Creative.App.exe",
    "Creative Cloud.exe",
    "Creative Cloud Helper.exe",
    "Creative Cloud UI Helper.exe",
];

fn kill_creative_app() {
    for name in CREATIVE_PROCS {
        let status = Command::new("taskkill")
            .args(["/F", "/IM", name])
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status();
        if let Ok(s) = status {
            if s.success() {
                eprintln!("  Closed: {name}");
            }
        }
    }
}

fn is_creative_running() -> bool {
    let output = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Creative.App.exe"])
        .output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("Creative.App.exe")
        }
        Err(_) => false,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let action = args.get(1).map(|s| s.as_str()).unwrap_or("toggle");

    // Kill Creative App if running
    if is_creative_running() {
        eprintln!("Creative App is running — closing it...");
        kill_creative_app();
        thread::sleep(Duration::from_millis(500));
    }

    // Open device
    let mut dev = match Katana::open_auto() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ERROR: Cannot open Katana: {e}");
            eprintln!("Is it connected via USB?");
            process::exit(1);
        }
    };

    let result = match action {
        "status" => {
            match dev.get_output() {
                Ok(Output::Headphones) => println!("HEADPHONES"),
                Ok(Output::Speakers) => println!("SPEAKERS"),
                Err(e) => eprintln!("ERROR: {e}"),
            }
            Ok(())
        }
        "toggle" => {
            match dev.get_output() {
                Ok(Output::Headphones) => {
                    print!("HEADPHONES -> SPEAKERS... ");
                    dev.set_output(Output::Speakers)
                }
                Ok(Output::Speakers) => {
                    print!("SPEAKERS -> HEADPHONES... ");
                    dev.set_output(Output::Headphones)
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    Err(e)
                }
            }
        }
        "speakers" => {
            print!("Switching to SPEAKERS... ");
            dev.set_output(Output::Speakers)
        }
        "headphones" => {
            print!("Switching to HEADPHONES... ");
            dev.set_output(Output::Headphones)
        }
        _ => {
            eprintln!("Usage: katana-toggle [toggle|speakers|headphones|status]");
            process::exit(1);
        }
    };

    match result {
        Ok(()) => println!("OK"),
        Err(e) => {
            eprintln!("ERROR: {e}");
            process::exit(1);
        }
    }
}

// Unseen: A CLI tool to hide an app from the Dock on macOS.

use clap::Parser;
use std::process;
use std::process::Command;
use std::str;

/// Simple tool to hide an app from the Dock on macOS.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List all running applications.
    #[clap(short, long)]
    list: bool,
    /// Application to hide from Dock.
    #[clap()]
    apps: Vec<String>,
}

fn am_i_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn main() {
    let args = Args::parse();
    if args.list {
        list_running_apps();
    } else if !args.apps.is_empty() {
        // Check if the current user is root
        if !am_i_root() {
            eprintln!("This program must be run as root. Please use `sudo`.");
            process::exit(1);
        }
        for app_to_remove in args.apps {
            remove_app(&app_to_remove);
        }
    } else {
        println!("Please specify either --list to list all running apps or an app to hide.");
    }
}

fn list_running_apps() {
    let output = Command::new("osascript")
      .args(&["-e", "tell application \"System Events\" to get the name of every process whose background only is false"])
      .output()
      .expect("Failed to execute command.");

    let output_str = str::from_utf8(&output.stdout).unwrap();

    for app in output_str.split(", ") {
        println!("{}", app);
    }
}

fn remove_app(app_name: &str) {
    // Set application to be a UIElement
    let output = Command::new("lsappinfo")
        .args(&["setinfo", "-app", app_name, "-uielement"])
        .output()
        .expect("Failed to set application as a UIElement.");

    if !output.status.success() {
        eprintln!(
            "Error setting app as a UIElement: {}",
            str::from_utf8(&output.stderr).unwrap()
        );
        return;
    }

    // Change application type
    let output = Command::new("lsappinfo")
        .args(&["setinfo", "-app", app_name, "ApplicationType=x"])
        .output()
        .expect("Failed to change application type.");

    if !output.status.success() {
        eprintln!(
            "Error changing application type: {}",
            str::from_utf8(&output.stderr).unwrap()
        );
        return;
    }

    println!("Removed app: {}", app_name);
}
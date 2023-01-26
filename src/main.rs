use clap::Parser;
use core::time;
use notify_rust::{Notification, NotificationHandle, Urgency};
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::{env, fs, thread};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pid: u32,
}

/// Returns the process name as it appears from the command line arguments (i.e.
/// `argv[0]`).
fn get_process_name(pid: u32) -> Result<String, Box<dyn Error>> {
    let pathname = format!("/proc/{}/cmdline", pid);
    let contents = fs::read_to_string(pathname)?;
    let tokens = contents.split('\0').collect::<Vec<_>>();
    if tokens.len() < 1 {
        return Err(Box::new(io::Error::new(
            ErrorKind::InvalidData,
            format!("/proc/{}/cmdline contained unknown format", pid),
        )));
    }
    Ok(String::from(tokens[0]))
}

/// Suspends the process until the process identified by the process id `pid`
/// has terminated. Upon termination a desktop notification is published.
fn wait_on_process(pid: u32) -> Result<(), Box<dyn Error>> {
    let pathname = format!("/proc/{}", pid);
    let pth = Path::new(&pathname);

    if pth.is_dir() {
        let command_name = get_process_name(pid)?;

        println!("Waiting on [{}] {}", pid, command_name);
        while pth.is_dir() {
            thread::sleep(time::Duration::from_millis(250));
        }

        Notification::new()
            .summary(&format!("{} Finished", command_name))
            .urgency(Urgency::Critical)
            .body("The command has terminated")
            .show()?;
    }
    Ok(())
}

/// Runs a new child process with the commands specified by `cmd`, then produces
/// a desktop notification about its exit status.
fn subprocess(cmd: &Vec<String>) -> Result<NotificationHandle, Box<dyn Error>> {
    println!("Running Command: {}", cmd.join(" "));
    if cmd.len() < 1 {
        return Err(Box::new(io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid input",
        )));
    }
    let command_name = &cmd[0];
    let res = Command::new(&cmd[0]).args(&cmd[1..]).status();
    match res {
        Ok(i) => {
            if i.success() {
                let notified = Notification::new()
                    .summary(&format!("{} Sucess", command_name))
                    .urgency(Urgency::Critical)
                    .body("The command ran successfully")
                    .show()?;
                return Ok(notified);
            } else {
                let notified = Notification::new()
                    .summary(&format!("{} FAIL", command_name))
                    .urgency(Urgency::Critical)
                    .body("The command failed to run correctly")
                    .show()?;
                return Ok(notified);
            }
        }
        Err(error) => {
            return Err(Box::new(error));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().collect();
    let cmd_start = raw_args.iter().position(|r| r == "--");
    if let Some(i) = cmd_start {
        subprocess(&raw_args[(i + 1)..].to_vec())?;
    } else {
        wait_on_process(Args::parse().pid).expect("Failed to wait on process");
    }
    Ok(())
}

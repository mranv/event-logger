mod constants;
mod registry_utils;
mod eventlog_utils;
mod models; // only if you're using the structured log approach

use clap::{Parser, Subcommand};
use crate::constants::{COMPANY_NAME, IVS_AGENT_NAME};
use crate::registry_utils::{source_exists, create_event_source, delete_event_source};
use crate::eventlog_utils::{EventLogger, EVENT_TYPE_WARNING};

use rand::Rng;

/// A simple CLI for demonstration: 
///   cargo run -- create-source
///   cargo run -- write-entry
///   cargo run -- delete-source
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateSource,
    WriteEntry,
    DeleteSource,
}

fn main() {
    env_logger::init(); // if you want logs
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateSource => create_source(),
        Commands::WriteEntry => write_entry(),
        Commands::DeleteSource => delete_source(),
    }

    println!("Application is closed");
}

fn create_source() {
    if source_exists(COMPANY_NAME, IVS_AGENT_NAME) {
        println!("Source '{IVS_AGENT_NAME}' already exists in '{COMPANY_NAME}'");
    } else {
        println!("Creating source '{IVS_AGENT_NAME}' under '{COMPANY_NAME}'");
        match create_event_source(COMPANY_NAME, IVS_AGENT_NAME) {
            Ok(_) => println!("Source created successfully."),
            Err(e) => eprintln!("Error creating source: {e:?}"),
        }
    }
}

fn write_entry() {
    let mut rng = rand::thread_rng();
    let random_id = rng.gen_range(1..=999);

    // We want to write an event to "Infopercept" from "IvsAgent"
    if let Some(logger) = EventLogger::register(IVS_AGENT_NAME, COMPANY_NAME) {
        let message = format!("Hello from Rust! random_id={random_id}");
        logger.write_entry(&message, EVENT_TYPE_WARNING);
        println!("Wrote a 'WARNING' event: {message}");
    } else {
        eprintln!("Could not register source '{IVS_AGENT_NAME}' under '{COMPANY_NAME}'");
    }
}

fn delete_source() {
    if source_exists(COMPANY_NAME, IVS_AGENT_NAME) {
        println!("Deleting source '{IVS_AGENT_NAME}' from '{COMPANY_NAME}'");
        if let Err(e) = delete_event_source(COMPANY_NAME, IVS_AGENT_NAME) {
            eprintln!("Error deleting source: {e:?}");
        }
    } else {
        println!("Source '{IVS_AGENT_NAME}' does not exist in '{COMPANY_NAME}'");
    }
}

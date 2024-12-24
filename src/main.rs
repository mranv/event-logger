mod constants;
mod models;
mod registry_utils;
mod eventlog_utils;

use clap::{Parser, Subcommand};
use rand::Rng;
use serde_json::json;
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::constants::{COMPANY_NAME, IVS_AGENT_NAME};
use crate::models::TaskExecutionLog;
use crate::registry_utils::{source_exists, create_event_source, delete_event_source};
use crate::eventlog_utils::{EventLogger, EVENT_TYPE_WARNING};

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
    ReadEntries,
    TestMySource,
    ClearAll,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateSource => create_source(),
        Commands::WriteEntry => write_event_log(),
        Commands::DeleteSource => delete_source(),
        Commands::ReadEntries => {
            println!("ReadEntries not implemented in this sample.");
        }
        Commands::TestMySource => test_my_log_source(),
        Commands::ClearAll => {
            println!("ClearAll not implemented in this sample.");
        }
    }

    println!("Application is closed");
}

fn create_source() {
    if source_exists(COMPANY_NAME, IVS_AGENT_NAME) {
        println!("Source '{}' already exists in '{}'", IVS_AGENT_NAME, COMPANY_NAME);
        if let Some(logger) = EventLogger::register(IVS_AGENT_NAME, COMPANY_NAME) {
            logger.write_entry("Test message - warning", EVENT_TYPE_WARNING);
        }
    } else {
        println!("Creating source '{}' under '{}'", IVS_AGENT_NAME, COMPANY_NAME);
        match create_event_source(COMPANY_NAME, IVS_AGENT_NAME) {
            Ok(_) => println!("Source created successfully."),
            Err(e) => eprintln!("Failed to create source: {:?}", e),
        }
    }
}

fn write_event_log() {
    let mut rng = rand::thread_rng();
    let src_id = rng.gen_range(1..10);
    let task_id = rng.gen_range(1..100);

    // We'll name the source "SRC-<src_id>"
    let source_name = format!("SRC-{src_id}");
    println!("Writing event with source={source_name}");

    if let Some(logger) = EventLogger::register(&source_name, COMPANY_NAME) {
        add_task_execution_log(
            &logger,
            &task_id.to_string(),
            &task_id.to_string(),
            "Success",
            "Task executed successfully",
            src_id,
        );
    } else {
        eprintln!("Could not register event source '{source_name}'");
    }
}

fn delete_source() {
    if source_exists(COMPANY_NAME, IVS_AGENT_NAME) {
        println!("Source '{}' exists under '{}'. Deleting...", IVS_AGENT_NAME, COMPANY_NAME);
        if let Err(e) = delete_event_source(COMPANY_NAME, IVS_AGENT_NAME) {
            eprintln!("Failed to delete event source: {:?}", e);
        }
    } else {
        println!("Source '{}' does NOT exist.", IVS_AGENT_NAME);
    }
}

fn test_my_log_source() {
    let my_source = "MySource";
    let my_log = "MyNewLog";

    if source_exists(my_log, my_source) {
        println!("Source '{}' already exists in '{}'. Deleting...", my_source, my_log);
        if let Err(e) = delete_event_source(my_log, my_source) {
            eprintln!("Could not delete source: {:?}", e);
        }
        println!("MySource is deleted from MyNewLog");
    } else {
        println!("Source '{}' does NOT exist in '{}'. Creating...", my_source, my_log);
        match create_event_source(my_log, my_source) {
            Ok(_) => println!("Source created."),
            Err(e) => eprintln!("Failed to create source: {:?}", e),
        }
    }
}

fn add_task_execution_log(
    logger: &EventLogger,
    task_id: &str,
    command: &str,
    status: &str,
    message: &str,
    src_id: i32,
) {
    let mut metadata = HashMap::new();
    metadata.insert("srcId".to_string(), json!(src_id));

    let exec_log = TaskExecutionLog {
        task_id: task_id.to_string(),
        command: command.to_string(),
        status: status.to_string(),
        message: message.to_string(),
        metadata,
        created_at: OffsetDateTime::now_utc(),
    };

    // Serialize to JSON
    let json_data = match serde_json::to_string_pretty(&exec_log) {
        Ok(s) => s,
        Err(_) => "Failed to serialize TaskExecutionLog".to_string(),
    };

    // Write with a "warning" event type for demonstration
    logger.write_entry(&json_data, EVENT_TYPE_WARNING);
}

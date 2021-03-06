//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 15 2022
//
pub mod action;
pub mod utils;

use crate::action::{dump};

use tokio_socketcan::CANSocket;

use clap::{Parser, Subcommand};

/// canutils provides several common CAN commands
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
    #[clap(short = 'd', long = "device", value_parser)]
    pub device: String,
}

pub enum Driver {
    SocketCan,
}

/// Command to run
#[derive(Subcommand)]
pub enum Command {
    /// Print CAN frames to console
    Dump,
    /// Send CAN frames to the selected interface
    Send,
    /// Bridge different CAN interfaces together
    Bridge,
}

/// Subcommand context
pub struct CommandContext {
    pub socket: CANSocket,
    pub device: String,
}

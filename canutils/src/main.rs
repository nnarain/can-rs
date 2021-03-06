use anyhow::Context;
//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 15 2022
//
use tokio;
use tokio_socketcan::{CANSocket, CanFrame};
use futures_util::StreamExt;
use clap::Parser;

use canutils::{CommandContext, Args, Command, action};


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = Args::parse();
    let device = args.device;

    let mut socket = CANSocket::open(&device)
                                .with_context(|| format!("Failed to open CAN interface {}", device))?;

    let ctx = CommandContext {socket, device};

    match args.cmd {
        Command::Dump => Ok(action::dump::run(ctx).await?),
        Command::Send => Ok(()),
        Command::Bridge => Ok(()),
    }
}

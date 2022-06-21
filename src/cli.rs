use std::{net::SocketAddr, path::PathBuf};

use log::LevelFilter;
use clap::{Parser, Subcommand, Args, ArgGroup, ArgAction};
use actix_web::http::StatusCode;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct App {
    /// Address on which the server listens.
    #[clap(long, short, default_value = "0.0.0.0:80")]
    pub bind_addr: SocketAddr,

    /// Lob verbosity.
    #[clap(long, default_value_t = LevelFilter::Info)]
    pub log_level: LevelFilter,

    /// Number of worker used to handle requests.
    #[clap(long, short, default_value_t = 1)]
    pub workers: usize,

    #[clap(subcommand)]
    pub command: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Answer with HTTP status.
    Status(Status),

    /// Answer with a payload.
    Payload(Payload)
}

#[derive(Args, Debug)]
pub struct Status {
    /// Status codes to send back as a response. Multiple means iterating one after the other.
    #[clap(default_value = "200")]
    pub status: Vec<StatusCode>,

    /// Time to wait before sending back the response ina human readable format. e.g.: 5s.
    #[clap(long, short)]
    pub wait: Option<humantime::Duration>
}

#[derive(Args, Debug)]
#[clap(group(ArgGroup::new("body").required(true)))]
pub struct Payload {
    /// Response content to send as a response.
    #[clap(long, short, group="body")]
    pub data: Option<String>,

    /// Read (until EOF) content from STDIN and send it back as a response.
    #[clap(long="in", short='i', action=ArgAction::SetTrue, group="body")]
    pub stdin: Option<bool>,

    /// File containing the response content to send.
    #[clap(long, short, group="body")]
    pub file: Option<PathBuf>,

    /// Time to wait before sending back the response ina human readable format. e.g.: 5s.
    #[clap(long, short)]
    pub wait: Option<humantime::Duration>
}

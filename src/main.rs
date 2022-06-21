
extern crate clap;
extern crate actix_web;
extern crate simplelog;

mod cli;

use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, net::SocketAddr, io::{BufReader, Read}};

use clap::Parser;
use log::info;
use simplelog::{TermLogger, Config};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, http::StatusCode, middleware};

struct ServerOptions {
    bind_addr: SocketAddr,
    workers: usize,
}

#[derive(Clone, Debug)]
struct StatusCommandState {
    status: Vec<StatusCode>,
    current_index: Arc<AtomicUsize>,
    wait: Option<std::time::Duration>
}

#[derive(Clone, Debug)]
struct PayloadCommandState {
    payload: String,
    wait: Option<std::time::Duration>
}

async fn status_handler(state: web::Data<StatusCommandState>) -> impl Responder {
    if let Some(duration) = state.wait {
        tokio::time::sleep(duration).await;
    }

    let current_index = state.current_index.fetch_add(1, Ordering::Relaxed);
    let index = current_index % state.status.len();
    HttpResponse::new(*state.status.get(index).unwrap())
}

async fn payload_handler(state: web::Data<PayloadCommandState>) -> impl Responder {
    if let Some(duration) = state.wait {
        tokio::time::sleep(duration).await;
    }
    
    HttpResponse::Ok().body(state.payload.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = cli::App::parse();
    let opts = ServerOptions { bind_addr: app.bind_addr, workers: app.workers };

    // Logging initialization.
    TermLogger::init(app.log_level, Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    
    match app.command {
        cli::Command::Status(cmd) => run_status_command(opts, cmd).await,
        cli::Command::Payload(cmd) => run_payload_command(opts, cmd).await
    }
}

async fn run_status_command(opts: ServerOptions, cmd: cli::Status) -> std::io::Result<()> {
    info!("Configuring status HTTP server: {:?}", cmd);

    let state = StatusCommandState {
        status: cmd.status.clone(), 
        current_index: Arc::new(AtomicUsize::new(0)),
        wait: cmd.wait.map(|d| d.into())
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .default_service(web::to(status_handler))
    })
    .workers(opts.workers)
    .bind(opts.bind_addr)?
    .run();

    info!("Server starting to listen on {}", opts.bind_addr);
    server.await
}

async fn run_payload_command(opts: ServerOptions, cmd: cli::Payload) -> std::io::Result<()> {
    info!("Configuring payload HTTP server: {:?}", cmd);

    let body = (cmd.data, cmd.file, cmd.stdin);
    let payload = match body {
        (Some(data), _, _) => data,
        (_, Some(file), _) => std::fs::read_to_string(file).unwrap(),
        (_, _, Some(true)) => {
            let mut s = String::new();
            BufReader::new(std::io::stdin()).read_to_string(&mut s).unwrap();
            s
        },
        _ => unreachable!()
    };

    let state = PayloadCommandState { payload, wait: cmd.wait.map(|d| d.into())};

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .default_service(web::to(payload_handler))
    })
    .workers(opts.workers)
    .bind(opts.bind_addr)?
    .run();

    info!("Server starting to listen on {}", opts.bind_addr);
    server.await
}
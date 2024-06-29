use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::Router,
  };
  use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
  use rust_embed::Embed;
  use log::{debug, error, info, trace, warn};
  use clap::Parser;
  use std::process;
  use dotenvy::dotenv;
  use std::env;



  mod utils;
  use utils::logging;

  mod api;
  use api::upload;

  mod ws; 
  use ws::connection;

  static INDEX_HTML: &str = "index.html";
  
  #[derive(Embed)]
  #[folder = "client/dist"]
  struct Assets;
  /// Full-stack program to play audio files from trunk-recorder. 
  #[derive(Parser, Debug)]
  #[command(version, about, long_about = None)]
  struct Args {
    /// Address to bind to
    #[arg(short, long, default_value= "0.0.0.0")]
    address: String,

    /// Port to behind to
    #[arg(short, long, default_value = "7373")]
    port: String,

    /// Logging Level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

  
  #[tokio::main]
  async fn main() {
    dotenv().map_err(|e| {
        error!("Failed to load .env file: {}", e);
        process::exit(1);
    }).unwrap();
    let args: Args = Args::parse();
    debug!("Parsing arguments...");
    trace!("Arguments: {:?}", args);

    match args.log_level.as_str() {
        "trace" => logging::setup_logger(log::LevelFilter::Trace).unwrap(),
        "debug" => logging::setup_logger(log::LevelFilter::Debug).unwrap(),
        "info" => logging::setup_logger(log::LevelFilter::Info).unwrap(),
        "warn" => logging::setup_logger(log::LevelFilter::Warn).unwrap(),
        "error" => logging::setup_logger(log::LevelFilter::Error).unwrap(),
        _ => logging::setup_logger(log::LevelFilter::Info).unwrap(),
        }

    info!("Starting radio-scout...");


    let layer = connection::return_layer();

    


    let app = Router::new().fallback(static_handler).layer(layer);
    debug!("Creating router...");

    let addr = args.address;
    let port = args.port;



  
    let listener = tokio::net::TcpListener::bind(addr.clone() + ":" + &port.clone()).await;

    match listener {
        Ok(listener) => {
            info!("Listening on: {}", addr.clone() + ":" + &port);
            let server =axum::serve(listener, app.into_make_service()).await;

            match server {
                Ok(_) => {
                    info!("Server started successfully");
                },
                Err(e) => {
                    error!("Failed to start server: {}", e);
                    crate::process::exit(1);
                }
            }
        },
        Err(e) => {
            error!("Failed to bind to address: {}", e);
            crate::process::exit(1);
        }
    }


  }
  
  async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
  
    if path.is_empty() || path == INDEX_HTML {
      return index_html().await;
    }
  
    match Assets::get(path) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
  
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => {
        if path.contains('.') {
          return not_found().await;
        }
  
        index_html().await
      }
    }
  }
  
  async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
      Some(content) => Html(content.data).into_response(),
      None => not_found().await,
    }
  }
  
  async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
  }
  
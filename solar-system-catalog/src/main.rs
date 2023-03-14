use std::future;
use std::net::SocketAddr;
use std::sync::Arc;

use aws_smithy_http_server::AddExtensionLayer;
use aws_smithy_http_server::instrumentation::InstrumentExt;
use clap::Parser;
use futures_util::stream::StreamExt;
use solar_system_catalog::{
    get_planet, setup_tracing, tls, State,
};
use solar_system_catalog_server_sdk::service::SolarSystemCatalog;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyper server bind address.
    #[clap(short, long, action, default_value = "127.0.0.1")]
    address: String,
    /// Hyper server bind port.
    #[clap(short, long, action, default_value = "13734")]
    port: u16,
    /// Hyper server TLS certificate path. Must be a PEM file.
    #[clap(long, default_value = "certs/localhost.crt")]
    tls_cert_path: String,
    /// Hyper server TLS private key path. Must be a PEM file.
    #[clap(long, default_value = "certs/localhost.key")]
    tls_key_path: String,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    setup_tracing();
    // Create the shared state.
    //
    let shared_state = Arc::new(State::default());
    // Setup shared state and middlewares.
    let app = SolarSystemCatalog::builder()
        // Build a registry containing implementations to all the operations in the service. These
        // are async functions or async closures that take as input the operation's input and
        // return the operation's output.
        .get_planet(get_planet)
        .instrument()
        .build()
        .layer(&AddExtensionLayer::new(shared_state));

    let addr: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");

    let acceptor = tls::acceptor(&args.tls_cert_path, &args.tls_key_path);
    let listener = tls_listener::TlsListener::new(
        acceptor,
        hyper::server::conn::AddrIncoming::bind(&addr).expect("could not bind"),
    )
    .filter(|conn| {
        if let Err(err) = conn {
            eprintln!("connection error: {:?}", err);
            future::ready(false)
        } else {
            future::ready(true)
        }
    });
    let server = hyper::Server::builder(hyper::server::accept::from_stream(listener))
        .serve(app.into_make_service());
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}

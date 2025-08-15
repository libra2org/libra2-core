use clap::Parser;
use std::net::SocketAddr;

/// Tiny CLI to run the txnstream gRPC server
#[derive(Parser, Debug)]
#[command(name = "txnstream-server")]
struct Args {
    /// Address to bind the gRPC server to, e.g. 127.0.0.1:50052
    #[arg(long, default_value = "127.0.0.1:50052")]
    addr: SocketAddr,

    /// REST endpoint of the local node, used by the server to fetch data
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    rest: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    libra2_txnstream_server::run_server(args.addr, args.rest).await
}

use hyper::Server;
use s3s::service::S3ServiceBuilder;
use std::net::SocketAddr;

use clap::Parser;

mod banyan_s3_auth;
#[macro_use]
mod multipart_uploads;
//mod mutex_memory_blockstore;
mod wnfs_s3_service;

/// start banyan s3 service
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Authentication endpoint for API keys
    #[arg(long)]
    auth_endpoint: String,

    /// Key endpoint for WNFS decryption keys
    #[arg(long)]
    key_endpoint: String,
}

// TODO add logging
#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let s3_service = {
        let wnfs_s3_service = wnfs_s3_service::WnfsS3Service::new().await;
        let banyan_s3_auth =
            banyan_s3_auth::BanyanS3Auth::new(args.auth_endpoint, args.key_endpoint)
                .await
                .map_err(|e| anyhow::anyhow!("couldn't connect to auth database: {}", e))
                .unwrap();
        let mut service_builder = S3ServiceBuilder::new(wnfs_s3_service);
        service_builder.set_auth(banyan_s3_auth);
        // service_builder.set_base_domain("localhost:3000"); ???
        service_builder.build()
    };

    // Then bind and serve...
    let server = Server::bind(&addr).serve(s3_service.into_shared().into_make_service());

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

use hyper::Server;
use s3s::service::S3ServiceBuilder;
use std::net::SocketAddr;

mod banyan_s3_auth;
mod wnfs_s3_service;

// async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello World")))
// }

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let s3_service = {
        let wnfs_s3_service = wnfs_s3_service::WnfsS3Service::new();
        let banyan_s3_auth = banyan_s3_auth::BanyanS3Auth::new();
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

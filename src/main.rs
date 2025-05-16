use std::net::SocketAddr;
use clap::Parser;
use warp::Filter;
use warp::http::StatusCode;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Port to listen on
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let dir = std::env::current_dir().expect("Failed to get current dir");
    println!("Serving directory {:?} on http://0.0.0.0:{}", dir, args.port);

    // check if / request
    let index_dir = dir.clone();
    let index_route = warp::path::end().and_then(move || {
        let index_html_file = index_dir.join("index.html");
        let index_htm_file = index_dir.join("index.htm");

        async move {
            if index_html_file.exists() {
                // try load existing index.html file

                Ok(warp::reply::with_header(
                    std::fs::read(index_html_file).expect("Failed to read index.html"),
                    "Content-Type",
                    "text/html",
                )) as Result<_, warp::Rejection>
            } else if index_htm_file.exists() {
                // try load existing index.html file

                Ok(warp::reply::with_header(
                    std::fs::read(index_htm_file).expect("Failed to read index.htm"),
                    "Content-Type",
                    "text/html",
                ))
            } else {
                Err(warp::reject::not_found())
            }
        }
    });

    // static files
    let static_files = warp::fs::dir(dir);

    // combine and build routes
    let routes = index_route.or(static_files).recover(handle_rejection);

    // 0.0.0.0:port
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    warp::serve(routes)
        .run(addr)
        .await;
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("404 Not Found", StatusCode::NOT_FOUND))
    } else {
        Ok(warp::reply::with_status("500 Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

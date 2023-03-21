use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde_json;

use std::fs::File;
use std::io::Read;


mod plot;
 
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // let data = serde_json::from_str(&_event.body);
    plot::run();

    let image_path = "/tmp/5.png";
    let mut image_file = File::open(image_path).map_err(Box::new)?;
    let mut image_data = Vec::new();
    image_file.read_to_end(&mut image_data).map_err(Box::new)?;

    // dbg!(data);
    dbg!("Hello AWS  Lambda HTTP request");

    let resp = Response::builder()
    .status(200)
    .header("content-type", "image/png")
    .body(Body::from(image_data))
    .map_err(Box::new)?;




    // let resp = Response::builder()
    //     .status(200)
    //     .header("content-type", "text/html")
    //     .body("Hello AWS Lambda HTTP request".into())
    //     .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

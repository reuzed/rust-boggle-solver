use lambda_http::{service_fn, Error};
mod http_handler;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::tracing::init_default_subscriber();

    let func = service_fn(function_handler);
    if let Err(err) = lambda_http::run(
        func
    ).await {
        eprint!("run error {:?}",err);
        return Err(err);
    }
    Ok(())
}

use lambda_http::{Body, Error, Request, Response};
use rust_boggle_solver::{board::Board, solver::solve};
use serde::{Deserialize};
use serde_json;

#[derive(Deserialize)]
struct SolveRequest {
    r1: [char; 4],
    r2: [char; 4],
    r3: [char; 4],
    r4: [char; 4],
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let route = event.uri().path();
    let method = event.method().as_str();
    match (route, method) {
        ("/solve", "POST") => {
            let solve_request: SolveRequest = serde_json::from_slice(event.body().as_ref())?;

            let board = Board::new(
                solve_request.r1,
                solve_request.r2,
                solve_request.r3,
                solve_request.r4,
            );

            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&solve(&board))?))
                .map_err(Box::new)?;

            Ok(resp)
        }
        ("/", "GET") => {
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(Body::from("Hello from rust-boggle-solver"))
                .map_err(Box::new)?;

            Ok(resp)
        }
        _ => {
            eprintln!("Route: {} Method: {}", route, method);
            Err("Route/Method not matched.".into())
        }
    }
}

use lambda_http::{Body, Error, Request, RequestExt, Response};
use rust_boggle_solver::{board::Board, solver::solve};
use serde::{Deserialize, Serialize};
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
    let solve_request: SolveRequest = serde_json::from_slice(event.body().as_ref())?;

    let board = Board::new(
        solve_request.r1,
        solve_request.r2,
        solve_request.r3,
        solve_request.r4,
    );

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from(serde_json::to_string(&solve(&board))?))
        .map_err(Box::new)?;

    Ok(resp)
}

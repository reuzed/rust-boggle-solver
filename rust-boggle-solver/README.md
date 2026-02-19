# Introduction

rust-boggle-solver is a Rust project that implements an AWS Lambda function in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example apigw-request
```

For generic events, where you define the event data structure, you can create a JSON file with the data you want to test with. For example:

```bash
curl "http://localhost:9000/generate" -H "Content-Type application/json" -d '{"quantity":2, "max_attempts":100}'
[[{"arr":[["c","s","i","k"],["o","s","n","u"],["a","u","x","i"],["t","w","o","s"]]},[{"path":[{"x":0,"y":0},{"x":1,"y":0}],"word":"cs"},{"path":[{"x":0,"y"...

curl "http://localhost:9000/solve" -H "Content-Type: application/json" -d '{"r1":["a","b","c","d"],"r2":["a","b","c","d"],"r3":["a","b","c","d"],"r4":["a","b","c","d"]}'
[{"path":[{"x":0,"y":0},{"x":1,"y":0}],"word":"ab"},{"path":[{"x":0,"y":0},{"x":1,"y":1}],"word":"ab"},{"path":[{"x":0,"y":0},{"x":0,"y":1}],"word":"aa"}...
```

Then, run `cargo lambda invoke --data-file ./data.json` to invoke the function with the data in `data.json`.

For HTTP events, you can also call the function directly with cURL or any other HTTP client. For example:

```bash
curl http://localhost:9000
```

Check if other cargo lambda is running:

```bash
lsof -i :9000
```

Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying

### Build and deploy with a public Function URL

To deploy with a **public HTTPS Function URL** (so you can call it without API Gateway):

```bash
# 1. Build for Lambda (Linux)
cargo lambda build --release

# 2. Deploy with Function URL enabled (prints URL when complete)
# --include words.txt is required: the solver/generator need this at runtime
cargo lambda deploy --enable-function-url --include words.txt rust-boggle-solver
```

The `--enable-function-url` flag creates a public HTTPS endpoint like:
`https://<id>.lambda-url.<region>.on.aws/`

Your `CargoLambda.toml` is preconfigured with `enable_function_url = true` and `include = ["words.txt"]`, so you can also run:

```bash
cargo lambda deploy rust-boggle-solver
```

### IAM permissions for Function URL

If deploy fails with permission errors, your AWS user/role needs these extra actions (in addition to the [standard deploy permissions](https://www.cargo-lambda.info/commands/deploy.html)):

- `lambda:AddPermission`
- `lambda:CreateFunctionUrlConfig`
- `lambda:GetFunctionUrlConfig`

**Note:** As of late 2025, Lambda Function URLs require both `lambda:InvokeFunctionUrl` and `lambda:InvokeFunction` in the resource policy. If you get 403 Forbidden when curling the URL, add the InvokeFunction permission:

```bash
aws lambda add-permission --function-name rust-boggle-solver \
  --statement-id FunctionURLInvokeAllowPublicAccess \
  --action lambda:InvokeFunction --principal '*' \
  --region <your-region>
```

### Calling the deployed function

Once deployed, use the printed URL:

```bash
# Health check
curl https://<your-url>.lambda-url.<region>.on.aws/

# Generate boards
curl -X POST https://<your-url>.lambda-url.<region>.on.aws/generate \
  -H "Content-Type: application/json" \
  -d '{"quantity":2, "max_attempts":100}'

# Solve a board
curl -X POST https://<your-url>.lambda-url.<region>.on.aws/solve \
  -H "Content-Type: application/json" \
  -d '{"r1":["a","b","c","d"],"r2":["e","f","g","h"],"r3":["i","j","k","l"],"r4":["m","n","o","p"]}'
```

### Debugging: "expects a JSON payload from API Gateway or Function URLs"

If you see this error when invoking, it usually means the function was called with raw JSON instead of an HTTP request. The `lambda_http` handler expects events from:

- Lambda Function URLs (HTTP)
- API Gateway
- Elastic Load Balancing

Use the Function URL with `curl` or a browser; do not use `aws lambda invoke` with custom JSON payloads.

Read more about deploying in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/deploy.html).

## Cloudflare Assignment

### Run
1. Have [Rust and Cargo](https://www.rust-lang.org/) installed
2. Run `cargo run -p job-queue-api --release`

### Run tests
Run `cargo test -p queue`.

### Assumptions
1. I assumed that for the `/jobs/enqueue` endpoint, only the job type needed to be specified in the request body.
2. I assumed that instead of naming the QUEUE_CONSUMER header as such, it could be queue-consumer (due to an issue with a library)
3. I assumed the JobType could be an enum inside of the code as long as it is publicly exposed in the JSON as a string
4. I assumed that Job ID's can start at 0 and increment as they are added to satisfy the uniqueness

### Postman Collection
There is a provided Postman collection which has the endpoints provided.

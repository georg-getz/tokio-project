# tokio-project

Simple program running 4 spawn task in parallel. All the spawned tasks have a chance of causing a critical
error, which is determined purely on randomness.

Use `cargo run` to run the program and `cargo test` for the tests.
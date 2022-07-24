
# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)


## Frameworks included
**[Astra](https://github.com/ibraheemdev/astra)** - Synchronous HTTP server built on top of hyper<br>
# Results
|   Concurrency: 10   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Astra** (FAIL)|0|FAIL|FAIL|0|






## Benchmarking tool
The benchmarks have been performed using [wrk](https://github.com/wg/wrk), locally. 

Check the raw output from wrk [here](https://github.com/Ishtmeet-Singh/rust-framework-benchmarks/tree/master/perf).


## Try it yourself
Everything is automated, including adding a framework, generating `md` file output, and running the tests without having to start all the servers at once!

To run the tests locally, please follow the steps - 

1. Download the repository as a zip, or clone/fork it.
2. `cd rust-framework-benchmarks`
3. `cargo build --release --bins`
4. Run the main script and you're good to go..
`./target/release/main` or `cargo run --release --bin main` 

All the output will be stored in `perf/{name}/{concurrency}.txt*`

## Machine used
M1 Max MacBook Pro 2021 - 64GB ram, 10 CPU cores and 32 GPU cores

## Suggestions and changes
All the suggestions, code changes or additions of another web framework is appreciated. I'd like to keep the code as close as a real world scenario, instead of optimising it to the metal.

To add a new library/framework, please make sure to use the `PORT` provided through the benchmark dynamically. Default is `3000` for all. You can change it in `config.json`.

Also, to add a framework, add an entry inside `config.json` for the benchmarks to detect it.

```json
[
  {
    // Name of your framework. Displayed in the readme and during logs
    "name": "Axum", 
    // Default for all the frameworks
    "port": 3000,
    // the name of the file that your framework is located
    "binary": "axum", 
    // Github or Crates.io or Website
    "url": "https://github.com/tokio-rs/axum" 
  }
]
```

```rs
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
 Server::build().bind("hello-world", format!("127.0.0.1:{}", port_number))
```


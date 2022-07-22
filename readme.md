# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.


## Frameworks included
**[Actix Web](https://actix.rs)**
**[Hyper](https://hyper.rs)**
**[Axum](https://github.com/tokio-rs/axum)**
**[Ntex](https://github.com/ntex-rs/ntex)**
**[Rocket](https://rocket.rs)**
**[Tide](https://github.com/http-rs/tide)**
**[Viz](https://github.com/viz-rs/viz)**
**[Warp](https://github.com/seanmonstar/warp)**
# Results
|   Concurrency: 10   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Ntex**|225,508|31.99us|0.91ms|6,787,926|
|**Actix Web**|218,494|34.21us|2.39ms|6,576,869|
|**Hyper**|201,448|36.71us|557.00us|6,063,527|
|**Axum**|199,114|37.57us|664.00us|5,993,306|
|**Warp**|198,503|37.23us|662.00us|5,974,891|
|**Viz**|197,735|40.33us|10.07ms|5,951,843|
|**Rocket**|155,413|52.70us|5.72ms|4,677,874|
|**Tide**|131,920|67.79us|8.21ms|3,970,779|


|   Concurrency: 50   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|235,058|126.23us|3.05ms|7,052,265|
|**Actix Web**|233,649|119.51us|3.14ms|7,033,305|
|**Warp**|229,900|127.93us|2.29ms|6,897,531|
|**Viz**|227,604|129.45us|3.05ms|6,851,296|
|**Axum**|227,360|129.67us|2.28ms|6,843,962|
|**Ntex**|224,752|124.93us|3.08ms|6,765,430|
|**Rocket**|184,113|189.99us|3.25ms|5,542,080|
|**Tide**|152,357|241.85us|4.34ms|4,586,299|


|   Concurrency: 100   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|233,902|234.52us|4.85ms|7,017,991|
|**Warp**|229,245|237.94us|5.96ms|6,878,445|
|**Viz**|226,401|242.83us|5.68ms|6,815,711|
|**Axum**|225,633|243.90us|5.96ms|6,792,699|
|**Actix Web**|215,028|250.34us|5.84ms|6,451,741|
|**Ntex**|210,214|256.14us|5.04ms|6,307,336|
|**Rocket**|175,194|373.20us|5.14ms|5,256,955|
|**Tide**|95,485|749.41us|11.98ms|2,865,018|


|   Concurrency: 500   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|231,712|548.69us|9.70ms|6,954,850|
|**Warp**|227,345|558.75us|11.85ms|6,823,672|
|**Viz**|224,034|568.16us|11.29ms|6,724,576|
|**Axum**|223,949|568.43us|8.97ms|6,721,430|
|**Ntex**|198,799|527.55us|11.54ms|5,966,654|
|**Actix Web**|193,529|546.13us|11.60ms|5,808,814|
|**Rocket**|161,657|0.87ms|11.43ms|4,857,418|
|**Tide**|47,185|3.88ms|48.44ms|1,416,607|


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



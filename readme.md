
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
|   Concurrency: 10   |   Duration: 24 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Actix Web**|219,642|33.91us|751.00us|5,293,324|
|**Ntex**|213,247|37.74us|2.03ms|5,139,371|
|**Hyper**|206,174|36.00us|1.23ms|4,968,906|
|**Viz**|203,385|36.85us|568.00us|4,901,655|
|**Warp**|201,732|36.88us|1.27ms|4,861,843|
|**Axum**|195,067|39.68us|3.33ms|4,701,275|
|**Rocket**|150,906|59.33us|10.70ms|3,636,864|
|**Tide**|132,205|66.58us|2.38ms|3,186,198|


|   Concurrency: 50   |   Duration: 24 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|240,761|121.82us|2.71ms|5,802,680|
|**Actix Web**|236,801|118.52us|3.12ms|5,683,784|
|**Warp**|236,041|123.51us|2.21ms|5,688,898|
|**Viz**|233,325|125.37us|2.98ms|5,623,628|
|**Ntex**|224,563|136.17us|13.98ms|5,390,355|
|**Axum**|216,343|142.22us|5.97ms|5,214,368|
|**Rocket**|185,711|186.84us|4.47ms|4,475,976|
|**Tide**|134,612|280.36us|3.51ms|3,244,356|


|   Concurrency: 100   |   Duration: 24 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|238,911|228.48us|5.39ms|5,758,873|
|**Warp**|235,921|230.38us|5.62ms|5,663,016|
|**Ntex**|232,672|233.88us|4.51ms|5,585,248|
|**Viz**|231,702|236.41us|6.07ms|5,584,897|
|**Axum**|228,132|242.70us|10.12ms|5,476,523|
|**Actix Web**|207,686|259.92us|5.62ms|5,006,263|
|**Rocket**|176,636|361.96us|4.66ms|4,240,171|
|**Tide**|69,550|1.43ms|25.12ms|1,676,449|


|   Concurrency: 250   |   Duration: 24 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Warp**|231,388|549.93us|11.35ms|5,556,133|
|**Hyper**|230,561|552.69us|11.41ms|5,535,983|
|**Viz**|228,069|559.87us|9.54ms|5,476,440|
|**Axum**|224,464|568.40us|4.92ms|5,391,927|
|**Ntex**|190,511|570.15us|8.22ms|4,575,103|
|**Actix Web**|177,295|598.19us|9.60ms|4,257,215|
|**Rocket**|156,353|0.91ms|11.32ms|3,756,387|
|**Tide**|44,413|6.08ms|51.39ms|1,066,699|


|   Concurrency: 500   |   Duration: 24 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|232,306|547.41us|10.86ms|5,578,519|
|**Warp**|231,787|547.67us|11.46ms|5,565,354|
|**Viz**|227,854|559.15us|8.17ms|5,470,751|
|**Axum**|222,472|575.21us|11.00ms|5,342,592|
|**Ntex**|205,928|516.88us|11.01ms|4,945,130|
|**Actix Web**|176,063|601.11us|10.45ms|4,227,782|
|**Rocket**|156,519|0.90ms|10.96ms|3,762,165|
|**Tide**|44,233|5.54ms|61.15ms|1,062,315|


|   Concurrency: 750   |   Duration: 24 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Actix Web**|130,405|1.02ms|127.90ms|3,138,348|
|**Ntex**|123,820|1.10ms|134.99ms|2,980,128|
|**Hyper**|103,253|1.34ms|11.37ms|2,485,026|
|**Viz**|101,062|1.36ms|11.02ms|2,434,656|
|**Warp**|100,425|1.38ms|12.82ms|2,419,343|
|**Axum**|100,364|1.36ms|89.51ms|2,411,876|
|**Rocket**|89,536|1.51ms|14.42ms|2,156,349|
|**Tide**|61,738|3.72ms|43.99ms|1,483,671|






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


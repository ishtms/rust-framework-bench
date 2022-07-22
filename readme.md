
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
|**Ntex**|226,774|31.89us|0.96ms|6,825,962|
|**Actix Web**|221,388|33.22us|463.00us|6,663,856|
|**Hyper**|201,572|36.93us|3.68ms|6,067,266|
|**Viz**|199,677|37.41us|573.00us|6,010,350|
|**Axum**|199,348|37.55us|832.00us|6,000,416|
|**Warp**|197,756|37.64us|2.21ms|5,952,524|
|**Rocket**|154,473|53.13us|6.67ms|4,649,621|
|**Tide**|131,539|66.81us|0.89ms|3,959,336|


|   Concurrency: 50   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|234,313|126.89us|3.10ms|7,053,296|
|**Actix Web**|232,603|120.14us|3.14ms|7,001,720|
|**Warp**|230,258|127.37us|2.51ms|6,931,058|
|**Viz**|227,095|130.22us|2.56ms|6,836,129|
|**Axum**|226,992|130.44us|3.58ms|6,832,989|
|**Ntex**|225,334|125.12us|3.29ms|6,782,938|
|**Rocket**|181,237|194.55us|6.09ms|5,437,529|
|**Tide**|150,411|246.08us|3.28ms|4,512,587|


|   Concurrency: 100   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|232,896|235.72us|4.66ms|7,011,106|
|**Warp**|227,894|240.62us|10.21ms|6,860,665|
|**Viz**|225,483|244.28us|5.99ms|6,787,724|
|**Axum**|225,307|244.24us|6.06ms|6,760,239|
|**Actix Web**|214,151|251.67us|6.00ms|6,446,935|
|**Ntex**|208,249|258.86us|6.00ms|6,248,425|
|**Rocket**|174,342|379.88us|5.85ms|5,231,231|
|**Tide**|97,950|729.94us|15.66ms|2,939,283|


|   Concurrency: 500   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|228,923|556.28us|10.53ms|6,870,931|
|**Warp**|226,057|564.08us|11.19ms|6,785,238|
|**Axum**|223,950|568.36us|11.56ms|6,721,774|
|**Viz**|223,941|568.09us|9.69ms|6,721,023|
|**Ntex**|199,810|525.28us|12.03ms|5,997,464|
|**Actix Web**|192,180|550.78us|12.02ms|5,768,134|
|**Rocket**|160,587|0.88ms|11.59ms|4,825,822|
|**Tide**|47,464|3.88ms|46.68ms|1,425,285|


|   Concurrency: 750   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|229,380|554.43us|9.72ms|6,885,411|
|**Warp**|226,507|560.42us|8.09ms|6,798,132|
|**Axum**|223,450|569.99us|10.98ms|6,706,654|
|**Viz**|223,328|569.85us|11.40ms|6,703,864|
|**Ntex**|199,046|527.00us|19.66ms|5,974,006|
|**Actix Web**|196,359|537.74us|10.92ms|5,893,522|
|**Rocket**|157,657|0.90ms|10.79ms|4,739,131|
|**Tide**|48,098|3.78ms|42.73ms|1,444,373|






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


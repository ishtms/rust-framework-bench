# Rust framework benchmarks

Benchmarking utility to test the performance of all the rust web frameworks. Built with [rust](https://rust-lang.org) 🚀.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)



### **(Last updated: Fri Jul 14 2023 10:48:53)** 

## Frameworks included
**[Actix-Web](https://actix.rs)** - A powerful, pragmatic, and extremely fast web framework for Rust<br>
**[Hyper](https://hyper.rs)** - Fast and safe HTTP for the Rust language<br>
**[Axum](https://github.com/tokio-rs/axum)** - Web application framework that focuses on ergonomics and modularity<br>
**[Warp](https://github.com/seanmonstar/warp)** - A super-easy, composable, web server framework for warp speeds<br>
**[Ntex](https://github.com/ntex-rs/ntex)** - Framework for composable network services<br>
**[Rocket](https://rocket.rs)** - Write fast, secure web applications without sacrificing flexibility, usability, or type safety<br>
**[Tide](https://github.com/http-rs/tide)** - Minimal and pragmatic Rust web application framework built for rapid development<br>
**[May-MiniHttp](https://github.com/Xudong-Huang/may_minihttp)** - Mini http server that implemented on top of may<br>
**[Viz](https://github.com/viz-rs/viz)** - Fast, robust, flexible, lightweight web framework for Rust<br>
**[Tokio-minihttp](https://github.com/tokio-rs/tokio-minihttp)** - Proof-of-concept implementation of a simple HTTP/1.1 server using Tokio.<br>
**[Thruster](https://github.com/thruster-rs/Thruster)** - A fast and intuitive rust web framework<br>
**[Salvo](https://github.com/salvo-rs/salvo)** - Simple and powerful Rust web backend framework<br>
**[Poem](https://github.com/poem-web/poem)** - A full-featured and easy-to-use web framework<br>
**[Gotham](https://github.com/gotham-rs/gotham)** - A flexible web framework that promotes stability, safety, security and speed.<br>
**[Astra](https://github.com/ibraheemdev/astra)** - Synchronous HTTP server built on top of hyper<br>
**[Nickel](https://github.com/nickel-org/nickel.rs)** - An expressjs inspired framework for rust<br>
**[Roa](https://github.com/Hexilee/roa)** - Roa is an async web framework inspired by koajs, lightweight but powerful.<br>
# Results
|   Concurrency: 16   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Warp** |186,015|0.09ms|0.01ms|10.21ms|0.04ms|0.17ms|0.26ms|0.61ms|3,719,629|22.88MB/Sec|0|
|**Axum** |181,315|0.09ms|0.01ms|2.85ms|0.04ms|0.18ms|0.26ms|0.49ms|3,625,922|22.31MB/Sec|0|
|**Poem** |179,327|0.09ms|0.02ms|8.99ms|0.05ms|0.18ms|0.28ms|0.82ms|3,585,318|22.23MB/Sec|0|
|**Hyper** |178,835|0.09ms|0.01ms|6.57ms|0.05ms|0.20ms|0.31ms|0.82ms|3,576,386|15.01MB/Sec|0|
|**Roa** |177,468|0.09ms|0.01ms|5.83ms|0.05ms|0.20ms|0.31ms|0.76ms|3,547,848|14.89MB/Sec|0|
|**Gotham** |176,294|0.09ms|0.02ms|7.41ms|0.05ms|0.19ms|0.29ms|0.85ms|3,524,817|28.08MB/Sec|0|
|**Actix-Web** |170,241|0.09ms|0.01ms|23.20ms|0.07ms|0.22ms|0.36ms|1.12ms|3,404,418|21.11MB/Sec|0|
|**Astra** |159,379|0.08ms|0.02ms|17.75ms|0.05ms|0.17ms|0.24ms|0.48ms|3,186,849|16.11MB/Sec|0|
|**Nickel** |154,231|0.08ms|0.02ms|7.70ms|0.04ms|0.16ms|0.24ms|0.69ms|3,084,006|22.21MB/Sec|0|
|**Salvo** |136,435|0.06ms|0.01ms|1478.46ms|2.19ms|0.22ms|0.55ms|3.79ms|2,727,582|16.91MB/Sec|0|
|**Tide** |97,537|0.16ms|0.03ms|10.38ms|0.09ms|0.37ms|0.61ms|1.53ms|1,950,396|12.00MB/Sec|0|
|**Thruster** |818|4.15ms|0.20ms|6710.95ms|128.50ms|59.79ms|269.78ms|2592.12ms|16,365|0.00B/Sec|16381|
|**Ntex** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Rocket** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**May-MiniHttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Viz** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 32   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Roa** |194,769|0.16ms|0.01ms|3.57ms|0.06ms|0.32ms|0.44ms|0.74ms|3,894,923|16.35MB/Sec|0|
|**Axum** |186,186|0.17ms|0.01ms|10.08ms|0.07ms|0.35ms|0.46ms|0.78ms|3,723,196|22.91MB/Sec|0|
|**Viz** |184,720|0.17ms|0.01ms|4.64ms|0.07ms|0.37ms|0.50ms|1.09ms|3,693,383|22.90MB/Sec|0|
|**Hyper** |184,089|0.17ms|0.01ms|8.84ms|0.08ms|0.38ms|0.53ms|1.24ms|3,680,719|15.45MB/Sec|0|
|**Warp** |183,451|0.17ms|0.01ms|10.41ms|0.09ms|0.38ms|0.51ms|1.21ms|3,668,067|22.57MB/Sec|0|
|**May-MiniHttp** |181,845|0.18ms|0.01ms|10.82ms|0.09ms|0.39ms|0.54ms|1.31ms|3,636,004|17.34MB/Sec|0|
|**Salvo** |179,435|0.18ms|0.01ms|11.24ms|0.09ms|0.40ms|0.58ms|1.50ms|3,587,341|22.25MB/Sec|0|
|**Poem** |179,071|0.18ms|0.02ms|8.09ms|0.08ms|0.38ms|0.54ms|1.25ms|3,579,219|22.20MB/Sec|0|
|**Gotham** |177,781|0.18ms|0.02ms|8.67ms|0.08ms|0.38ms|0.52ms|1.22ms|3,553,591|28.31MB/Sec|0|
|**Actix-Web** |175,296|0.18ms|0.02ms|8.30ms|0.08ms|0.39ms|0.55ms|1.35ms|3,505,456|21.73MB/Sec|0|
|**Astra** |154,599|0.20ms|0.02ms|21.67ms|0.10ms|0.42ms|0.62ms|1.59ms|3,091,509|15.63MB/Sec|0|
|**Nickel** |151,747|0.08ms|0.02ms|5.99ms|0.04ms|0.17ms|0.25ms|0.70ms|3,034,079|21.85MB/Sec|0|
|**Ntex** |144,057|0.22ms|0.02ms|29.19ms|0.12ms|0.48ms|0.73ms|1.72ms|2,880,883|17.72MB/Sec|0|
|**Rocket** |125,474|0.25ms|0.02ms|21.67ms|0.15ms|0.56ms|0.91ms|2.30ms|2,508,845|29.56MB/Sec|0|
|**Tide** |101,242|0.32ms|0.03ms|6.81ms|0.12ms|0.59ms|0.79ms|1.85ms|2,024,018|12.46MB/Sec|0|
|**Thruster** |817|10.64ms|0.10ms|13155.85ms|303.43ms|202.98ms|1001.91ms|8499.36ms|16,354|0.00B/Sec|16386|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 64   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Roa** |188,698|0.34ms|0.02ms|3.04ms|0.13ms|0.71ms|0.90ms|1.48ms|3,772,871|15.84MB/Sec|0|
|**May-MiniHttp** |187,573|0.34ms|0.01ms|19.51ms|0.16ms|0.73ms|0.96ms|2.17ms|3,749,473|17.89MB/Sec|0|
|**Viz** |183,183|0.35ms|0.02ms|6.48ms|0.14ms|0.72ms|0.92ms|1.73ms|3,661,475|22.71MB/Sec|0|
|**Hyper** |182,956|0.35ms|0.02ms|11.95ms|0.15ms|0.76ms|1.02ms|2.15ms|3,658,183|15.35MB/Sec|0|
|**Warp** |179,072|0.36ms|0.02ms|20.45ms|0.17ms|0.77ms|1.06ms|2.61ms|3,580,216|22.03MB/Sec|0|
|**Poem** |175,422|0.36ms|0.02ms|9.07ms|0.16ms|0.78ms|1.05ms|2.43ms|3,507,046|21.75MB/Sec|0|
|**Actix-Web** |175,036|0.37ms|0.02ms|15.24ms|0.16ms|0.77ms|1.06ms|2.60ms|3,499,340|21.70MB/Sec|0|
|**Gotham** |174,956|0.37ms|0.02ms|13.19ms|0.16ms|0.76ms|1.00ms|2.13ms|3,498,669|27.86MB/Sec|0|
|**Axum** |170,096|0.38ms|0.02ms|19.08ms|0.20ms|0.86ms|1.25ms|2.90ms|3,401,524|20.93MB/Sec|0|
|**Ntex** |160,336|0.40ms|0.02ms|63.80ms|0.25ms|0.84ms|1.19ms|2.92ms|3,205,888|19.73MB/Sec|0|
|**Astra** |156,218|0.40ms|0.02ms|18.14ms|0.16ms|0.81ms|1.12ms|2.49ms|3,123,023|15.79MB/Sec|0|
|**Salvo** |155,599|0.41ms|0.02ms|8.83ms|0.20ms|0.90ms|1.28ms|2.95ms|3,111,112|19.29MB/Sec|0|
|**Nickel** |153,045|0.08ms|0.02ms|9.67ms|0.04ms|0.17ms|0.26ms|0.77ms|3,059,917|22.04MB/Sec|0|
|**Rocket** |126,227|0.51ms|0.02ms|9.90ms|0.19ms|0.92ms|1.20ms|2.54ms|2,524,275|29.73MB/Sec|0|
|**Tide** |92,412|0.69ms|0.04ms|12.22ms|0.23ms|1.13ms|1.49ms|3.86ms|1,848,002|11.37MB/Sec|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Thruster** |0|2441.23ms|0.66ms|6711.09ms|3227.02ms|6711.09ms|6711.09ms|6711.09ms|22|0.00B/Sec|52|


|   Concurrency: 128   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |191,298|0.67ms|0.01ms|14.37ms|0.27ms|1.35ms|1.71ms|3.46ms|3,824,575|18.24MB/Sec|0|
|**Hyper** |186,574|0.69ms|0.01ms|5.80ms|0.28ms|1.41ms|1.66ms|2.49ms|3,730,044|15.66MB/Sec|0|
|**Roa** |179,394|0.71ms|0.02ms|6.35ms|0.29ms|1.44ms|1.68ms|2.52ms|3,586,567|15.06MB/Sec|0|
|**Salvo** |175,965|0.73ms|0.02ms|11.27ms|0.31ms|1.50ms|1.90ms|4.03ms|3,516,493|21.82MB/Sec|0|
|**Warp** |174,936|0.73ms|0.02ms|13.62ms|0.30ms|1.46ms|1.74ms|2.84ms|3,498,324|21.52MB/Sec|0|
|**Viz** |172,724|0.74ms|0.02ms|14.39ms|0.32ms|1.53ms|2.04ms|4.72ms|3,453,835|21.41MB/Sec|0|
|**Actix-Web** |170,737|0.75ms|0.02ms|21.84ms|0.32ms|1.52ms|2.06ms|5.11ms|3,413,546|21.17MB/Sec|0|
|**Ntex** |169,786|0.75ms|0.02ms|103.78ms|0.37ms|1.52ms|1.97ms|4.43ms|3,394,477|20.89MB/Sec|0|
|**Gotham** |165,746|0.77ms|0.02ms|15.66ms|0.32ms|1.54ms|1.94ms|4.07ms|3,313,533|26.40MB/Sec|0|
|**Poem** |165,374|0.77ms|0.02ms|11.79ms|0.32ms|1.52ms|1.91ms|4.10ms|3,305,923|20.50MB/Sec|0|
|**Axum** |159,304|0.80ms|0.01ms|22.34ms|0.38ms|1.65ms|2.31ms|6.02ms|3,184,963|19.60MB/Sec|0|
|**Astra** |155,853|0.82ms|0.02ms|19.66ms|0.32ms|1.68ms|2.36ms|4.93ms|3,116,128|15.76MB/Sec|0|
|**Nickel** |146,694|0.08ms|0.02ms|8.54ms|0.05ms|0.18ms|0.30ms|0.88ms|2,932,458|21.12MB/Sec|0|
|**Rocket** |127,689|1.00ms|0.02ms|14.84ms|0.34ms|1.71ms|2.28ms|5.02ms|2,552,815|30.08MB/Sec|0|
|**Tide** |88,189|1.45ms|0.04ms|12.77ms|0.35ms|2.16ms|2.73ms|6.18ms|1,763,248|10.85MB/Sec|0|
|**Thruster** |818|37.39ms|0.17ms|13108.80ms|503.32ms|674.38ms|3201.12ms|10310.94ms|16,343|0.00B/Sec|16471|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 256   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Actix-Web** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Hyper** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Axum** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Warp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Ntex** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Rocket** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Tide** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**May-MiniHttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Viz** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Thruster** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Salvo** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Poem** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Gotham** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Astra** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Nickel** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Roa** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|






## Benchmarking tool
The benchmarks have been performed using [rewrk](https://github.com/ChillFish8/rewrk), locally. 

Check the raw output from rewrk [here](https://github.com/Ishtmeet-Singh/rust-framework-benchmarks/tree/master/perf).


## Try it yourself
Everything is automated, including adding a framework, generating `md` file output, and running the tests without having to start all the servers at once!

Pleas make sure you do not have capped soft limit or hard limit for file descriptors, this may cause benchmarks with high concurrency (-c) fail with OS error 54,
to fix that - 

## Linux
1. Open `/etc/sysctl.conf`
2. Add `fs.file-max = 65536`
3. Reboot your pc and verify it after rebooting with `sysctl -p`

## Mac
1. Check your current limit - `launchctl limit maxfiles`
2. The first entry is the soft limit, and the second entry is hard limit. In most cases the hard limit is unlimited, it's the soft limit that we need to change
3. Update the hard limit `sudo launchctl limit maxfiles 65536 200000`
4. Reboot.

Alternatively, if you wish to change the soft limit to only run the benchmark one time, you can change it for your current terminal session by
`ulimit -n 65536`.
This doesn't require boot, and will reset back to the default (usually 2560) after restarting the terminal.

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

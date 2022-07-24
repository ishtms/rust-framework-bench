
# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)


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
|   Concurrency: 10   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**May-MiniHttp** |235,648|29.22us|776.00us|4,736,581|
|**Tokio-minihttp** |232,239|34.11us|1.10ms|4,668,143|
|**Actix-Web** |224,918|33.07us|0.98ms|4,520,838|
|**Astra** |216,361|34.30us|0.95ms|4,348,813|
|**Ntex** |212,492|38.29us|2.24ms|4,271,053|
|**Poem** |205,506|36.87us|0.87ms|4,130,635|
|**Roa** |204,852|36.53us|2.53ms|4,117,626|
|**Hyper** |204,825|36.22us|789.00us|4,117,072|
|**Viz** |203,405|36.95us|0.94ms|4,088,546|
|**Axum** |203,300|37.00us|609.00us|4,086,292|
|**Salvo** |202,529|37.08us|0.87ms|4,070,834|
|**Gotham** |201,790|37.50us|0.89ms|4,056,036|
|**Thruster** |201,642|36.72us|1.19ms|4,032,956|
|**Warp** |200,405|37.08us|0.89ms|4,028,158|
|**Nickel** |165,625|47.22us|1.54ms|3,329,182|
|**Rocket** |157,943|51.53us|1.22ms|3,174,762|
|**Tide** |132,598|66.32us|1.26ms|2,665,238|


|   Concurrency: 50   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |246,010|122.56us|3.26ms|4,945,307|
|**Actix-Web** |240,409|116.24us|3.08ms|4,832,699|
|**Hyper** |239,073|123.09us|10.19ms|4,781,888|
|**Roa** |238,927|123.30us|10.08ms|4,778,874|
|**Warp** |237,486|123.02us|3.28ms|4,774,028|
|**Thruster** |235,972|124.39us|3.27ms|4,743,501|
|**Ntex** |235,645|121.45us|3.54ms|4,736,764|
|**Poem** |232,814|126.28us|2.26ms|4,680,059|
|**Salvo** |232,419|126.10us|3.37ms|4,672,112|
|**Viz** |232,290|125.97us|3.23ms|4,669,496|
|**Axum** |232,063|126.49us|2.56ms|4,664,861|
|**Astra** |229,805|125.07us|3.04ms|4,619,556|
|**Gotham** |228,238|128.09us|2.86ms|4,565,259|
|**Rocket** |187,055|186.74us|2.82ms|3,760,161|
|**May-MiniHttp** |178,296|162.37us|3.20ms|3,584,132|
|**Nickel** |170,152|53.56us|3.18ms|3,420,330|
|**Tide** |126,717|312.00us|11.17ms|2,547,331|


|   Concurrency: 250   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |246,524|542.59us|11.42ms|6,165,677|
|**Roa** |230,863|561.03us|10.30ms|5,774,358|
|**Thruster** |229,312|562.71us|11.20ms|5,735,575|
|**Hyper** |227,603|567.79us|11.37ms|5,693,512|
|**Salvo** |225,378|574.50us|9.75ms|5,659,272|
|**Poem** |224,342|578.87us|8.91ms|5,611,777|
|**Axum** |224,282|576.80us|11.57ms|5,609,777|
|**Viz** |223,658|578.69us|10.93ms|5,595,360|
|**Gotham** |221,153|586.37us|10.52ms|5,531,353|
|**Astra** |218,480|383.67us|11.14ms|5,464,477|
|**Warp** |216,154|597.66us|11.04ms|5,407,171|
|**Ntex** |202,407|640.25us|11.95ms|5,063,459|
|**Nickel** |168,277|54.39us|11.79ms|4,225,405|
|**Rocket** |157,985|0.91ms|11.13ms|3,955,296|
|**Actix-Web** |146,162|0.89ms|14.80ms|3,657,475|
|**May-MiniHttp** |133,933|0.96ms|11.25ms|3,352,477|
|**Tide** |42,297|6.58ms|52.68ms|1,057,900|


|   Concurrency: 700   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |236,782|1.54ms|12.84ms|5,925,632|
|**Astra** |215,419|391.42us|12.79ms|5,391,856|
|**Nickel** |163,328|66.66us|174.30ms|4,088,206|
|**Hyper** |158,300|2.24ms|23.25ms|3,960,196|
|**Roa** |157,051|2.26ms|21.07ms|3,929,140|
|**Thruster** |156,376|2.27ms|19.37ms|3,910,608|
|**Warp** |154,759|2.29ms|20.95ms|3,873,414|
|**Viz** |154,001|2.30ms|14.58ms|3,852,010|
|**Axum** |152,893|2.32ms|18.86ms|3,827,680|
|**Salvo** |151,746|2.33ms|21.53ms|3,807,425|
|**Poem** |149,537|2.37ms|17.77ms|3,741,057|
|**Gotham** |148,215|2.40ms|19.91ms|3,710,441|
|**Rocket** |131,263|2.74ms|17.71ms|3,289,927|
|**Ntex** |128,846|2.75ms|17.73ms|3,228,909|
|**Actix-Web** |127,935|2.77ms|16.29ms|3,208,440|
|**May-MiniHttp** |127,003|2.78ms|19.85ms|3,180,523|
|**Tide** |42,284|18.42ms|130.13ms|1,058,387|






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


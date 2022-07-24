
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
|   Concurrency: 16   |   Duration: 25 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |234,497|0.07ms|14.48ms|5,862,276|
|**May-MiniHttp** |228,819|0.07ms|1.24ms|5,720,374|
|**Roa** |219,832|0.07ms|2.17ms|5,495,616|
|**Warp** |219,297|0.07ms|1.56ms|5,482,285|
|**Hyper** |217,388|0.07ms|2.06ms|5,434,528|
|**Thruster** |216,138|0.07ms|1.27ms|5,403,342|
|**Salvo** |215,532|0.07ms|1.79ms|5,388,173|
|**Viz** |214,519|0.07ms|1.46ms|5,362,811|
|**Axum** |213,842|0.07ms|1.59ms|5,345,952|
|**Poem** |210,839|0.08ms|1.24ms|5,270,857|
|**Gotham** |209,076|0.08ms|1.79ms|5,226,762|
|**Actix-Web** |207,196|0.08ms|4.98ms|5,179,758|
|**Ntex** |193,838|0.08ms|11.78ms|4,845,802|
|**Astra** |190,064|0.07ms|1.83ms|4,751,477|
|**Nickel** |186,016|0.06ms|2.30ms|4,650,243|
|**Rocket** |171,207|0.09ms|2.06ms|4,280,087|
|**Tide** |134,272|0.12ms|1.73ms|3,356,726|


|   Concurrency: 64   |   Duration: 25 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |242,722|0.26ms|3.72ms|6,067,833|
|**May-MiniHttp** |234,731|0.27ms|4.18ms|5,867,996|
|**Hyper** |229,195|0.28ms|3.89ms|5,729,639|
|**Roa** |228,064|0.28ms|4.01ms|5,701,388|
|**Thruster** |226,181|0.28ms|3.94ms|5,654,305|
|**Warp** |226,163|0.28ms|3.87ms|5,653,868|
|**Salvo** |224,501|0.28ms|3.04ms|5,612,262|
|**Axum** |224,116|0.29ms|2.67ms|5,602,684|
|**Actix-Web** |223,848|0.29ms|4.34ms|5,595,971|
|**Viz** |219,984|0.29ms|2.62ms|5,499,352|
|**Poem** |218,769|0.29ms|3.89ms|5,468,988|
|**Ntex** |217,805|0.29ms|103.82ms|5,444,920|
|**Gotham** |214,287|0.30ms|4.35ms|5,356,978|
|**Nickel** |186,998|0.06ms|3.03ms|4,674,705|
|**Astra** |179,632|0.36ms|2.91ms|4,490,660|
|**Rocket** |146,229|0.44ms|7.98ms|3,655,632|
|**Tide** |97,365|0.66ms|9.17ms|2,434,010|


|   Concurrency: 128   |   Duration: 25 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,599|0.53ms|7.42ms|6,089,436|
|**May-MiniHttp** |236,240|0.54ms|7.15ms|5,905,676|
|**Actix-Web** |228,360|0.56ms|7.79ms|5,708,672|
|**Ntex** |220,217|0.58ms|125.46ms|5,505,228|
|**Roa** |218,617|0.59ms|8.07ms|5,465,052|
|**Hyper** |215,887|0.59ms|7.41ms|5,396,849|
|**Thruster** |213,715|0.60ms|7.27ms|5,342,532|
|**Viz** |212,911|0.60ms|8.41ms|5,322,489|
|**Salvo** |211,744|0.60ms|5.90ms|5,293,220|
|**Warp** |210,396|0.61ms|8.06ms|5,259,540|
|**Axum** |207,210|0.62ms|12.33ms|5,179,934|
|**Poem** |207,180|0.62ms|8.09ms|5,179,160|
|**Gotham** |200,417|0.64ms|6.06ms|5,010,104|
|**Nickel** |186,724|0.06ms|7.25ms|4,667,794|
|**Astra** |174,679|0.73ms|7.87ms|4,366,625|
|**Rocket** |120,315|1.06ms|7.11ms|3,007,759|
|**Tide** |73,773|1.73ms|9.09ms|1,844,223|


|   Concurrency: 256   |   Duration: 25 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**May-MiniHttp** |241,806|1.06ms|23.66ms|6,044,393|
|**Tokio-minihttp** |240,776|1.06ms|21.19ms|6,018,553|
|**Actix-Web** |223,875|1.14ms|107.09ms|5,596,474|
|**Ntex** |222,624|1.15ms|126.19ms|5,564,985|
|**Hyper** |199,904|1.28ms|22.10ms|4,997,109|
|**Roa** |194,190|1.32ms|22.52ms|4,854,185|
|**Thruster** |193,758|1.32ms|19.63ms|4,843,460|
|**Warp** |190,623|1.34ms|23.01ms|4,764,914|
|**Nickel** |187,469|0.06ms|21.71ms|4,686,220|
|**Viz** |181,519|1.41ms|21.67ms|4,537,536|
|**Poem** |177,414|1.44ms|19.41ms|4,434,948|
|**Astra** |171,940|0.85ms|22.45ms|4,298,098|
|**Salvo** |171,863|1.49ms|19.04ms|4,296,163|
|**Axum** |171,616|1.49ms|20.03ms|4,290,043|
|**Gotham** |166,112|1.54ms|20.22ms|4,152,882|
|**Rocket** |101,880|2.51ms|22.87ms|2,546,793|
|**Tide** |71,957|3.56ms|22.50ms|1,798,814|






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


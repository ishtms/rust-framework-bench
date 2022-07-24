
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
|   Concurrency: 16   |   Duration: 60 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |236,389|0.07ms|5.45ms|14,183,234|
|**May-MiniHttp** |229,454|0.07ms|0.96ms|13,767,145|
|**Hyper** |220,386|0.07ms|1.15ms|13,223,102|
|**Roa** |219,596|0.07ms|1.10ms|13,175,694|
|**Warp** |219,381|0.07ms|1.26ms|13,162,793|
|**Thruster** |218,111|0.07ms|2.24ms|13,086,519|
|**Viz** |215,710|0.07ms|1.29ms|12,942,531|
|**Axum** |215,094|0.07ms|1.50ms|12,905,499|
|**Salvo** |214,961|0.07ms|1.62ms|12,897,578|
|**Poem** |210,962|0.08ms|1.18ms|12,657,636|
|**Actix-Web** |208,601|0.08ms|7.82ms|12,515,900|
|**Gotham** |208,358|0.08ms|1.53ms|12,501,403|
|**Ntex** |195,232|0.08ms|26.10ms|11,713,826|
|**Astra** |188,695|0.07ms|3.27ms|11,321,755|
|**Nickel** |186,924|0.06ms|3.02ms|11,215,287|
|**Rocket** |170,331|0.09ms|1.29ms|10,219,823|
|**Tide** |134,098|0.12ms|1.26ms|8,045,840|


|   Concurrency: 64   |   Duration: 60 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,744|0.26ms|3.54ms|14,624,339|
|**May-MiniHttp** |232,906|0.27ms|4.24ms|13,974,231|
|**Warp** |227,919|0.28ms|3.76ms|13,674,840|
|**Hyper** |227,611|0.28ms|7.75ms|13,656,401|
|**Roa** |226,971|0.28ms|6.52ms|13,617,933|
|**Thruster** |226,299|0.28ms|3.73ms|13,577,720|
|**Actix-Web** |224,747|0.28ms|4.41ms|13,484,564|
|**Viz** |223,126|0.29ms|4.46ms|13,387,270|
|**Axum** |222,972|0.29ms|3.84ms|13,378,103|
|**Salvo** |221,842|0.29ms|4.16ms|13,310,353|
|**Ntex** |217,296|0.29ms|125.35ms|13,037,532|
|**Poem** |217,252|0.29ms|2.86ms|13,034,954|
|**Gotham** |215,821|0.30ms|3.79ms|12,949,104|
|**Nickel** |186,879|0.06ms|2.97ms|11,212,572|
|**Astra** |178,476|0.35ms|3.15ms|10,708,427|
|**Rocket** |147,346|0.43ms|17.76ms|8,840,607|
|**Tide** |95,598|0.67ms|4.75ms|5,735,788|


|   Concurrency: 128   |   Duration: 60 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,534|0.53ms|7.94ms|14,611,747|
|**May-MiniHttp** |236,861|0.54ms|7.47ms|14,211,403|
|**Actix-Web** |228,437|0.56ms|7.72ms|13,705,928|
|**Ntex** |223,970|0.57ms|125.59ms|13,438,043|
|**Roa** |215,280|0.59ms|5.27ms|12,916,533|
|**Thruster** |214,816|0.60ms|8.15ms|12,888,707|
|**Hyper** |212,710|0.60ms|6.60ms|12,762,189|
|**Warp** |211,319|0.61ms|5.80ms|12,678,830|
|**Poem** |211,149|0.61ms|7.36ms|12,668,720|
|**Viz** |210,881|0.61ms|7.29ms|12,652,504|
|**Salvo** |210,297|0.61ms|6.41ms|12,617,481|
|**Axum** |208,534|0.61ms|7.24ms|12,511,789|
|**Gotham** |201,142|0.64ms|8.01ms|12,068,152|
|**Nickel** |186,274|0.06ms|7.56ms|11,176,103|
|**Astra** |174,753|0.72ms|8.06ms|10,485,044|
|**Rocket** |116,706|1.10ms|26.82ms|7,002,298|
|**Tide** |74,768|1.71ms|15.32ms|4,486,028|


|   Concurrency: 256   |   Duration: 60 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**May-MiniHttp** |241,908|1.06ms|23.81ms|14,513,734|
|**Tokio-minihttp** |241,472|1.06ms|18.64ms|14,487,662|
|**Actix-Web** |228,239|1.12ms|72.13ms|13,693,607|
|**Ntex** |223,572|1.14ms|127.45ms|13,413,761|
|**Hyper** |199,153|1.29ms|19.49ms|11,948,885|
|**Thruster** |197,289|1.30ms|20.53ms|11,836,841|
|**Roa** |195,641|1.31ms|21.35ms|11,737,811|
|**Warp** |189,565|1.35ms|18.66ms|11,373,418|
|**Nickel** |186,661|0.06ms|19.67ms|11,199,201|
|**Poem** |180,560|1.42ms|23.49ms|10,833,240|
|**Viz** |180,147|1.42ms|20.08ms|10,808,540|
|**Salvo** |176,313|1.45ms|20.65ms|10,578,262|
|**Astra** |173,019|0.84ms|23.81ms|10,380,624|
|**Axum** |171,080|1.50ms|19.59ms|10,264,544|
|**Gotham** |158,513|1.61ms|21.51ms|9,510,492|
|**Rocket** |103,745|2.47ms|28.48ms|6,224,502|
|**Tide** |72,384|3.54ms|46.13ms|4,342,833|






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


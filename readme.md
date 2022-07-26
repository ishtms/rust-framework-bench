# Rust framework benchmarks

Benchmarking utility to test the performance of all the rust web frameworks. Built with [rust](https://rust-lang.org) 🚀.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)



### **(Last updated: Tue Jul 26 2022 20:12:25)** 

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
|   Concurrency: 16   |   Duration: 40 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |236,788|0.07ms|1.38ms|9,471,385|
|**May-MiniHttp** |229,811|0.07ms|1.60ms|9,192,297|
|**Hyper** |221,704|0.07ms|1.21ms|8,868,079|
|**Roa** |220,854|0.07ms|1.33ms|8,834,071|
|**Warp** |219,624|0.07ms|1.31ms|8,784,909|
|**Thruster** |217,016|0.07ms|1.43ms|8,680,495|
|**Salvo** |215,329|0.07ms|2.04ms|8,613,014|
|**Axum** |215,039|0.07ms|1.47ms|8,601,475|
|**Viz** |214,694|0.07ms|6.19ms|8,587,659|
|**Poem** |210,871|0.08ms|1.34ms|8,434,765|
|**Actix-Web** |208,980|0.08ms|8.00ms|8,359,129|
|**Gotham** |208,807|0.08ms|2.12ms|8,352,179|
|**Ntex** |194,006|0.08ms|29.13ms|7,760,178|
|**Astra** |191,170|0.07ms|2.11ms|7,646,642|
|**Nickel** |185,448|0.06ms|5.79ms|7,417,817|
|**Rocket** |169,526|0.09ms|1.17ms|6,780,990|
|**Tide** |134,257|0.12ms|2.60ms|5,370,193|


|   Concurrency: 64   |   Duration: 40 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |244,404|0.26ms|2.82ms|9,775,992|
|**May-MiniHttp** |230,965|0.28ms|3.09ms|9,238,480|
|**Hyper** |229,421|0.28ms|3.18ms|9,176,688|
|**Roa** |227,882|0.28ms|11.34ms|9,115,032|
|**Warp** |226,555|0.28ms|2.98ms|9,061,989|
|**Thruster** |225,786|0.28ms|3.02ms|9,031,346|
|**Actix-Web** |225,197|0.28ms|2.81ms|9,007,735|
|**Viz** |223,161|0.29ms|3.43ms|8,926,181|
|**Axum** |221,867|0.29ms|3.77ms|8,874,511|
|**Salvo** |221,208|0.29ms|2.81ms|8,848,189|
|**Poem** |218,170|0.29ms|2.62ms|8,726,660|
|**Gotham** |216,252|0.30ms|5.13ms|8,649,951|
|**Ntex** |215,866|0.30ms|121.99ms|8,634,509|
|**Nickel** |186,365|0.06ms|3.79ms|7,454,434|
|**Astra** |179,642|0.34ms|2.79ms|7,185,649|
|**Rocket** |145,271|0.44ms|9.65ms|5,810,736|
|**Tide** |97,707|0.65ms|34.47ms|3,908,179|


|   Concurrency: 128   |   Duration: 40 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,624|0.53ms|7.50ms|9,744,475|
|**May-MiniHttp** |240,071|0.53ms|7.21ms|9,602,524|
|**Actix-Web** |228,619|0.56ms|22.02ms|9,144,528|
|**Ntex** |222,743|0.57ms|125.62ms|8,909,407|
|**Thruster** |219,374|0.58ms|7.34ms|8,774,584|
|**Roa** |218,730|0.58ms|6.90ms|8,748,947|
|**Hyper** |217,473|0.59ms|8.03ms|8,698,570|
|**Warp** |216,697|0.59ms|7.30ms|8,667,651|
|**Poem** |213,860|0.60ms|6.75ms|8,554,212|
|**Viz** |213,314|0.60ms|6.75ms|8,532,419|
|**Salvo** |212,647|0.60ms|5.47ms|8,505,967|
|**Axum** |210,629|0.61ms|5.17ms|8,424,884|
|**Gotham** |205,751|0.62ms|6.40ms|8,229,725|
|**Nickel** |187,093|0.06ms|5.95ms|7,483,501|
|**Astra** |176,305|0.71ms|6.79ms|7,052,083|
|**Rocket** |116,574|1.10ms|15.85ms|4,662,885|
|**Tide** |74,053|1.73ms|10.63ms|2,962,013|


|   Concurrency: 256   |   Duration: 40 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,007|1.05ms|15.81ms|9,719,924|
|**May-MiniHttp** |239,507|1.07ms|18.14ms|9,579,933|
|**Actix-Web** |226,707|1.13ms|54.53ms|9,067,791|
|**Ntex** |223,489|1.14ms|126.65ms|8,939,164|
|**Hyper** |207,465|1.23ms|16.55ms|8,298,365|
|**Roa** |201,638|1.27ms|17.04ms|8,065,076|
|**Thruster** |200,735|1.27ms|17.27ms|8,028,994|
|**Warp** |199,327|1.28ms|19.26ms|7,972,458|
|**Viz** |188,647|1.36ms|18.67ms|7,545,480|
|**Nickel** |186,433|0.06ms|22.64ms|7,456,712|
|**Axum** |184,333|1.39ms|24.62ms|7,372,747|
|**Salvo** |184,084|1.39ms|19.93ms|7,362,866|
|**Poem** |180,975|1.41ms|17.96ms|7,238,748|
|**Gotham** |176,034|1.45ms|21.64ms|7,040,894|
|**Astra** |173,207|0.85ms|24.55ms|6,927,715|
|**Rocket** |102,761|2.49ms|16.17ms|4,110,398|
|**Tide** |72,378|3.54ms|22.35ms|2,894,901|






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

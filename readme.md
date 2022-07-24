
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
|**Tokio-minihttp** |236,677|0.07ms|1.43ms|5,916,739|
|**May-MiniHttp** |229,701|0.07ms|1.14ms|5,742,336|
|**Hyper** |223,304|0.07ms|1.82ms|5,582,470|
|**Roa** |222,150|0.07ms|3.18ms|5,553,561|
|**Warp** |220,654|0.07ms|1.75ms|5,516,183|
|**Thruster** |218,711|0.07ms|1.79ms|5,467,592|
|**Salvo** |216,081|0.07ms|1.91ms|5,401,860|
|**Viz** |215,437|0.07ms|1.70ms|5,385,760|
|**Axum** |214,830|0.07ms|1.65ms|5,370,603|
|**Poem** |210,574|0.08ms|1.48ms|5,264,158|
|**Actix-Web** |210,234|0.08ms|29.70ms|5,255,698|
|**Gotham** |208,313|0.08ms|1.46ms|5,207,655|
|**Ntex** |191,505|0.08ms|10.30ms|4,787,411|
|**Astra** |190,322|0.07ms|1.80ms|4,757,874|
|**Nickel** |187,262|0.06ms|2.35ms|4,681,364|
|**Rocket** |158,767|0.10ms|1.94ms|3,969,093|
|**Tide** |133,740|0.12ms|1.23ms|3,343,383|


|   Concurrency: 64   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |146,661|0.44ms|4.23ms|3,666,316|
|**Hyper** |143,577|0.45ms|3.87ms|3,589,194|
|**Roa** |142,921|0.45ms|2.93ms|3,572,855|
|**Warp** |141,665|0.45ms|3.99ms|3,541,427|
|**Thruster** |140,053|0.46ms|3.95ms|3,501,149|
|**Salvo** |135,838|0.47ms|4.06ms|3,395,746|
|**Poem** |135,545|0.47ms|4.06ms|3,388,426|
|**Gotham** |133,749|0.48ms|3.74ms|3,343,505|
|**May-MiniHttp** |133,431|0.48ms|4.17ms|3,335,576|
|**Actix-Web** |133,415|0.48ms|4.28ms|3,335,181|
|**Viz** |133,297|0.48ms|3.23ms|3,332,201|
|**Ntex** |132,491|0.48ms|4.20ms|3,312,052|
|**Axum** |131,685|0.49ms|3.90ms|3,291,946|
|**Rocket** |126,334|0.51ms|3.55ms|3,158,187|
|**Nickel** |125,269|0.10ms|2.39ms|3,131,493|
|**Astra** |122,807|0.51ms|2.65ms|3,070,096|
|**Tide** |98,323|0.65ms|5.97ms|2,457,941|


|   Concurrency: 256   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |142,480|1.80ms|18.75ms|3,561,219|
|**Hyper** |140,585|1.82ms|19.30ms|3,513,693|
|**Roa** |140,077|1.83ms|17.51ms|3,501,217|
|**Warp** |138,100|1.85ms|18.06ms|3,451,632|
|**Thruster** |135,694|1.89ms|28.97ms|3,391,579|
|**Salvo** |134,222|1.91ms|20.56ms|3,354,689|
|**Viz** |131,577|1.94ms|19.73ms|3,288,704|
|**May-MiniHttp** |130,915|1.95ms|18.30ms|3,271,984|
|**Gotham** |130,645|1.96ms|18.17ms|3,265,289|
|**Poem** |130,438|1.96ms|20.48ms|3,260,179|
|**Actix-Web** |129,654|1.97ms|19.49ms|3,240,610|
|**Ntex** |129,116|1.98ms|20.92ms|3,227,103|
|**Axum** |127,296|2.01ms|16.04ms|3,181,596|
|**Nickel** |124,859|0.10ms|19.41ms|3,120,806|
|**Rocket** |121,970|2.10ms|20.47ms|3,048,589|
|**Astra** |119,820|1.24ms|17.73ms|2,994,834|
|**Tide** |92,578|2.76ms|15.13ms|2,314,235|


|   Concurrency: 512   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |145,197|3.52ms|172.19ms|3,628,729|
|**Hyper** |141,336|3.62ms|181.99ms|3,532,059|
|**Roa** |141,307|3.62ms|237.15ms|3,531,638|
|**Warp** |140,183|3.65ms|211.74ms|3,503,454|
|**Thruster** |136,930|3.74ms|228.33ms|3,422,343|
|**Salvo** |134,873|3.79ms|231.39ms|3,370,770|
|**Viz** |134,603|3.80ms|185.97ms|3,363,983|
|**Axum** |134,455|3.80ms|187.16ms|3,360,539|
|**Poem** |132,601|3.86ms|188.77ms|3,314,076|
|**May-MiniHttp** |130,825|3.91ms|198.80ms|3,269,602|
|**Gotham** |130,673|3.92ms|190.82ms|3,265,701|
|**Actix-Web** |129,987|3.94ms|198.19ms|3,248,657|
|**Ntex** |128,481|3.98ms|203.94ms|3,210,951|
|**Nickel** |126,263|0.10ms|25.05ms|3,155,495|
|**Rocket** |121,974|4.19ms|209.44ms|3,048,386|
|**Astra** |120,083|1.22ms|105.54ms|3,001,032|
|**Tide** |91,943|5.57ms|230.79ms|2,297,819|






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


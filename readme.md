
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
|   Concurrency: 8   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**May-MiniHttp** |231,511|24.71us|602.00us|6,968,589|
|**Tokio-minihttp** |222,456|29.43us|0.98ms|6,695,928|
|**Actix-Web** |214,931|28.61us|522.00us|6,469,406|
|**Astra** |206,589|30.31us|0.88ms|6,218,374|
|**Ntex** |191,492|33.80us|502.00us|5,763,806|
|**Hyper** |189,460|32.73us|1.50ms|5,702,695|
|**Roa** |189,096|33.01us|773.00us|5,691,853|
|**Poem** |187,409|33.85us|1.13ms|5,622,364|
|**Thruster** |187,376|33.07us|5.09ms|5,640,016|
|**Viz** |186,416|33.69us|0.88ms|5,611,104|
|**Salvo** |186,242|33.79us|0.96ms|5,605,931|
|**Warp** |185,901|33.45us|676.00us|5,595,628|
|**Axum** |185,613|34.59us|6.42ms|5,586,958|
|**Gotham** |184,946|34.42us|8.31ms|5,566,922|
|**Nickel** |160,305|39.94us|1.10ms|4,825,196|
|**Rocket** |147,884|45.24us|1.08ms|4,451,410|
|**Tide** |121,065|58.65us|2.97ms|3,644,079|


|   Concurrency: 64   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |244,122|156.74us|3.98ms|7,348,770|
|**Roa** |237,515|153.30us|3.51ms|7,149,859|
|**Hyper** |236,735|153.55us|2.77ms|7,126,194|
|**Ntex** |233,294|153.61us|6.90ms|7,022,843|
|**Thruster** |233,166|155.61us|3.56ms|7,018,916|
|**Warp** |232,887|155.43us|3.68ms|7,010,395|
|**Actix-Web** |229,139|153.76us|3.54ms|6,897,640|
|**Viz** |228,668|159.35us|4.55ms|6,883,429|
|**Poem** |228,221|160.37us|3.91ms|6,869,994|
|**Salvo** |228,115|159.29us|2.50ms|6,866,675|
|**Axum** |228,032|159.37us|3.07ms|6,841,650|
|**Gotham** |223,838|163.32us|3.97ms|6,738,085|
|**Astra** |221,318|164.93us|2.94ms|6,662,295|
|**May-MiniHttp** |180,697|197.16us|4.15ms|5,439,424|
|**Nickel** |174,003|52.09us|3.51ms|5,238,303|
|**Rocket** |170,875|247.07us|4.13ms|5,143,834|
|**Tide** |124,966|380.49us|6.14ms|3,749,254|


|   Concurrency: 128   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |244,186|294.22us|5.49ms|7,351,129|
|**Roa** |235,307|292.75us|7.35ms|7,060,461|
|**Hyper** |234,704|292.63us|10.15ms|7,065,772|
|**Thruster** |233,507|293.67us|7.36ms|7,030,067|
|**Warp** |231,615|296.35us|7.50ms|6,949,705|
|**Ntex** |227,792|302.18us|7.51ms|6,857,744|
|**Axum** |227,254|302.57us|5.84ms|6,818,316|
|**Salvo** |226,883|303.70us|10.11ms|6,807,752|
|**Viz** |226,860|302.84us|4.82ms|6,807,008|
|**Poem** |224,325|313.25us|10.07ms|6,753,314|
|**Gotham** |221,666|311.71us|6.68ms|6,651,035|
|**Astra** |213,479|340.55us|7.38ms|6,426,875|
|**Actix-Web** |210,505|323.98us|7.13ms|6,316,536|
|**Nickel** |174,117|51.93us|5.77ms|5,241,851|
|**May-MiniHttp** |169,660|398.38us|6.51ms|5,107,719|
|**Rocket** |163,943|480.39us|11.62ms|4,919,233|
|**Tide** |63,435|1.93ms|26.29ms|1,903,550|


|   Concurrency: 256   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,986|566.96us|11.94ms|7,322,038|
|**Roa** |229,947|576.63us|12.06ms|6,901,989|
|**Thruster** |229,006|577.69us|12.50ms|6,873,255|
|**Salvo** |224,042|591.66us|11.39ms|6,724,858|
|**Viz** |222,916|593.68us|9.24ms|6,691,444|
|**Axum** |222,670|594.57us|11.84ms|6,683,053|
|**Poem** |222,369|597.68us|11.60ms|6,674,822|
|**Ntex** |220,351|602.05us|9.82ms|6,613,331|
|**Gotham** |217,261|621.17us|26.81ms|6,521,409|
|**Warp** |216,758|609.16us|11.12ms|6,506,182|
|**Hyper** |209,040|632.88us|12.28ms|6,275,249|
|**Astra** |208,530|396.99us|10.54ms|6,278,367|
|**Nickel** |171,577|53.03us|9.81ms|5,149,198|
|**Actix-Web** |162,850|808.40us|11.12ms|4,891,747|
|**Rocket** |153,742|0.95ms|11.85ms|4,618,163|
|**May-MiniHttp** |151,781|0.86ms|10.34ms|4,557,101|
|**Tide** |45,233|6.04ms|52.75ms|1,358,037|


|   Concurrency: 512   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Tokio-minihttp** |242,623|1.11ms|7.31ms|7,285,551|
|**Astra** |211,689|399.46us|9.51ms|6,355,410|
|**Roa** |196,899|1.32ms|10.21ms|5,925,744|
|**Thruster** |194,921|1.33ms|9.99ms|5,850,817|
|**Gotham** |193,597|1.34ms|12.82ms|5,826,793|
|**Salvo** |193,382|1.34ms|11.99ms|5,803,182|
|**Viz** |192,897|1.35ms|10.00ms|5,790,307|
|**Poem** |192,882|1.35ms|12.43ms|5,789,348|
|**Ntex** |173,146|1.50ms|9.72ms|5,197,594|
|**Hyper** |170,304|1.52ms|11.58ms|5,115,794|
|**Nickel** |169,761|53.54us|16.82ms|5,096,384|
|**Warp** |169,322|1.53ms|9.62ms|5,088,857|
|**Axum** |168,333|1.54ms|13.42ms|5,056,068|
|**Actix-Web** |142,629|1.82ms|8.63ms|4,287,200|
|**Rocket** |140,175|1.94ms|16.46ms|4,187,145|
|**May-MiniHttp** |137,115|1.88ms|11.17ms|4,126,706|
|**Tide** |44,120|12.60ms|98.51ms|1,325,060|






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


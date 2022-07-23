
# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)


## Frameworks included
**[Actix-Web](https://actix.rs)**
**[Hyper](https://hyper.rs)**
**[Axum](https://github.com/tokio-rs/axum)**
**[Ntex](https://github.com/ntex-rs/ntex)**
**[Rocket](https://rocket.rs)**
**[Tide](https://github.com/http-rs/tide)**
**[Viz](https://github.com/viz-rs/viz)**
**[Warp](https://github.com/seanmonstar/warp)**
**[Thruster](https://github.com/thruster-rs/Thruster)**
**[Salvo](https://github.com/salvo-rs/salvo)**
**[Poem](https://github.com/poem-web/poem)**
**[Astra](https://github.com/ibraheemdev/astra)**
# Results
|   Concurrency: 10   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Actix-Web**|221,736|33.65us|1.28ms|4,456,957|
|**Ntex**|214,402|38.24us|2.61ms|4,309,574|
|**Hyper**|206,014|36.03us|846.00us|4,140,979|
|**Axum**|202,769|37.06us|0.88ms|4,075,603|
|**Viz**|202,607|37.02us|0.98ms|4,072,421|
|**Thruster**|202,020|36.58us|1.24ms|4,040,503|
|**Warp**|200,752|39.75us|10.07ms|4,035,115|
|**Poem**|200,752|39.75us|10.07ms|4,035,115|
|**Astra**|200,752|39.75us|10.07ms|4,035,115|
|**Salvo**|195,276|2.75ms|228.88ms|3,908,804|
|**Rocket**|157,179|51.79us|0.99ms|3,159,312|
|**Tide**|132,724|66.33us|0.94ms|2,667,815|


|   Concurrency: 50   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|239,636|122.97us|3.03ms|4,793,199|
|**Ntex**|238,966|118.37us|3.18ms|4,803,828|
|**Actix-Web**|238,265|117.53us|3.14ms|4,789,505|
|**Thruster**|238,040|123.15us|2.85ms|4,785,022|
|**Warp**|236,274|122.81us|1.26ms|4,749,618|
|**Poem**|236,274|122.81us|1.26ms|4,749,618|
|**Astra**|236,274|122.81us|1.26ms|4,749,618|
|**Viz**|232,920|125.29us|2.83ms|4,682,091|
|**Axum**|232,405|126.09us|2.94ms|4,671,724|
|**Salvo**|231,646|126.35us|3.12ms|4,656,728|
|**Rocket**|183,447|190.94us|3.04ms|3,669,371|
|**Tide**|125,202|325.50us|10.90ms|2,516,868|


|   Concurrency: 100   |   Duration: 25 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|239,716|227.39us|4.36ms|5,993,663|
|**Warp**|236,051|230.70us|5.56ms|5,902,299|
|**Poem**|236,051|230.70us|5.56ms|5,902,299|
|**Astra**|236,051|230.70us|5.56ms|5,902,299|
|**Thruster**|235,311|231.07us|4.46ms|5,883,609|
|**Ntex**|234,239|230.64us|4.71ms|5,880,301|
|**Salvo**|231,484|235.93us|5.69ms|5,788,179|
|**Axum**|230,426|237.52us|4.73ms|5,784,672|
|**Viz**|230,382|238.13us|5.85ms|5,760,379|
|**Actix-Web**|211,582|255.46us|5.94ms|5,290,415|
|**Rocket**|171,840|375.91us|5.37ms|4,296,787|
|**Tide**|51,864|2.09ms|25.77ms|1,296,877|


|   Concurrency: 250   |   Duration: 35 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Warp**|231,270|549.53us|10.46ms|8,097,954|
|**Poem**|231,270|549.53us|10.46ms|8,097,954|
|**Astra**|231,270|549.53us|10.46ms|8,097,954|
|**Hyper**|229,291|556.10us|11.32ms|8,028,181|
|**Viz**|226,817|561.26us|11.64ms|7,940,626|
|**Axum**|226,595|561.81us|10.88ms|7,934,399|
|**Salvo**|224,891|567.86us|8.13ms|7,872,981|
|**Ntex**|212,607|499.26us|11.42ms|7,464,272|
|**Thruster**|188,248|21.75ms|1.07s|6,601,620|
|**Actix-Web**|178,648|594.26us|12.33ms|6,254,651|
|**Rocket**|152,892|0.94ms|10.66ms|5,356,590|
|**Tide**|41,902|6.72ms|62.56ms|1,467,116|


|   Concurrency: 500   |   Duration: 45 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|233,384|544.37us|10.12ms|10,506,177|
|**Warp**|231,365|548.23us|11.35ms|10,414,831|
|**Poem**|231,365|548.23us|11.35ms|10,414,831|
|**Astra**|231,365|548.23us|11.35ms|10,414,831|
|**Axum**|226,180|562.19us|11.05ms|10,180,997|
|**Salvo**|225,676|563.49us|10.46ms|10,158,000|
|**Viz**|225,447|565.57us|10.31ms|10,148,821|
|**Ntex**|213,931|495.19us|11.32ms|9,629,432|
|**Thruster**|204,331|6.55ms|688.82ms|9,203,770|
|**Actix-Web**|183,286|577.29us|12.30ms|8,250,370|
|**Rocket**|152,404|0.92ms|10.41ms|6,866,194|
|**Tide**|41,598|6.05ms|70.86ms|1,872,586|


|   Concurrency: 700   |   Duration: 60 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|235,606|539.12us|10.52ms|14,140,110|
|**Warp**|231,148|549.02us|11.13ms|13,872,631|
|**Poem**|231,148|549.02us|11.13ms|13,872,631|
|**Astra**|231,148|549.02us|11.13ms|13,872,631|
|**Axum**|227,333|559.29us|11.58ms|13,643,920|
|**Viz**|226,947|560.10us|10.95ms|13,643,020|
|**Salvo**|225,933|562.79us|11.62ms|13,559,195|
|**Ntex**|212,646|497.99us|11.66ms|12,761,300|
|**Thruster**|201,655|8.25ms|1.03s|12,112,435|
|**Actix-Web**|182,703|578.96us|11.12ms|10,964,955|
|**Rocket**|153,206|0.92ms|10.64ms|9,202,047|
|**Tide**|41,624|5.90ms|61.40ms|2,498,426|






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


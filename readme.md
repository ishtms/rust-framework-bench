
# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.


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
|**Actix-Web**|223,984|33.02us|792.00us|4,502,144|
|**Ntex**|213,833|38.22us|2.02ms|4,298,169|
|**Hyper**|205,934|35.98us|549.00us|4,139,237|
|**Axum**|203,411|36.97us|582.00us|4,088,586|
|**Viz**|203,242|36.95us|789.00us|4,085,230|
|**Thruster**|202,014|36.62us|1.09ms|4,060,560|
|**Warp**|200,626|37.01us|789.00us|4,032,646|
|**Poem**|200,626|37.01us|789.00us|4,032,646|
|**Astra**|200,626|37.01us|789.00us|4,032,646|
|**Salvo**|191,722|19.44ms|923.95ms|3,836,635|
|**Rocket**|157,147|54.05us|10.08ms|3,158,636|
|**Tide**|132,796|66.20us|7.58ms|2,669,156|


|   Concurrency: 50   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|239,985|122.46us|2.23ms|4,824,075|
|**Ntex**|239,318|118.55us|3.16ms|4,810,704|
|**Actix-Web**|238,138|117.29us|3.21ms|4,787,074|
|**Thruster**|237,431|123.59us|3.17ms|4,772,715|
|**Warp**|235,470|124.06us|3.21ms|4,733,370|
|**Poem**|235,470|124.06us|3.21ms|4,733,370|
|**Astra**|235,470|124.06us|3.21ms|4,733,370|
|**Salvo**|232,898|125.86us|2.78ms|4,658,452|
|**Viz**|232,746|125.73us|2.61ms|4,678,565|
|**Axum**|232,494|125.94us|3.10ms|4,650,422|
|**Rocket**|182,992|190.03us|10.04ms|3,678,579|
|**Tide**|125,334|331.84us|9.42ms|2,519,402|


|   Concurrency: 100   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|237,824|229.77us|4.58ms|4,757,102|
|**Ntex**|235,971|229.86us|5.86ms|4,744,114|
|**Thruster**|235,290|231.57us|5.78ms|4,706,843|
|**Warp**|234,111|232.86us|5.45ms|4,706,525|
|**Poem**|234,111|232.86us|5.45ms|4,706,525|
|**Astra**|234,111|232.86us|5.45ms|4,706,525|
|**Viz**|230,498|237.42us|5.83ms|4,610,799|
|**Salvo**|230,025|238.67us|4.30ms|4,601,588|
|**Axum**|229,481|240.10us|10.41ms|4,590,583|
|**Actix-Web**|209,875|257.14us|5.88ms|4,219,141|
|**Rocket**|174,688|368.19us|4.63ms|3,512,035|
|**Tide**|51,886|2.13ms|29.91ms|1,037,924|


|   Concurrency: 250   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|236,327|538.65us|9.51ms|4,729,012|
|**Warp**|231,524|549.10us|10.65ms|4,633,262|
|**Poem**|231,524|549.10us|10.65ms|4,633,262|
|**Astra**|231,524|549.10us|10.65ms|4,633,262|
|**Viz**|227,271|561.69us|10.23ms|4,548,454|
|**Axum**|224,577|569.14us|11.46ms|4,494,329|
|**Salvo**|224,146|569.29us|11.77ms|4,485,733|
|**Ntex**|211,291|503.15us|11.76ms|4,228,043|
|**Thruster**|200,268|1.87ms|189.01ms|4,015,727|
|**Actix-Web**|180,039|589.22us|11.88ms|3,602,637|
|**Rocket**|154,474|0.92ms|14.33ms|3,094,598|
|**Tide**|41,042|6.87ms|51.13ms|821,337|


|   Concurrency: 500   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|237,202|535.74us|9.90ms|4,746,550|
|**Warp**|230,853|550.11us|11.32ms|4,619,919|
|**Poem**|230,853|550.11us|11.32ms|4,619,919|
|**Astra**|230,853|550.11us|11.32ms|4,619,919|
|**Viz**|226,730|561.89us|12.12ms|4,538,656|
|**Salvo**|226,084|562.93us|11.48ms|4,525,259|
|**Axum**|224,229|569.02us|11.29ms|4,487,660|
|**Ntex**|210,356|503.31us|9.67ms|4,209,571|
|**Thruster**|202,852|10.07ms|1.18s|4,072,958|
|**Actix-Web**|184,041|576.25us|11.10ms|3,682,923|
|**Rocket**|154,901|0.91ms|11.22ms|3,102,138|
|**Tide**|40,943|6.14ms|59.53ms|819,459|


|   Concurrency: 700   |   Duration: 20 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|235,402|539.85us|10.68ms|4,710,985|
|**Thruster**|233,370|314.15us|46.58ms|4,669,143|
|**Warp**|231,776|547.67us|10.79ms|4,639,177|
|**Poem**|231,776|547.67us|10.79ms|4,639,177|
|**Astra**|231,776|547.67us|10.79ms|4,639,177|
|**Salvo**|228,014|558.14us|11.24ms|4,563,024|
|**Viz**|226,952|560.11us|11.85ms|4,541,715|
|**Axum**|226,509|561.72us|9.59ms|4,533,232|
|**Ntex**|215,565|492.27us|12.11ms|4,313,980|
|**Actix-Web**|176,114|601.50us|12.30ms|3,524,401|
|**Rocket**|154,113|0.91ms|18.16ms|3,085,802|
|**Tide**|40,942|6.00ms|57.84ms|819,558|






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


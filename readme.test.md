
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
|**Ntex**|225657.11|31.78us|771.00us|6792373|
|**Actix Web**|220938.71|33.28us|1.07ms|6650347|
|**Hyper**|201807.6|36.59us|553.00us|6074511|
|**Viz**|199807|37.34us|656.00us|6014204|
|**Axum**|199447.63|37.45us|0.89ms|6003370|
|**Warp**|198571.57|37.22us|608.00us|5977065|
|**Rocket**|155251.65|52.55us|3.63ms|4673127|
|**Tide**|131681.42|66.82us|2.74ms|3963568|


|   Concurrency: 50   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|235608.71|125.93us|3.01ms|7068694|
|**Actix Web**|232514.52|120.22us|3.15ms|6999021|
|**Warp**|230333.42|127.92us|2.38ms|6933171|
|**Viz**|227322.12|129.25us|2.11ms|6842680|
|**Axum**|226940.29|130.78us|3.48ms|6831248|
|**Ntex**|226678.13|124.26us|3.00ms|6823418|
|**Rocket**|183414.77|191.78us|3.23ms|5521090|
|**Tide**|152053.33|244.34us|4.83ms|4561843|


|   Concurrency: 100   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|233593.06|235.30us|7.35ms|7008928|
|**Warp**|229891.96|237.75us|5.83ms|6897704|
|**Axum**|227819.78|240.69us|5.78ms|6858138|
|**Viz**|226391.6|242.52us|5.49ms|6792682|
|**Actix Web**|216282.91|248.39us|5.69ms|6489443|
|**Ntex**|211784.22|254.52us|6.26ms|6354415|
|**Rocket**|175100.24|376.68us|5.45ms|5253953|
|**Tide**|92705|778.68us|13.54ms|2781548|


|   Concurrency: 500   |   Duration: 30 secs   |   Threads: 1   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|**Hyper**|231785.07|548.60us|8.67ms|6957108|
|**Warp**|227676.95|558.24us|11.41ms|6834319|
|**Axum**|225683.17|564.77us|12.02ms|6772883|
|**Viz**|224047.26|568.06us|8.88ms|6724933|
|**Ntex**|203266.77|515.19us|11.87ms|6100620|
|**Actix Web**|196609.53|537.13us|9.42ms|5901044|
|**Rocket**|159613.93|0.89ms|11.46ms|4794937|
|**Tide**|47075|3.89ms|45.72ms|1412991|





## Benchmarking tool
The benchmarks have been performed using [wrk](https://github.com/wg/wrk), locally. 

Check the raw output from wrk [here](https://github.com/Ishtmeet-Singh/rust-framework-benchmarks/tree/master/perf).
## Try it yourself
To run the code please follow the steps - 

1. Download the repository as a zip, or clone/fork it.
2. `cd rust-framework-benchmarks`
3. `cargo build --release`
4. Open multiple terminals and start each server (if you want to run all simultaneously). 
Eg, `./target/release/actix` on one, `./target/release/hyper` on another and so on.
5. Run batch tests - `sh ./start-test.sh`

All the output will be stored in `perf/*`

## Machine used
M1 Max MacBook Pro 2021 - 64GB ram, 10 CPU cores and 32 GPU cores

## Suggestions and changes
All the suggestions, code changes or additions of another web framework is appreciated. I'd like to keep the code as close as a real world scenario, instead of optimising it to the metal.



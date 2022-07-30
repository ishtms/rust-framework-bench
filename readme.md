# Rust framework benchmarks

Benchmarking utility to test the performance of all the rust web frameworks. Built with [rust](https://rust-lang.org) 🚀.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)



### **(Last updated: Thu Jul 28 2022 03:05:25)** 

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
|   Concurrency: 16   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |238,686|0.07ms|0.02ms|3.24ms|0.02ms|0.10ms|0.11ms|0.19ms|10,740,677|23.45MB/Sec|0|
| 2 |**May-MiniHttp** |231,432|0.07ms|0.01ms|3.38ms|0.02ms|0.11ms|0.14ms|0.28ms|10,414,246|22.51MB/Sec|0|
| 3 |**Roa** |223,753|0.07ms|0.01ms|1.86ms|0.02ms|0.12ms|0.14ms|0.21ms|10,068,762|18.78MB/Sec|0|
| 4 |**Hyper** |222,951|0.07ms|0.01ms|10.13ms|0.03ms|0.12ms|0.17ms|0.29ms|10,032,577|18.71MB/Sec|0|
| 5 |**Warp** |221,690|0.07ms|0.01ms|3.26ms|0.02ms|0.12ms|0.14ms|0.23ms|9,975,827|27.27MB/Sec|0|
| 6 |**Thruster** |220,126|0.07ms|0.01ms|3.15ms|0.02ms|0.12ms|0.14ms|0.21ms|9,905,501|21.41MB/Sec|0|
| 7 |**Viz** |218,662|0.07ms|0.01ms|3.08ms|0.02ms|0.12ms|0.14ms|0.22ms|9,839,639|27.11MB/Sec|0|
| 8 |**Axum** |218,294|0.07ms|0.01ms|2.97ms|0.02ms|0.12ms|0.15ms|0.23ms|9,823,027|26.86MB/Sec|0|
| 9 |**Salvo** |217,573|0.07ms|0.01ms|3.13ms|0.02ms|0.12ms|0.14ms|0.22ms|9,790,611|26.97MB/Sec|0|
| 10 |**Poem** |213,952|0.07ms|0.01ms|3.37ms|0.02ms|0.12ms|0.15ms|0.25ms|9,627,672|26.53MB/Sec|0|
| 11 |**Gotham** |211,476|0.08ms|0.01ms|3.21ms|0.02ms|0.13ms|0.15ms|0.24ms|9,516,154|33.68MB/Sec|0|
| 12 |**Actix-Web** |209,643|0.08ms|0.01ms|10.21ms|0.04ms|0.13ms|0.19ms|0.52ms|9,433,760|25.99MB/Sec|0|
| 13 |**Ntex** |193,848|0.08ms|0.01ms|19.77ms|0.05ms|0.15ms|0.22ms|0.71ms|8,722,967|23.85MB/Sec|0|
| 14 |**Astra** |191,461|0.08ms|0.02ms|3.07ms|0.02ms|0.14ms|0.17ms|0.26ms|8,615,552|19.54MB/Sec|0|
| 15 |**Nickel** |188,887|0.06ms|0.01ms|2.69ms|0.02ms|0.11ms|0.13ms|0.26ms|8,499,779|27.20MB/Sec|0|
| 16 |**Rocket** |171,687|0.09ms|0.02ms|3.23ms|0.03ms|0.17ms|0.22ms|0.33ms|7,725,782|40.44MB/Sec|0|
| 17 |**Tide** |135,418|0.12ms|0.02ms|2.93ms|0.03ms|0.20ms|0.23ms|0.32ms|6,093,680|16.66MB/Sec|0|


|   Concurrency: 32   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |245,812|0.13ms|0.02ms|1.92ms|0.03ms|0.18ms|0.23ms|0.39ms|11,061,288|24.15MB/Sec|0|
| 2 |**May-MiniHttp** |239,846|0.13ms|0.01ms|2.09ms|0.04ms|0.21ms|0.25ms|0.43ms|10,792,892|23.33MB/Sec|0|
| 3 |**Roa** |228,586|0.14ms|0.01ms|1.49ms|0.05ms|0.24ms|0.32ms|0.48ms|10,286,210|19.18MB/Sec|0|
| 4 |**Hyper** |227,372|0.14ms|0.01ms|2.35ms|0.05ms|0.25ms|0.35ms|0.53ms|10,231,525|19.08MB/Sec|0|
| 5 |**Warp** |227,114|0.14ms|0.01ms|1.70ms|0.05ms|0.25ms|0.34ms|0.52ms|10,219,896|27.94MB/Sec|0|
| 6 |**Thruster** |227,008|0.14ms|0.01ms|2.01ms|0.05ms|0.23ms|0.30ms|0.47ms|10,215,191|22.08MB/Sec|0|
| 7 |**Viz** |222,850|0.14ms|0.01ms|1.80ms|0.05ms|0.25ms|0.33ms|0.50ms|10,028,060|27.63MB/Sec|0|
| 8 |**Salvo** |222,694|0.14ms|0.01ms|2.03ms|0.05ms|0.25ms|0.33ms|0.51ms|10,021,081|27.61MB/Sec|0|
| 9 |**Actix-Web** |221,940|0.14ms|0.01ms|11.96ms|0.05ms|0.23ms|0.28ms|0.55ms|9,987,106|27.52MB/Sec|0|
| 10 |**Axum** |221,439|0.14ms|0.01ms|2.21ms|0.05ms|0.26ms|0.36ms|0.52ms|9,964,529|27.24MB/Sec|0|
| 11 |**Poem** |216,877|0.15ms|0.01ms|1.80ms|0.05ms|0.26ms|0.35ms|0.51ms|9,759,299|26.89MB/Sec|0|
| 12 |**Gotham** |214,737|0.15ms|0.01ms|1.73ms|0.05ms|0.27ms|0.36ms|0.52ms|9,663,030|34.20MB/Sec|0|
| 13 |**Ntex** |210,517|0.15ms|0.02ms|125.12ms|0.21ms|0.27ms|0.42ms|1.36ms|9,473,080|25.90MB/Sec|0|
| 14 |**Nickel** |187,710|0.06ms|0.01ms|2.41ms|0.02ms|0.11ms|0.13ms|0.23ms|8,446,785|27.03MB/Sec|0|
| 15 |**Astra** |185,867|0.16ms|0.02ms|3.48ms|0.05ms|0.29ms|0.36ms|0.49ms|8,363,857|18.97MB/Sec|0|
| 16 |**Rocket** |158,209|0.20ms|0.02ms|10.48ms|0.08ms|0.40ms|0.50ms|0.80ms|7,119,315|37.27MB/Sec|0|
| 17 |**Tide** |138,135|0.23ms|0.03ms|2.81ms|0.07ms|0.43ms|0.54ms|0.74ms|6,215,964|16.99MB/Sec|0|


|   Concurrency: 64   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |244,199|0.26ms|0.02ms|3.97ms|0.05ms|0.38ms|0.56ms|0.77ms|10,988,671|23.99MB/Sec|0|
| 2 |**Roa** |230,321|0.28ms|0.01ms|3.58ms|0.10ms|0.52ms|0.73ms|1.07ms|10,364,176|19.33MB/Sec|0|
| 3 |**Thruster** |229,430|0.28ms|0.01ms|3.92ms|0.09ms|0.49ms|0.68ms|1.03ms|10,324,033|22.32MB/Sec|0|
| 4 |**Hyper** |228,160|0.28ms|0.01ms|9.96ms|0.10ms|0.55ms|0.76ms|1.17ms|10,266,897|19.15MB/Sec|0|
| 5 |**Actix-Web** |227,411|0.28ms|0.01ms|13.02ms|0.09ms|0.45ms|0.59ms|0.96ms|10,233,239|28.19MB/Sec|0|
| 6 |**Warp** |226,976|0.28ms|0.01ms|3.21ms|0.10ms|0.54ms|0.75ms|1.06ms|10,213,624|27.92MB/Sec|0|
| 7 |**Viz** |222,757|0.29ms|0.01ms|4.05ms|0.10ms|0.55ms|0.76ms|1.09ms|10,023,818|27.62MB/Sec|0|
| 8 |**May-MiniHttp** |222,691|0.29ms|0.01ms|4.35ms|0.11ms|0.63ms|0.79ms|1.02ms|10,020,860|21.66MB/Sec|0|
| 9 |**Axum** |222,387|0.29ms|0.01ms|3.71ms|0.10ms|0.55ms|0.76ms|1.08ms|10,007,174|27.36MB/Sec|0|
| 10 |**Ntex** |220,077|0.29ms|0.01ms|125.05ms|0.24ms|0.47ms|0.65ms|1.68ms|9,903,245|27.07MB/Sec|0|
| 11 |**Salvo** |218,791|0.29ms|0.01ms|4.01ms|0.11ms|0.59ms|0.78ms|1.13ms|9,845,388|27.13MB/Sec|0|
| 12 |**Poem** |215,273|0.30ms|0.01ms|4.09ms|0.11ms|0.60ms|0.79ms|1.17ms|9,687,065|26.69MB/Sec|0|
| 13 |**Gotham** |212,980|0.30ms|0.01ms|4.02ms|0.11ms|0.59ms|0.78ms|1.06ms|9,583,904|33.92MB/Sec|0|
| 14 |**Nickel** |188,903|0.06ms|0.01ms|3.46ms|0.02ms|0.11ms|0.14ms|0.28ms|8,500,383|27.20MB/Sec|0|
| 15 |**Astra** |180,229|0.35ms|0.02ms|3.17ms|0.12ms|0.66ms|0.82ms|1.23ms|8,109,995|18.39MB/Sec|0|
| 16 |**Rocket** |142,331|0.45ms|0.02ms|5.26ms|0.18ms|0.88ms|1.08ms|1.63ms|6,404,755|33.53MB/Sec|0|
| 17 |**Tide** |102,274|0.63ms|0.04ms|7.41ms|0.22ms|1.09ms|1.28ms|1.73ms|4,602,238|12.58MB/Sec|0|


|   Concurrency: 128   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |242,157|0.53ms|0.02ms|8.05ms|0.09ms|0.79ms|1.14ms|1.53ms|10,896,581|23.79MB/Sec|0|
| 2 |**May-MiniHttp** |225,449|0.57ms|0.01ms|7.78ms|0.23ms|1.26ms|1.51ms|1.91ms|10,144,739|21.93MB/Sec|0|
| 3 |**Thruster** |224,328|0.57ms|0.01ms|8.02ms|0.23ms|1.23ms|1.59ms|2.27ms|10,094,310|21.82MB/Sec|0|
| 4 |**Hyper** |224,163|0.57ms|0.01ms|5.95ms|0.23ms|1.24ms|1.54ms|2.12ms|10,087,026|18.81MB/Sec|0|
| 5 |**Roa** |221,890|0.58ms|0.01ms|8.06ms|0.24ms|1.29ms|1.61ms|2.32ms|9,984,717|18.62MB/Sec|0|
| 6 |**Actix-Web** |219,696|0.58ms|0.01ms|21.16ms|0.21ms|1.17ms|1.48ms|1.97ms|9,885,928|27.24MB/Sec|0|
| 7 |**Ntex** |219,593|0.58ms|0.02ms|125.38ms|0.39ms|1.10ms|1.57ms|3.26ms|9,881,296|27.02MB/Sec|0|
| 8 |**Warp** |218,784|0.58ms|0.01ms|5.71ms|0.24ms|1.31ms|1.63ms|2.34ms|9,844,849|26.92MB/Sec|0|
| 9 |**Salvo** |216,043|0.59ms|0.01ms|7.76ms|0.24ms|1.31ms|1.63ms|2.34ms|9,721,495|26.78MB/Sec|0|
| 10 |**Viz** |215,102|0.59ms|0.01ms|6.36ms|0.24ms|1.32ms|1.62ms|2.34ms|9,679,231|26.67MB/Sec|0|
| 11 |**Axum** |213,566|0.60ms|0.01ms|8.14ms|0.25ms|1.34ms|1.66ms|2.46ms|9,610,174|26.27MB/Sec|0|
| 12 |**Poem** |207,554|0.62ms|0.01ms|8.16ms|0.25ms|1.36ms|1.67ms|2.42ms|9,339,574|25.73MB/Sec|0|
| 13 |**Gotham** |205,640|0.62ms|0.01ms|7.17ms|0.25ms|1.37ms|1.69ms|2.55ms|9,253,488|32.75MB/Sec|0|
| 14 |**Nickel** |188,774|0.06ms|0.01ms|6.65ms|0.02ms|0.11ms|0.13ms|0.23ms|8,494,478|27.18MB/Sec|0|
| 15 |**Astra** |176,775|0.71ms|0.02ms|5.11ms|0.24ms|1.43ms|2.01ms|2.82ms|7,954,516|18.04MB/Sec|0|
| 16 |**Rocket** |126,406|1.01ms|0.02ms|9.95ms|0.36ms|1.79ms|2.27ms|3.34ms|5,688,078|29.78MB/Sec|0|
| 17 |**Tide** |86,320|1.48ms|0.04ms|16.70ms|0.37ms|2.26ms|2.82ms|5.24ms|3,884,279|10.62MB/Sec|0|


|   Concurrency: 256   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |239,935|1.07ms|0.03ms|23.70ms|0.17ms|1.63ms|2.26ms|2.87ms|10,796,254|23.57MB/Sec|112|
| 2 |**May-MiniHttp** |225,891|1.13ms|0.01ms|20.20ms|0.46ms|2.48ms|2.98ms|3.66ms|10,164,348|21.97MB/Sec|116|
| 3 |**Hyper** |214,153|1.19ms|0.01ms|23.93ms|0.52ms|2.66ms|3.24ms|4.57ms|9,636,297|17.97MB/Sec|113|
| 4 |**Roa** |213,560|1.20ms|0.01ms|27.46ms|0.54ms|2.74ms|3.41ms|5.33ms|9,609,530|17.92MB/Sec|112|
| 5 |**Ntex** |212,350|1.21ms|0.02ms|126.74ms|0.76ms|2.61ms|3.53ms|8.38ms|9,555,230|26.12MB/Sec|115|
| 6 |**Actix-Web** |211,556|1.21ms|0.01ms|28.15ms|0.49ms|2.55ms|3.00ms|3.79ms|9,519,375|26.23MB/Sec|114|
| 7 |**Warp** |207,447|1.23ms|0.01ms|20.08ms|0.56ms|2.80ms|3.39ms|4.74ms|9,334,486|25.52MB/Sec|117|
| 8 |**Thruster** |206,336|1.24ms|0.01ms|24.78ms|0.57ms|2.89ms|3.68ms|5.34ms|9,284,528|20.07MB/Sec|112|
| 9 |**Viz** |195,104|1.31ms|0.01ms|25.17ms|0.63ms|3.07ms|4.00ms|5.94ms|8,779,221|24.19MB/Sec|114|
| 10 |**Salvo** |191,004|1.34ms|0.01ms|25.07ms|0.66ms|3.14ms|4.15ms|5.92ms|8,594,560|23.68MB/Sec|112|
| 11 |**Nickel** |187,718|0.06ms|0.01ms|22.42ms|0.06ms|0.11ms|0.16ms|0.46ms|8,446,617|27.03MB/Sec|229|
| 12 |**Poem** |186,233|1.37ms|0.02ms|20.81ms|0.65ms|3.15ms|4.15ms|5.99ms|8,380,052|23.09MB/Sec|114|
| 13 |**Axum** |184,895|1.38ms|0.01ms|17.54ms|0.69ms|3.25ms|4.38ms|6.05ms|8,319,745|22.75MB/Sec|118|
| 14 |**Gotham** |177,918|1.44ms|0.02ms|23.32ms|0.70ms|3.29ms|4.48ms|6.31ms|8,005,776|28.34MB/Sec|113|
| 15 |**Astra** |172,412|0.83ms|0.02ms|24.57ms|0.31ms|1.72ms|2.48ms|3.76ms|7,757,995|17.59MB/Sec|114|
| 16 |**Rocket** |120,433|2.12ms|0.02ms|24.72ms|0.74ms|3.70ms|4.84ms|6.59ms|5,419,109|28.37MB/Sec|116|
| 17 |**Tide** |85,056|3.01ms|0.03ms|41.62ms|0.83ms|4.73ms|6.58ms|16.39ms|3,827,291|10.46MB/Sec|113|


|   Concurrency: 512   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**Tokio-minihttp** |228,214|2.24ms|0.77ms|29.24ms|0.33ms|3.40ms|4.54ms|5.94ms|10,268,547|22.42MB/Sec|678|
| 2 |**May-MiniHttp** |222,317|2.30ms|0.02ms|108.87ms|3.02ms|12.29ms|20.25ms|31.41ms|10,003,223|21.62MB/Sec|688|
| 3 |**Ntex** |213,078|2.40ms|0.03ms|144.93ms|3.38ms|13.54ms|21.03ms|36.66ms|9,587,607|26.21MB/Sec|661|
| 4 |**Actix-Web** |210,507|2.43ms|0.03ms|124.57ms|3.28ms|13.32ms|21.09ms|33.20ms|9,471,972|26.10MB/Sec|693|
| 5 |**Hyper** |202,647|2.53ms|0.03ms|115.34ms|3.56ms|14.86ms|21.05ms|29.19ms|9,118,191|17.01MB/Sec|693|
| 6 |**Roa** |201,271|2.54ms|0.03ms|109.50ms|3.62ms|15.15ms|21.56ms|29.57ms|9,056,332|16.89MB/Sec|686|
| 7 |**Thruster** |200,880|2.55ms|0.03ms|109.41ms|3.52ms|14.63ms|20.43ms|28.06ms|9,038,670|19.54MB/Sec|670|
| 8 |**Warp** |192,638|2.66ms|0.02ms|116.19ms|3.75ms|15.71ms|22.32ms|30.40ms|8,667,958|23.70MB/Sec|706|
| 9 |**Viz** |180,575|2.83ms|0.03ms|115.68ms|4.09ms|17.28ms|24.69ms|33.41ms|8,125,285|22.39MB/Sec|694|
| 10 |**Axum** |179,257|2.85ms|0.03ms|112.98ms|4.15ms|17.52ms|24.85ms|33.51ms|8,065,741|22.05MB/Sec|702|
| 11 |**Salvo** |177,022|2.89ms|0.03ms|109.84ms|4.19ms|17.69ms|25.09ms|34.04ms|7,965,165|21.95MB/Sec|654|
| 12 |**Poem** |173,978|2.94ms|0.03ms|106.42ms|4.45ms|18.95ms|27.12ms|36.17ms|7,828,332|21.57MB/Sec|680|
| 13 |**Astra** |171,835|0.84ms|0.02ms|110.75ms|0.34ms|1.73ms|2.57ms|4.78ms|7,731,596|17.53MB/Sec|655|
| 14 |**Gotham** |162,488|3.15ms|0.03ms|109.89ms|4.72ms|20.08ms|28.24ms|38.08ms|7,311,263|25.88MB/Sec|679|
| 15 |**Rocket** |119,401|4.29ms|0.04ms|109.46ms|6.12ms|26.11ms|35.53ms|47.11ms|5,372,599|28.12MB/Sec|669|
| 16 |**Tide** |84,716|6.04ms|0.05ms|107.07ms|5.47ms|22.45ms|29.63ms|39.37ms|3,811,779|10.42MB/Sec|673|
| 17 |**Nickel** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 1024   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|Rank|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:--:|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
| 1 |**May-MiniHttp** |226,169|4.52ms|0.03ms|142.72ms|7.71ms|28.18ms|47.06ms|68.14ms|10,176,352|21.99MB/Sec|3498|
| 2 |**Tokio-minihttp** |217,174|4.71ms|1.62ms|125.73ms|0.95ms|7.14ms|9.84ms|21.77ms|9,771,058|21.33MB/Sec|3472|
| 3 |**Actix-Web** |203,924|5.02ms|0.03ms|128.59ms|8.98ms|33.77ms|52.90ms|74.43ms|9,175,367|25.27MB/Sec|3513|
| 4 |**Hyper** |200,049|5.11ms|0.03ms|127.52ms|10.10ms|37.97ms|47.98ms|62.35ms|9,001,201|16.78MB/Sec|3569|
| 5 |**Ntex** |199,485|5.13ms|0.03ms|124.40ms|9.72ms|36.99ms|58.44ms|80.03ms|8,975,806|24.53MB/Sec|3492|
| 6 |**Thruster** |195,080|5.25ms|0.03ms|125.90ms|10.30ms|38.63ms|48.14ms|61.97ms|8,777,493|18.97MB/Sec|3450|
| 7 |**Roa** |192,925|5.30ms|0.03ms|127.61ms|10.55ms|39.81ms|49.20ms|63.14ms|8,680,645|16.18MB/Sec|3584|
| 8 |**Warp** |192,306|5.32ms|0.03ms|118.88ms|10.46ms|39.22ms|49.16ms|62.84ms|8,652,570|23.65MB/Sec|3483|
| 9 |**Nickel** |186,305|0.07ms|0.01ms|47.41ms|0.40ms|0.26ms|0.89ms|7.75ms|8,381,899|26.82MB/Sec|4471|
| 10 |**Axum** |175,977|5.81ms|0.03ms|147.37ms|11.51ms|43.68ms|55.28ms|69.51ms|7,918,042|21.64MB/Sec|3500|
| 11 |**Salvo** |175,319|5.84ms|0.03ms|142.53ms|11.61ms|44.04ms|55.37ms|70.13ms|7,888,255|21.73MB/Sec|3527|
| 12 |**Viz** |172,988|5.92ms|0.03ms|122.06ms|11.87ms|45.51ms|58.48ms|76.98ms|7,783,641|21.44MB/Sec|3447|
| 13 |**Poem** |169,456|6.04ms|0.03ms|126.04ms|12.56ms|48.57ms|61.86ms|77.57ms|7,624,721|21.00MB/Sec|3479|
| 14 |**Astra** |164,477|0.91ms|0.02ms|117.93ms|0.71ms|2.21ms|3.96ms|15.90ms|7,399,718|16.78MB/Sec|3519|
| 15 |**Gotham** |160,986|6.36ms|0.03ms|141.66ms|13.20ms|50.83ms|64.31ms|82.12ms|7,243,408|25.63MB/Sec|3559|
| 16 |**Rocket** |119,394|8.57ms|0.04ms|126.31ms|17.43ms|66.22ms|81.75ms|99.22ms|5,372,281|28.11MB/Sec|3557|
| 17 |**Tide** |84,316|12.14ms|0.06ms|184.70ms|16.26ms|55.81ms|75.70ms|97.77ms|3,793,813|10.36MB/Sec|3477|






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

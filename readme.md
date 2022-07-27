# Rust framework benchmarks

Benchmarking utility to test the performance of all the rust web frameworks. Built with [rust](https://rust-lang.org) 🚀.

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)



### **(Last updated: Wed Jul 27 2022 02:43:13)** 

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

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Tokio-minihttp** |236,278|0.07ms|0.02ms|11.25ms|0.02ms|0.10ms|0.11ms|0.21ms|10,632,267|23.21MB/Sec|0|
|**May-MiniHttp** |229,824|0.07ms|0.01ms|10.90ms|0.03ms|0.11ms|0.13ms|0.24ms|10,341,825|22.36MB/Sec|0|
|**Hyper** |220,872|0.07ms|0.01ms|11.23ms|0.03ms|0.12ms|0.15ms|0.27ms|9,939,022|18.54MB/Sec|0|
|**Roa** |220,396|0.07ms|0.01ms|2.11ms|0.02ms|0.12ms|0.14ms|0.22ms|9,917,710|18.50MB/Sec|0|
|**Warp** |218,461|0.07ms|0.01ms|11.12ms|0.03ms|0.12ms|0.15ms|0.26ms|9,830,481|26.88MB/Sec|0|
|**Thruster** |217,339|0.07ms|0.01ms|9.46ms|0.03ms|0.12ms|0.14ms|0.23ms|9,780,006|21.14MB/Sec|0|
|**Viz** |215,551|0.07ms|0.01ms|9.70ms|0.03ms|0.12ms|0.15ms|0.24ms|9,699,565|26.72MB/Sec|0|
|**Axum** |215,377|0.07ms|0.01ms|11.45ms|0.03ms|0.12ms|0.14ms|0.24ms|9,691,752|26.50MB/Sec|0|
|**Salvo** |213,993|0.07ms|0.01ms|10.72ms|0.03ms|0.13ms|0.16ms|0.28ms|9,629,455|26.53MB/Sec|0|
|**Poem** |211,045|0.08ms|0.01ms|11.44ms|0.03ms|0.13ms|0.15ms|0.26ms|9,496,820|26.16MB/Sec|0|
|**Gotham** |207,737|0.08ms|0.01ms|11.17ms|0.03ms|0.13ms|0.15ms|0.24ms|9,348,004|33.08MB/Sec|0|
|**Actix-Web** |206,925|0.08ms|0.01ms|8.12ms|0.03ms|0.13ms|0.18ms|0.45ms|9,311,519|25.65MB/Sec|0|
|**Ntex** |194,075|0.08ms|0.01ms|82.60ms|0.07ms|0.15ms|0.21ms|0.64ms|8,733,211|23.88MB/Sec|0|
|**Astra** |189,262|0.08ms|0.02ms|11.21ms|0.03ms|0.13ms|0.16ms|0.28ms|8,516,614|19.31MB/Sec|0|
|**Nickel** |186,846|0.06ms|0.01ms|10.58ms|0.03ms|0.11ms|0.15ms|0.37ms|8,407,897|26.91MB/Sec|0|
|**Rocket** |168,829|0.09ms|0.02ms|10.44ms|0.04ms|0.18ms|0.22ms|0.35ms|7,597,137|39.77MB/Sec|0|
|**Tide** |134,145|0.12ms|0.03ms|10.23ms|0.04ms|0.20ms|0.23ms|0.33ms|6,036,340|16.50MB/Sec|0|


|   Concurrency: 64   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,656|0.26ms|0.03ms|4.10ms|0.04ms|0.34ms|0.43ms|0.78ms|10,964,229|23.93MB/Sec|0|
|**May-MiniHttp** |231,151|0.28ms|0.01ms|4.20ms|0.10ms|0.53ms|0.81ms|1.13ms|10,401,512|22.49MB/Sec|0|
|**Hyper** |229,047|0.28ms|0.01ms|3.49ms|0.10ms|0.51ms|0.79ms|1.30ms|10,306,893|19.22MB/Sec|0|
|**Roa** |228,564|0.28ms|0.01ms|3.84ms|0.10ms|0.50ms|0.75ms|1.26ms|10,285,159|19.18MB/Sec|0|
|**Thruster** |225,666|0.28ms|0.01ms|2.72ms|0.09ms|0.49ms|0.73ms|1.19ms|10,154,890|21.95MB/Sec|0|
|**Actix-Web** |224,786|0.28ms|0.01ms|19.25ms|0.09ms|0.43ms|0.51ms|0.91ms|10,115,212|27.87MB/Sec|0|
|**Warp** |223,905|0.29ms|0.01ms|2.88ms|0.10ms|0.54ms|0.84ms|1.30ms|10,075,642|27.55MB/Sec|0|
|**Axum** |223,282|0.29ms|0.01ms|3.45ms|0.10ms|0.53ms|0.80ms|1.30ms|10,047,440|27.47MB/Sec|0|
|**Salvo** |223,241|0.29ms|0.01ms|3.06ms|0.10ms|0.52ms|0.79ms|1.26ms|10,045,761|27.68MB/Sec|0|
|**Viz** |223,076|0.29ms|0.01ms|3.92ms|0.10ms|0.52ms|0.80ms|1.34ms|10,038,133|27.66MB/Sec|0|
|**Poem** |217,922|0.29ms|0.01ms|2.94ms|0.10ms|0.53ms|0.77ms|1.33ms|9,806,274|27.02MB/Sec|0|
|**Ntex** |216,818|0.29ms|0.02ms|89.10ms|0.16ms|0.47ms|0.62ms|1.32ms|9,756,674|26.67MB/Sec|0|
|**Gotham** |213,874|0.30ms|0.01ms|3.96ms|0.10ms|0.56ms|0.83ms|1.34ms|9,624,104|34.06MB/Sec|0|
|**Nickel** |185,882|0.06ms|0.01ms|3.47ms|0.02ms|0.11ms|0.13ms|0.24ms|8,364,450|26.77MB/Sec|0|
|**Astra** |180,278|0.34ms|0.02ms|3.75ms|0.11ms|0.63ms|0.75ms|1.20ms|8,112,342|18.40MB/Sec|0|
|**Rocket** |139,500|0.46ms|0.02ms|7.67ms|0.22ms|1.07ms|1.42ms|2.15ms|6,277,385|32.86MB/Sec|0|
|**Tide** |96,638|0.66ms|0.03ms|10.36ms|0.28ms|1.29ms|1.55ms|2.08ms|4,348,650|11.89MB/Sec|0|


|   Concurrency: 128   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Tokio-minihttp** |243,493|0.53ms|0.02ms|6.21ms|0.05ms|0.64ms|0.85ms|1.30ms|10,956,762|23.92MB/Sec|0|
|**May-MiniHttp** |238,791|0.54ms|0.01ms|8.60ms|0.19ms|0.99ms|1.52ms|2.05ms|10,745,278|23.23MB/Sec|0|
|**Actix-Web** |225,866|0.57ms|0.01ms|20.93ms|0.19ms|0.97ms|1.45ms|2.20ms|10,163,658|28.00MB/Sec|0|
|**Ntex** |223,863|0.57ms|0.02ms|125.49ms|0.38ms|0.93ms|1.33ms|3.17ms|10,073,437|27.54MB/Sec|0|
|**Hyper** |219,138|0.58ms|0.01ms|5.20ms|0.26ms|1.43ms|1.88ms|2.60ms|9,860,926|18.39MB/Sec|0|
|**Thruster** |218,480|0.59ms|0.01ms|6.19ms|0.26ms|1.40ms|1.90ms|2.81ms|9,831,348|21.25MB/Sec|0|
|**Roa** |217,154|0.59ms|0.01ms|7.31ms|0.27ms|1.48ms|1.97ms|2.97ms|9,771,547|18.22MB/Sec|0|
|**Warp** |215,043|0.59ms|0.01ms|6.67ms|0.27ms|1.49ms|1.95ms|2.73ms|9,676,599|26.46MB/Sec|0|
|**Poem** |213,979|0.60ms|0.01ms|7.36ms|0.24ms|1.32ms|1.91ms|3.11ms|9,628,912|26.53MB/Sec|0|
|**Viz** |213,200|0.60ms|0.01ms|8.10ms|0.27ms|1.48ms|2.03ms|3.18ms|9,593,758|26.43MB/Sec|0|
|**Salvo** |213,053|0.60ms|0.01ms|6.85ms|0.28ms|1.49ms|2.06ms|3.27ms|9,587,201|26.41MB/Sec|0|
|**Axum** |211,699|0.60ms|0.01ms|8.07ms|0.28ms|1.52ms|2.06ms|3.33ms|9,526,179|26.04MB/Sec|0|
|**Gotham** |202,429|0.63ms|0.02ms|5.56ms|0.29ms|1.57ms|2.11ms|3.46ms|9,108,995|32.24MB/Sec|0|
|**Nickel** |186,735|0.06ms|0.01ms|6.53ms|0.02ms|0.11ms|0.13ms|0.24ms|8,402,716|26.89MB/Sec|0|
|**Astra** |174,066|0.73ms|0.02ms|4.74ms|0.24ms|1.44ms|2.06ms|2.89ms|7,832,928|17.76MB/Sec|0|
|**Rocket** |115,439|1.11ms|0.02ms|15.33ms|0.51ms|2.34ms|3.27ms|4.71ms|5,194,656|27.19MB/Sec|0|
|**Tide** |75,340|1.70ms|0.04ms|8.23ms|0.46ms|2.63ms|3.07ms|4.06ms|3,390,276|9.27MB/Sec|0|


|   Concurrency: 256   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Tokio-minihttp** |241,506|1.06ms|0.02ms|23.20ms|0.11ms|1.30ms|1.72ms|2.54ms|10,867,025|23.72MB/Sec|115|
|**May-MiniHttp** |239,303|1.07ms|0.01ms|18.28ms|0.37ms|1.85ms|2.81ms|3.91ms|10,768,403|23.28MB/Sec|123|
|**Actix-Web** |227,462|1.12ms|0.01ms|67.11ms|0.40ms|1.90ms|2.69ms|4.13ms|10,235,128|28.20MB/Sec|117|
|**Ntex** |222,075|1.15ms|0.02ms|127.37ms|0.77ms|2.11ms|3.41ms|8.35ms|9,992,626|27.32MB/Sec|114|
|**Hyper** |204,997|1.25ms|0.01ms|20.05ms|0.63ms|3.19ms|4.01ms|5.57ms|9,224,376|17.20MB/Sec|116|
|**Roa** |202,504|1.26ms|0.01ms|16.21ms|0.67ms|3.38ms|4.45ms|6.28ms|9,112,020|16.99MB/Sec|118|
|**Thruster** |201,722|1.27ms|0.01ms|17.26ms|0.66ms|3.33ms|4.45ms|6.32ms|9,077,144|19.62MB/Sec|124|
|**Warp** |199,855|1.28ms|0.01ms|16.35ms|0.67ms|3.35ms|4.25ms|6.07ms|8,993,202|24.59MB/Sec|122|
|**Salvo** |187,026|1.37ms|0.01ms|15.28ms|0.81ms|3.90ms|5.64ms|7.69ms|8,415,840|23.19MB/Sec|124|
|**Poem** |186,749|1.37ms|0.01ms|17.82ms|0.79ms|3.85ms|5.78ms|8.79ms|8,403,423|23.15MB/Sec|122|
|**Viz** |186,177|1.37ms|0.02ms|24.57ms|0.81ms|3.89ms|5.58ms|7.77ms|8,377,424|23.08MB/Sec|116|
|**Nickel** |185,359|0.06ms|0.01ms|16.94ms|0.05ms|0.11ms|0.16ms|0.46ms|8,340,791|26.69MB/Sec|1|
|**Axum** |180,455|1.42ms|0.01ms|18.12ms|0.85ms|4.01ms|5.72ms|7.65ms|8,120,254|22.20MB/Sec|125|
|**Astra** |173,031|0.83ms|0.02ms|18.75ms|0.29ms|1.64ms|2.35ms|3.65ms|7,786,057|17.66MB/Sec|120|
|**Gotham** |164,362|1.56ms|0.02ms|17.98ms|0.96ms|4.38ms|6.64ms|8.75ms|7,396,088|26.18MB/Sec|124|
|**Rocket** |103,038|2.48ms|0.02ms|23.71ms|1.07ms|5.08ms|7.32ms|9.79ms|4,636,485|24.27MB/Sec|115|
|**Tide** |74,647|3.43ms|0.04ms|21.35ms|0.87ms|5.23ms|6.48ms|9.50ms|3,358,943|9.18MB/Sec|118|


|   Concurrency: 512   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |236,507|2.16ms|0.02ms|41.67ms|2.48ms|9.78ms|14.37ms|22.44ms|10,641,792|23.00MB/Sec|681|
|**Tokio-minihttp** |232,278|2.20ms|0.22ms|105.15ms|0.19ms|2.68ms|3.46ms|5.22ms|10,451,864|22.81MB/Sec|722|
|**Actix-Web** |222,901|2.30ms|0.03ms|126.32ms|2.90ms|11.61ms|16.66ms|24.71ms|10,029,652|27.63MB/Sec|711|
|**Ntex** |219,236|2.33ms|0.03ms|131.52ms|3.17ms|12.72ms|18.56ms|31.18ms|9,864,821|26.97MB/Sec|690|
|**Thruster** |192,314|2.66ms|0.03ms|109.95ms|3.72ms|15.46ms|21.98ms|30.64ms|8,653,891|18.71MB/Sec|728|
|**Hyper** |190,895|2.68ms|0.03ms|108.72ms|3.83ms|15.94ms|22.27ms|30.60ms|8,589,352|16.02MB/Sec|672|
|**Roa** |186,592|2.74ms|0.03ms|108.10ms|3.91ms|16.30ms|22.95ms|31.21ms|8,396,302|15.66MB/Sec|728|
|**Warp** |181,911|2.81ms|0.03ms|49.30ms|4.00ms|16.70ms|23.62ms|32.87ms|8,185,416|22.38MB/Sec|711|
|**Astra** |172,344|0.84ms|0.02ms|108.52ms|0.34ms|1.75ms|2.67ms|4.71ms|7,754,496|17.59MB/Sec|671|
|**Salvo** |171,490|2.98ms|0.03ms|111.13ms|4.34ms|18.23ms|25.65ms|34.64ms|7,716,834|21.26MB/Sec|707|
|**Viz** |169,502|3.02ms|0.03ms|117.22ms|4.42ms|18.55ms|26.32ms|35.58ms|7,627,179|21.01MB/Sec|721|
|**Poem** |165,732|3.09ms|0.03ms|124.35ms|4.68ms|19.70ms|27.86ms|37.61ms|7,457,409|20.55MB/Sec|702|
|**Axum** |163,194|3.14ms|0.03ms|111.37ms|4.57ms|19.20ms|27.06ms|36.63ms|7,343,460|20.07MB/Sec|721|
|**Gotham** |147,863|3.46ms|0.03ms|109.71ms|5.21ms|21.84ms|30.52ms|41.08ms|6,653,545|23.55MB/Sec|707|
|**Rocket** |101,158|5.06ms|0.03ms|111.69ms|7.04ms|29.51ms|39.74ms|51.28ms|4,551,915|23.82MB/Sec|721|
|**Tide** |73,429|6.97ms|0.09ms|109.63ms|6.65ms|26.88ms|35.64ms|47.30ms|3,304,081|9.03MB/Sec|714|
|**Nickel** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 1024   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |230,451|4.44ms|0.03ms|120.93ms|6.78ms|23.44ms|33.05ms|55.21ms|10,370,362|22.41MB/Sec|3510|
|**Tokio-minihttp** |219,507|4.66ms|1.76ms|121.80ms|0.79ms|6.26ms|8.38ms|22.13ms|9,875,814|21.55MB/Sec|3547|
|**Actix-Web** |217,225|4.71ms|0.03ms|121.41ms|7.62ms|26.42ms|35.73ms|57.53ms|9,775,030|26.92MB/Sec|3476|
|**Ntex** |209,934|4.88ms|0.03ms|142.97ms|8.63ms|31.09ms|43.97ms|73.42ms|9,447,073|25.82MB/Sec|3534|
|**Hyper** |188,089|5.44ms|0.03ms|134.24ms|10.77ms|40.44ms|50.39ms|66.22ms|8,463,917|15.78MB/Sec|3505|
|**Thruster** |186,745|5.48ms|0.03ms|121.62ms|10.70ms|40.22ms|50.14ms|61.19ms|8,403,663|18.16MB/Sec|3465|
|**Nickel** |184,459|0.08ms|0.02ms|126.67ms|0.61ms|0.37ms|1.45ms|13.33ms|8,299,442|26.54MB/Sec|7440|
|**Roa** |181,011|5.65ms|0.03ms|139.19ms|11.20ms|42.28ms|52.87ms|66.71ms|8,144,954|15.18MB/Sec|3556|
|**Warp** |176,802|5.79ms|0.03ms|122.32ms|11.19ms|41.73ms|52.91ms|69.81ms|7,956,030|21.74MB/Sec|3556|
|**Astra** |170,371|0.86ms|0.02ms|113.55ms|0.59ms|1.94ms|3.53ms|13.09ms|7,666,134|17.38MB/Sec|3490|
|**Viz** |165,540|6.18ms|0.03ms|173.09ms|12.19ms|46.05ms|57.76ms|68.75ms|7,449,165|20.51MB/Sec|3515|
|**Salvo** |161,642|6.33ms|0.03ms|123.64ms|12.46ms|47.11ms|59.20ms|74.98ms|7,273,962|20.03MB/Sec|3500|
|**Axum** |158,770|6.45ms|0.03ms|135.94ms|12.66ms|47.89ms|60.98ms|80.03ms|7,144,104|19.52MB/Sec|3521|
|**Poem** |151,198|6.77ms|0.03ms|137.96ms|13.82ms|52.85ms|66.22ms|84.06ms|6,802,936|18.74MB/Sec|3566|
|**Gotham** |145,017|7.06ms|0.03ms|121.27ms|14.42ms|54.96ms|68.52ms|83.00ms|6,526,103|23.08MB/Sec|3534|
|**Rocket** |99,598|10.28ms|0.05ms|172.64ms|20.24ms|75.41ms|94.02ms|122.59ms|4,481,850|23.44MB/Sec|3577|
|**Tide** |72,785|14.06ms|0.04ms|193.00ms|19.13ms|64.81ms|87.54ms|113.26ms|3,275,316|8.95MB/Sec|3518|


|   Concurrency: 2048   |   Duration: 45 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Tokio-minihttp** |209,885|9.75ms|3.80ms|478.75ms|6.03ms|16.92ms|35.43ms|149.64ms|9,443,464|20.58MB/Sec|18691|
|**May-MiniHttp** |197,089|10.38ms|0.03ms|487.23ms|21.12ms|74.53ms|110.21ms|241.47ms|8,868,256|19.12MB/Sec|23831|
|**Ntex** |175,725|11.64ms|0.03ms|696.45ms|22.47ms|75.31ms|114.72ms|246.00ms|7,906,865|21.56MB/Sec|19633|
|**Astra** |164,781|1.04ms|0.02ms|741.47ms|4.47ms|5.18ms|19.32ms|85.75ms|7,413,815|16.77MB/Sec|20165|
|**Actix-Web** |159,270|12.85ms|0.03ms|644.49ms|23.28ms|83.90ms|125.19ms|256.44ms|7,166,628|19.69MB/Sec|19422|
|**Thruster** |100,857|20.29ms|0.03ms|672.74ms|49.03ms|178.37ms|216.57ms|399.76ms|4,539,076|9.78MB/Sec|16504|
|**Hyper** |97,258|21.04ms|0.03ms|491.06ms|49.61ms|185.13ms|217.13ms|333.20ms|4,376,703|8.13MB/Sec|17546|
|**Roa** |96,809|21.14ms|0.03ms|692.47ms|48.48ms|184.92ms|223.20ms|392.67ms|4,355,959|8.09MB/Sec|18992|
|**Warp** |92,201|22.19ms|0.03ms|614.50ms|52.96ms|198.97ms|235.68ms|397.28ms|4,149,240|11.29MB/Sec|18101|
|**Salvo** |85,473|23.94ms|0.03ms|448.08ms|58.23ms|213.09ms|243.69ms|336.10ms|3,846,352|10.54MB/Sec|19001|
|**Viz** |84,979|24.08ms|0.03ms|769.64ms|59.59ms|214.27ms|261.12ms|516.41ms|3,823,637|10.48MB/Sec|18986|
|**Poem** |83,673|24.46ms|0.03ms|553.29ms|59.97ms|217.75ms|251.65ms|349.51ms|3,765,257|10.32MB/Sec|19011|
|**Axum** |82,492|24.81ms|0.04ms|663.70ms|59.96ms|215.18ms|252.25ms|407.63ms|3,711,724|10.09MB/Sec|21543|
|**Gotham** |82,224|24.88ms|0.04ms|787.57ms|61.93ms|221.81ms|265.97ms|504.31ms|3,700,358|13.03MB/Sec|18167|
|**Rocket** |75,606|27.06ms|0.07ms|687.93ms|65.54ms|232.99ms|270.97ms|444.10ms|3,402,200|17.71MB/Sec|18734|
|**Tide** |64,492|31.73ms|0.10ms|548.58ms|35.47ms|123.31ms|186.47ms|378.25ms|2,902,094|7.88MB/Sec|19228|
|**Nickel** |20,075|97.38ms|18.52ms|1280.93ms|152.32ms|731.50ms|1034.26ms|1113.35ms|903,123|37.45KB/Sec|892805|






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

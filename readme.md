# Rust framework benchmarks

Benchmarking utility to test the performance of any backend/server side frameworks. Built with [rust](https://rust-lang.org) 🚀. (currently only rust/golang based frameworks supported)

# Demo
![Demo](https://s4.gifyu.com/images/outputf55c6e3d5b6a1f8e.gif)



### **(Last updated: Fri Jul 14 2023 11:49:45)** 

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
|   Concurrency: 16   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |196,058|0.08ms|0.01ms|9.63ms|0.03ms|0.15ms|0.22ms|0.51ms|3,919,895|18.70MB/Sec|0|
|**Hyper** |185,987|0.09ms|0.01ms|5.88ms|0.04ms|0.18ms|0.27ms|0.73ms|3,718,786|15.61MB/Sec|0|
|**Warp** |185,944|0.09ms|0.01ms|8.17ms|0.04ms|0.18ms|0.28ms|0.73ms|3,718,101|22.88MB/Sec|0|
|**Salvo** |185,723|0.09ms|0.01ms|8.82ms|0.04ms|0.17ms|0.27ms|0.72ms|3,713,855|23.03MB/Sec|0|
|**Viz** |183,957|0.09ms|0.01ms|7.62ms|0.05ms|0.18ms|0.29ms|0.81ms|3,677,482|22.81MB/Sec|0|
|**Axum** |182,147|0.09ms|0.01ms|5.09ms|0.04ms|0.18ms|0.27ms|0.73ms|3,642,048|22.41MB/Sec|0|
|**Poem** |180,297|0.09ms|0.02ms|4.73ms|0.03ms|0.17ms|0.22ms|0.36ms|3,605,724|22.35MB/Sec|0|
|**Gotham** |179,680|0.09ms|0.02ms|14.68ms|0.05ms|0.18ms|0.28ms|0.75ms|3,591,288|28.62MB/Sec|0|
|**Actix-Web** |166,331|0.10ms|0.01ms|6.06ms|0.05ms|0.22ms|0.36ms|1.03ms|3,326,160|20.62MB/Sec|0|
|**Astra** |161,134|0.08ms|0.02ms|3.84ms|0.04ms|0.17ms|0.26ms|0.74ms|3,221,679|16.29MB/Sec|0|
|**Ntex** |159,619|0.10ms|0.02ms|41.72ms|0.08ms|0.21ms|0.33ms|1.06ms|3,191,402|19.64MB/Sec|0|
|**Rocket** |147,545|0.11ms|0.02ms|7.60ms|0.06ms|0.23ms|0.37ms|1.23ms|2,950,284|34.76MB/Sec|0|
|**Thruster** |818|4.86ms|0.16ms|6722.89ms|149.78ms|83.46ms|400.19ms|3679.56ms|16,373|0.00B/Sec|16389|
|**Tide** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Nickel** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Roa** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 32   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Roa** |192,720|0.17ms|0.01ms|4.88ms|0.07ms|0.34ms|0.51ms|1.19ms|3,853,159|16.17MB/Sec|0|
|**Salvo** |188,869|0.17ms|0.02ms|7.09ms|0.08ms|0.35ms|0.53ms|1.35ms|3,776,379|23.42MB/Sec|0|
|**Hyper** |188,731|0.17ms|0.01ms|8.02ms|0.08ms|0.36ms|0.53ms|1.36ms|3,772,489|15.84MB/Sec|0|
|**Warp** |188,557|0.17ms|0.01ms|7.73ms|0.08ms|0.36ms|0.52ms|1.22ms|3,770,117|23.20MB/Sec|0|
|**Viz** |187,664|0.17ms|0.01ms|6.86ms|0.08ms|0.35ms|0.51ms|1.22ms|3,752,403|23.27MB/Sec|0|
|**Axum** |184,814|0.17ms|0.01ms|9.98ms|0.08ms|0.36ms|0.52ms|1.23ms|3,695,085|22.74MB/Sec|0|
|**Poem** |184,060|0.17ms|0.02ms|5.24ms|0.07ms|0.34ms|0.43ms|0.66ms|3,680,634|22.82MB/Sec|0|
|**Gotham** |182,769|0.17ms|0.02ms|10.68ms|0.08ms|0.36ms|0.53ms|1.29ms|3,654,263|29.11MB/Sec|0|
|**May-MiniHttp** |182,092|0.18ms|0.01ms|11.56ms|0.09ms|0.40ms|0.59ms|1.56ms|3,641,328|17.37MB/Sec|0|
|**Actix-Web** |171,106|0.19ms|0.02ms|7.62ms|0.08ms|0.39ms|0.54ms|1.32ms|3,421,376|21.21MB/Sec|0|
|**Ntex** |167,185|0.19ms|0.02ms|57.92ms|0.13ms|0.40ms|0.58ms|1.74ms|3,342,806|20.57MB/Sec|0|
|**Astra** |163,832|0.19ms|0.02ms|9.38ms|0.08ms|0.38ms|0.55ms|1.41ms|3,275,694|16.56MB/Sec|0|
|**Nickel** |156,104|0.08ms|0.02ms|7.42ms|0.04ms|0.16ms|0.26ms|0.81ms|3,120,998|22.48MB/Sec|0|
|**Rocket** |144,510|0.22ms|0.02ms|10.35ms|0.11ms|0.45ms|0.65ms|1.87ms|2,889,513|34.04MB/Sec|0|
|**Tide** |99,647|0.32ms|0.03ms|6.30ms|0.12ms|0.60ms|0.84ms|1.89ms|1,992,672|12.26MB/Sec|0|
|**Thruster** |817|10.07ms|0.12ms|13529.24ms|290.42ms|193.34ms|956.58ms|8002.73ms|16,345|0.00B/Sec|16377|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 64   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Roa** |194,383|0.33ms|0.01ms|9.77ms|0.14ms|0.68ms|0.97ms|2.26ms|3,886,353|16.31MB/Sec|0|
|**Hyper** |190,918|0.33ms|0.01ms|10.13ms|0.14ms|0.71ms|0.96ms|2.07ms|3,817,057|16.02MB/Sec|0|
|**Salvo** |188,333|0.34ms|0.02ms|6.80ms|0.14ms|0.71ms|0.99ms|2.15ms|3,765,776|23.35MB/Sec|0|
|**Viz** |187,274|0.34ms|0.02ms|8.79ms|0.15ms|0.72ms|1.03ms|2.54ms|3,744,126|23.22MB/Sec|0|
|**Poem** |185,047|0.35ms|0.02ms|4.07ms|0.13ms|0.70ms|0.83ms|1.14ms|3,700,404|22.94MB/Sec|0|
|**Warp** |184,210|0.35ms|0.02ms|8.61ms|0.15ms|0.75ms|1.01ms|2.15ms|3,683,484|22.66MB/Sec|0|
|**Axum** |182,786|0.35ms|0.02ms|12.73ms|0.15ms|0.75ms|1.02ms|2.25ms|3,654,826|22.49MB/Sec|0|
|**May-MiniHttp** |181,688|0.35ms|0.01ms|7.68ms|0.15ms|0.75ms|0.97ms|1.93ms|3,631,909|17.33MB/Sec|0|
|**Gotham** |179,835|0.36ms|0.02ms|9.18ms|0.15ms|0.74ms|1.01ms|2.13ms|3,595,901|28.64MB/Sec|0|
|**Astra** |163,679|0.38ms|0.02ms|13.32ms|0.16ms|0.78ms|1.12ms|3.19ms|3,272,396|16.55MB/Sec|0|
|**Actix-Web** |161,267|0.40ms|0.02ms|74.22ms|0.23ms|0.84ms|1.22ms|3.56ms|3,224,367|19.99MB/Sec|0|
|**Ntex** |160,296|0.40ms|0.02ms|111.84ms|0.33ms|0.82ms|1.16ms|3.63ms|3,204,283|19.72MB/Sec|0|
|**Nickel** |156,044|0.08ms|0.02ms|9.25ms|0.04ms|0.16ms|0.25ms|0.76ms|3,119,802|22.47MB/Sec|0|
|**Rocket** |131,955|0.48ms|0.02ms|10.19ms|0.19ms|0.88ms|1.16ms|2.72ms|2,638,656|31.08MB/Sec|0|
|**Tide** |89,351|0.72ms|0.04ms|6.13ms|0.21ms|1.17ms|1.57ms|2.87ms|1,786,699|10.99MB/Sec|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Thruster** |0|3099.55ms|0.73ms|6714.79ms|3345.02ms|6714.79ms|6714.79ms|6714.79ms|26|0.00B/Sec|61|


|   Concurrency: 128   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |187,239|0.68ms|0.01ms|11.48ms|0.29ms|1.42ms|1.83ms|3.82ms|3,743,108|17.86MB/Sec|0|
|**Salvo** |186,513|0.69ms|0.02ms|10.70ms|0.28ms|1.42ms|1.84ms|3.97ms|3,728,894|23.12MB/Sec|0|
|**Viz** |186,289|0.69ms|0.02ms|12.22ms|0.29ms|1.44ms|1.89ms|4.16ms|3,723,062|23.10MB/Sec|0|
|**Hyper** |183,792|0.70ms|0.02ms|10.91ms|0.29ms|1.45ms|1.84ms|3.86ms|3,674,274|15.42MB/Sec|0|
|**Warp** |181,017|0.71ms|0.02ms|11.55ms|0.29ms|1.46ms|1.84ms|3.76ms|3,618,805|22.27MB/Sec|0|
|**Poem** |179,017|0.71ms|0.02ms|7.59ms|0.28ms|1.43ms|1.70ms|2.89ms|3,579,673|22.19MB/Sec|0|
|**Gotham** |176,835|0.72ms|0.02ms|16.15ms|0.30ms|1.48ms|1.94ms|4.29ms|3,534,689|28.16MB/Sec|0|
|**Axum** |175,712|0.73ms|0.02ms|9.16ms|0.30ms|1.49ms|1.89ms|3.89ms|3,512,451|21.62MB/Sec|0|
|**Roa** |173,465|0.74ms|0.02ms|10.77ms|0.33ms|1.55ms|2.05ms|4.61ms|3,467,218|14.56MB/Sec|0|
|**Actix-Web** |164,666|0.78ms|0.02ms|14.92ms|0.31ms|1.50ms|1.85ms|3.88ms|3,291,837|20.41MB/Sec|0|
|**Ntex** |158,018|0.81ms|0.02ms|26.16ms|0.34ms|1.55ms|1.99ms|4.88ms|3,159,623|19.44MB/Sec|0|
|**Nickel** |157,611|0.08ms|0.02ms|9.51ms|0.04ms|0.15ms|0.24ms|0.68ms|3,151,260|22.70MB/Sec|0|
|**Astra** |153,002|0.81ms|0.02ms|10.35ms|0.32ms|1.69ms|2.46ms|5.27ms|3,058,406|15.47MB/Sec|0|
|**Rocket** |127,738|1.00ms|0.02ms|13.10ms|0.33ms|1.69ms|2.24ms|4.66ms|2,553,577|30.09MB/Sec|0|
|**Tide** |89,692|1.43ms|0.05ms|15.75ms|0.39ms|2.24ms|3.10ms|7.14ms|1,793,237|11.03MB/Sec|0|
|**Thruster** |816|40.61ms|0.35ms|13118.36ms|547.16ms|712.63ms|3513.26ms|12315.34ms|16,335|0.00B/Sec|16463|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 256   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |186,610|1.37ms|0.01ms|20.77ms|0.56ms|2.79ms|3.50ms|6.89ms|3,729,479|17.80MB/Sec|52|
|**Roa** |184,422|1.39ms|0.01ms|18.41ms|0.57ms|2.79ms|3.43ms|6.52ms|3,685,447|15.48MB/Sec|7|
|**Viz** |173,950|1.47ms|0.02ms|18.73ms|0.62ms|2.94ms|3.74ms|7.97ms|3,476,747|21.57MB/Sec|8|
|**Salvo** |173,848|1.47ms|0.02ms|21.19ms|0.60ms|2.91ms|3.60ms|7.24ms|3,474,653|21.55MB/Sec|8|
|**Hyper** |171,299|1.49ms|0.01ms|19.19ms|0.62ms|2.92ms|3.55ms|6.78ms|3,424,658|14.38MB/Sec|79|
|**Warp** |171,048|1.50ms|0.02ms|19.74ms|0.62ms|2.94ms|3.57ms|6.95ms|3,418,322|21.04MB/Sec|66|
|**Axum** |167,980|1.52ms|0.02ms|18.24ms|0.63ms|2.99ms|3.66ms|7.03ms|3,357,211|20.67MB/Sec|47|
|**Actix-Web** |161,680|1.58ms|0.02ms|22.93ms|0.63ms|2.98ms|3.72ms|7.52ms|3,231,910|20.04MB/Sec|80|
|**Poem** |159,960|1.60ms|0.02ms|22.80ms|0.65ms|3.03ms|3.76ms|8.02ms|3,198,241|19.83MB/Sec|116|
|**Gotham** |158,082|1.62ms|0.02ms|18.27ms|0.64ms|3.06ms|3.91ms|7.90ms|3,160,314|25.18MB/Sec|52|
|**Astra** |154,864|0.92ms|0.02ms|23.10ms|0.35ms|1.87ms|2.61ms|5.21ms|3,095,003|15.66MB/Sec|110|
|**Nickel** |153,965|0.08ms|0.02ms|19.85ms|0.11ms|0.18ms|0.32ms|1.41ms|3,078,213|22.17MB/Sec|283|
|**Ntex** |150,558|1.70ms|0.02ms|114.13ms|0.76ms|3.08ms|3.92ms|9.53ms|3,010,390|18.52MB/Sec|115|
|**Rocket** |130,740|1.96ms|0.03ms|18.71ms|0.61ms|3.19ms|4.08ms|7.79ms|2,613,376|30.80MB/Sec|47|
|**Tide** |91,571|2.79ms|0.05ms|18.14ms|0.61ms|4.13ms|5.36ms|11.45ms|1,830,549|11.27MB/Sec|50|
|**Thruster** |813|95.68ms|0.12ms|13137.94ms|906.53ms|1789.49ms|8488.05ms|13113.14ms|16,259|0.00B/Sec|753|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|


|   Concurrency: 512   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |183,629|2.79ms|0.02ms|115.48ms|3.80ms|15.68ms|25.70ms|37.56ms|3,668,690|17.51MB/Sec|443|
|**Roa** |170,993|2.99ms|0.03ms|116.08ms|4.44ms|18.73ms|28.15ms|39.60ms|3,418,592|14.35MB/Sec|667|
|**Viz** |168,417|3.04ms|0.03ms|107.31ms|4.62ms|19.55ms|30.59ms|45.05ms|3,366,015|20.88MB/Sec|398|
|**Salvo** |166,343|3.08ms|0.03ms|111.42ms|4.71ms|19.85ms|31.43ms|49.29ms|3,324,851|20.62MB/Sec|395|
|**Warp** |164,739|3.10ms|0.03ms|108.67ms|4.62ms|19.54ms|29.39ms|43.28ms|3,291,918|20.26MB/Sec|522|
|**Hyper** |162,757|3.14ms|0.03ms|63.05ms|4.69ms|19.81ms|29.63ms|42.64ms|3,252,534|13.66MB/Sec|414|
|**Actix-Web** |158,601|3.23ms|0.03ms|122.56ms|4.94ms|21.25ms|34.07ms|47.45ms|3,170,232|19.66MB/Sec|520|
|**Axum** |157,622|3.24ms|0.03ms|120.70ms|4.92ms|21.00ms|32.04ms|45.96ms|3,150,744|19.39MB/Sec|696|
|**Gotham** |154,265|3.32ms|0.04ms|65.51ms|5.37ms|23.12ms|36.42ms|51.79ms|3,083,266|24.57MB/Sec|420|
|**Poem** |151,142|3.38ms|0.04ms|77.54ms|5.57ms|24.11ms|38.04ms|53.14ms|3,021,602|18.73MB/Sec|633|
|**Astra** |147,862|1.00ms|0.02ms|116.35ms|0.50ms|2.15ms|3.29ms|9.41ms|2,956,257|14.95MB/Sec|601|
|**Ntex** |147,421|3.47ms|0.04ms|111.74ms|5.67ms|24.91ms|38.37ms|53.43ms|2,946,083|18.13MB/Sec|440|
|**Nickel** |145,019|0.09ms|0.02ms|116.77ms|0.29ms|0.36ms|1.14ms|7.87ms|2,898,859|20.86MB/Sec|1|
|**Rocket** |131,030|3.90ms|0.06ms|81.84ms|6.87ms|30.82ms|45.93ms|63.45ms|2,619,054|30.86MB/Sec|433|
|**Tide** |92,786|5.51ms|0.09ms|110.18ms|3.55ms|16.59ms|21.66ms|29.32ms|1,854,294|11.41MB/Sec|477|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Thruster** |0|3051.28ms|0.26ms|6737.91ms|3340.39ms|6733.14ms|6736.49ms|6737.91ms|267|0.00B/Sec|548|


|   Concurrency: 1024   |   Duration: 20 secs   |   Threads: 2   |
|:-------------------:|:---------------------:|:--------------:|

|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**May-MiniHttp** |179,952|5.68ms|0.03ms|125.85ms|11.33ms|44.73ms|60.35ms|79.94ms|3,597,285|17.15MB/Sec|3297|
|**Roa** |170,804|5.99ms|0.04ms|131.05ms|12.50ms|47.73ms|60.27ms|77.52ms|3,412,299|14.32MB/Sec|3480|
|**Viz** |164,009|6.24ms|0.03ms|123.27ms|12.72ms|49.01ms|64.98ms|81.06ms|3,277,666|20.32MB/Sec|2775|
|**Salvo** |162,416|6.30ms|0.03ms|122.10ms|12.80ms|49.21ms|64.96ms|89.38ms|3,245,955|20.12MB/Sec|2980|
|**Warp** |161,833|6.32ms|0.03ms|148.36ms|12.90ms|49.17ms|62.99ms|80.68ms|3,234,786|19.89MB/Sec|3438|
|**Hyper** |160,575|6.37ms|0.03ms|128.50ms|13.04ms|50.05ms|65.11ms|78.24ms|3,209,247|13.46MB/Sec|2757|
|**Axum** |156,053|6.55ms|0.03ms|125.25ms|13.74ms|53.96ms|71.86ms|89.47ms|3,118,167|19.18MB/Sec|2547|
|**Actix-Web** |153,939|6.64ms|0.04ms|125.40ms|13.70ms|55.59ms|76.83ms|98.93ms|3,078,271|19.07MB/Sec|3173|
|**Poem** |153,259|6.67ms|0.03ms|125.03ms|14.46ms|57.65ms|76.16ms|94.48ms|3,063,010|18.98MB/Sec|3166|
|**Astra** |153,192|0.96ms|0.02ms|120.20ms|0.97ms|2.46ms|5.44ms|26.56ms|3,059,441|15.47MB/Sec|3191|
|**Ntex** |148,314|6.89ms|0.05ms|128.54ms|15.24ms|63.00ms|85.73ms|107.35ms|2,963,304|18.23MB/Sec|2949|
|**Nickel** |137,511|0.76ms|0.02ms|635.32ms|8.89ms|13.68ms|53.17ms|218.01ms|2,747,419|19.27MB/Sec|74310|
|**Rocket** |129,422|7.90ms|0.05ms|131.00ms|17.73ms|71.93ms|93.78ms|118.28ms|2,586,775|30.45MB/Sec|3398|
|**Tide** |91,921|11.13ms|0.12ms|124.16ms|11.82ms|43.35ms|57.28ms|71.16ms|1,837,182|11.29MB/Sec|2680|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Thruster** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Gotham** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|






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

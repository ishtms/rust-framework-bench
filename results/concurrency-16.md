|   **Name**   |   Req/sec   | Avg Latency | Min Latency | Max Latency | Std Dev | 95% | 99% | 99.9% |  # Requests | Transfer Rate |  # Errors |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:-----------:|:----:|:----:|:----:|:-----------:|:-----------:|
|**Warp** |186,015|0.09ms|0.01ms|10.21ms|0.04ms|0.17ms|0.26ms|0.61ms|3,719,629|22.88MB/Sec|0|
|**Axum** |181,315|0.09ms|0.01ms|2.85ms|0.04ms|0.18ms|0.26ms|0.49ms|3,625,922|22.31MB/Sec|0|
|**Poem** |179,327|0.09ms|0.02ms|8.99ms|0.05ms|0.18ms|0.28ms|0.82ms|3,585,318|22.23MB/Sec|0|
|**Hyper** |178,835|0.09ms|0.01ms|6.57ms|0.05ms|0.20ms|0.31ms|0.82ms|3,576,386|15.01MB/Sec|0|
|**Roa** |177,468|0.09ms|0.01ms|5.83ms|0.05ms|0.20ms|0.31ms|0.76ms|3,547,848|14.89MB/Sec|0|
|**Gotham** |176,294|0.09ms|0.02ms|7.41ms|0.05ms|0.19ms|0.29ms|0.85ms|3,524,817|28.08MB/Sec|0|
|**Actix-Web** |170,241|0.09ms|0.01ms|23.20ms|0.07ms|0.22ms|0.36ms|1.12ms|3,404,418|21.11MB/Sec|0|
|**Astra** |159,379|0.08ms|0.02ms|17.75ms|0.05ms|0.17ms|0.24ms|0.48ms|3,186,849|16.11MB/Sec|0|
|**Nickel** |154,231|0.08ms|0.02ms|7.70ms|0.04ms|0.16ms|0.24ms|0.69ms|3,084,006|22.21MB/Sec|0|
|**Salvo** |136,435|0.06ms|0.01ms|1478.46ms|2.19ms|0.22ms|0.55ms|3.79ms|2,727,582|16.91MB/Sec|0|
|**Tide** |97,537|0.16ms|0.03ms|10.38ms|0.09ms|0.37ms|0.61ms|1.53ms|1,950,396|12.00MB/Sec|0|
|**Thruster** |818|4.15ms|0.20ms|6710.95ms|128.50ms|59.79ms|269.78ms|2592.12ms|16,365|0.00B/Sec|16381|
|**Ntex** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Rocket** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**May-MiniHttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Viz** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|
|**Tokio-minihttp** (FAIL)|0|FAIL|FAIL|FAIL|FAIL|N/A|N/A|N/A|0|N/A|0|

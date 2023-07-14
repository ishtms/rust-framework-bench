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

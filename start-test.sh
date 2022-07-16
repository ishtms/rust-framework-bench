rm -rf ./perf; mkdir perf;
echo "WARP: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/warp.txt ; wrk -d20s -c10 -t1 "http://localhost:3000/" >> perf/warp.txt ;
echo "WARP: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/warp.txt ; wrk -d20s -c100 -t1 "http://localhost:3000/" >> perf/warp.txt ;
echo "WARP: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/warp.txt ; wrk -d30s -c1000 -t1 "http://localhost:3000/" >> perf/warp.txt ;
echo "HYPER: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/hyper.txt ; wrk -d20s -c10 -t1 "http://localhost:3001/" >> perf/hyper.txt ;
echo "HYPER: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/hyper.txt ; wrk -d20s -c100 -t1 "http://localhost:3001/" >> perf/hyper.txt ;
echo "HYPER: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/hyper.txt ; wrk -d30s -c1000 -t1 "http://localhost:3001/" >> perf/hyper.txt ;
echo "AXUM: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/axum.txt ; wrk -d20s -c10 -t1 "http://localhost:3002/" >> perf/axum.txt ;
echo "AXUM: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/axum.txt ; wrk -d20s -c100 -t1 "http://localhost:3002/" >> perf/axum.txt ;
echo "AXUM: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/axum.txt ; wrk -d30s -c1000 -t1 "http://localhost:3002/" >> perf/axum.txt ;
echo "NTEX: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/ntex.txt ; wrk -d20s -c10 -t1 "http://localhost:3003/" >> perf/ntex.txt ;
echo "NTEX: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/ntex.txt ; wrk -d20s -c100 -t1 "http://localhost:3003/" >> perf/ntex.txt ;
echo "NTEX: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/ntex.txt ; wrk -d30s -c1000 -t1 "http://localhost:3003/" >> perf/ntex.txt ;
echo "ACTIX: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/actix.txt ; wrk -d20s -c10 -t1 "http://localhost:3004/" >> perf/actix.txt ;
echo "ACTIX: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/actix.txt ; wrk -d20s -c100 -t1 "http://localhost:3004/" >> perf/actix.txt ;
echo "ACTIX: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/actix.txt ; wrk -d30s -c1000 -t1 "http://localhost:3004/" >> perf/actix.txt;
echo "TIDE: Concurrency [10]"; echo "Duration: 20s, Concurrency: 10, Threads: 1" >> perf/tide.txt ; wrk -d20s -c10 -t1 "http://localhost:3005/" >> perf/tide.txt ;
echo "TIDE: Concurrency [100]"; echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >> perf/tide.txt ; wrk -d20s -c100 -t1 "http://localhost:3005/" >> perf/tide.txt ;
echo "TIDE: Concurrency [1000]"; echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >> perf/tide.txt ; wrk -d30s -c1000 -t1 "http://localhost:3005/" >> perf/tide.txt




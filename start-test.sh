rm -rf ./perf
mkdir perf
echo "WARP: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/warp.txt
wrk -d20s -c10 -t1 "http://localhost:3000/" >>perf/warp.txt
sleep 3
echo "WARP: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/warp.txt
wrk -d20s -c50 -t1 "http://localhost:3000/" >>perf/warp.txt
sleep 3
echo "WARP: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/warp.txt
wrk -d20s -c100 -t1 "http://localhost:3000/" >>perf/warp.txt
sleep 3
echo "WARP: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/warp.txt
wrk -d30s -c1000 -t1 "http://localhost:3000/" >>perf/warp.txt
sleep 3
echo "HYPER: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/hyper.txt
wrk -d20s -c10 -t1 "http://localhost:3001/" >>perf/hyper.txt
sleep 3
echo "HYPER: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/hyper.txt
wrk -d20s -c50 -t1 "http://localhost:3001/" >>perf/hyper.txt
sleep 3
echo "HYPER: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/hyper.txt
wrk -d20s -c100 -t1 "http://localhost:3001/" >>perf/hyper.txt
sleep 3
echo "HYPER: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/hyper.txt
wrk -d30s -c1000 -t1 "http://localhost:3001/" >>perf/hyper.txt
sleep 3
echo "AXUM: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/axum.txt
wrk -d20s -c10 -t1 "http://localhost:3002/" >>perf/axum.txt
sleep 3
echo "AXUM: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/axum.txt
wrk -d20s -c50 -t1 "http://localhost:3002/" >>perf/axum.txt
sleep 3
echo "AXUM: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/axum.txt
wrk -d20s -c100 -t1 "http://localhost:3002/" >>perf/axum.txt
sleep 3
echo "AXUM: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/axum.txt
wrk -d30s -c1000 -t1 "http://localhost:3002/" >>perf/axum.txt
sleep 3
echo "NTEX: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/ntex.txt
wrk -d20s -c10 -t1 "http://localhost:3003/" >>perf/ntex.txt
sleep 3
echo "NTEX: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/ntex.txt
wrk -d20s -c50 -t1 "http://localhost:3003/" >>perf/ntex.txt
sleep 3
echo "NTEX: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/ntex.txt
wrk -d20s -c100 -t1 "http://localhost:3003/" >>perf/ntex.txt
sleep 3
echo "NTEX: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/ntex.txt
wrk -d30s -c1000 -t1 "http://localhost:3003/" >>perf/ntex.txt
sleep 3
echo "ACTIX: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/actix.txt
wrk -d20s -c10 -t1 "http://localhost:3004/" >>perf/actix.txt
sleep 3
echo "ACTIX: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/actix.txt
wrk -d20s -c50 -t1 "http://localhost:3004/" >>perf/actix.txt
sleep 3
echo "ACTIX: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/actix.txt
wrk -d20s -c100 -t1 "http://localhost:3004/" >>perf/actix.txt
sleep 3
echo "ACTIX: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/actix.txt
wrk -d30s -c1000 -t1 "http://localhost:3004/" >>perf/actix.txt
sleep 3
echo "TIDE: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/tide.txt
wrk -d20s -c10 -t1 "http://localhost:3005/" >>perf/tide.txt
sleep 3
echo "TIDE: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/tide.txt
wrk -d20s -c50 -t1 "http://localhost:3005/" >>perf/tide.txt
sleep 3
echo "TIDE: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/tide.txt
wrk -d20s -c100 -t1 "http://localhost:3005/" >>perf/tide.txt
sleep 3
echo "TIDE: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/tide.txt
wrk -d30s -c1000 -t1 "http://localhost:3005/" >>perf/tide.txt
sleep 3
echo "ROCKET: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/rocket.txt
wrk -d20s -c10 -t1 "http://localhost:3006/" >>perf/rocket.txt
sleep 3
echo "ROCKET: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/rocket.txt
wrk -d20s -c50 -t1 "http://localhost:3006/" >>perf/rocket.txt
sleep 3
echo "ROCKET: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/rocket.txt
wrk -d20s -c100 -t1 "http://localhost:3006/" >>perf/rocket.txt
sleep 3
echo "ROCKET: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/rocket.txt
wrk -d30s -c1000 -t1 "http://localhost:3006/" >>perf/rocket.txt
sleep 3
echo "VIZ: Concurrency [10]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/viz.txt
wrk -d20s -c10 -t1 "http://localhost:3007/" >>perf/viz.txt
sleep 3
echo "VIZ: Concurrency [50]"
echo "Duration: 20s, Concurrency: 10, Threads: 1" >>perf/viz.txt
wrk -d20s -c50 -t1 "http://localhost:3007/" >>perf/viz.txt
sleep 3
echo "VIZ: Concurrency [100]"
echo "\n\nDuration: 20s, Concurrency: 100, Threads: 1" >>perf/viz.txt
wrk -d20s -c100 -t1 "http://localhost:3007/" >>perf/viz.txt
sleep 3
echo "VIZ: Concurrency [1000]"
echo "\n\nDuration: 30s, Concurrency: 1000, Threads: 1" >>perf/viz.txt
wrk -d30s -c1000 -t1 "http://localhost:3007/" >>perf/viz.txt
sleep 3

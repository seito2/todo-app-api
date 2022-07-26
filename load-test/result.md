# Test result

## Create result
```bash
k6 run ./create_task.js --vus 200 --duration 60s
```

```
running (1m30.0s), 000/200 VUs, 5749 complete and 25 interrupted iterations
default ✓ [======================================] 200 VUs  1m0s

     data_received..................: 1.0 MB 11 kB/s
     data_sent......................: 947 kB 11 kB/s
     http_req_blocked...............: avg=2.91ms min=1µs      med=2µs   max=365.02ms p(90)=906.2µs p(95)=3.03ms
     http_req_connecting............: avg=2.82ms min=0s       med=0s    max=364.94ms p(90)=854µs   p(95)=2.88ms
     http_req_duration..............: avg=2.12s  min=97µs     med=1.85s max=36.96s   p(90)=2.61s   p(95)=3.11s
       { expected_response:true }...: avg=2.05s  min=51.88ms  med=1.89s max=36.96s   p(90)=2.58s   p(95)=3.04s
     http_req_failed................: 15.86% ✓ 912       ✗ 4837
     http_req_receiving.............: avg=39.9µs min=0s       med=42µs  max=330µs    p(90)=61µs    p(95)=72.59µs
     http_req_sending...............: avg=23.4µs min=7µs      med=16µs  max=186µs    p(90)=49µs    p(95)=60µs
     http_req_tls_handshaking.......: avg=0s     min=0s       med=0s    max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=2.12s  min=12µs     med=1.85s max=36.96s   p(90)=2.61s   p(95)=3.11s
     http_reqs......................: 5749   63.876014/s
     iteration_duration.............: avg=2.13s  min=456.11µs med=1.85s max=36.96s   p(90)=2.61s   p(95)=3.12s
     iterations.....................: 5749   63.876014/s
     vus............................: 25     min=25      max=200
     vus_max........................: 200    min=200     max=200
```

## Get result

```bash
k6 run ./get_task.js --vus 100 --duration 60s
```

```
running (1m01.8s), 000/100 VUs, 3376 complete and 0 interrupted iterations
default ✓ [======================================] 100 VUs  1m0s

     data_received..................: 496 MB 8.0 MB/s
     data_sent......................: 313 kB 5.1 kB/s
     http_req_blocked...............: avg=160.65µs min=1µs     med=2µs   max=5.7ms  p(90)=5µs    p(95)=303µs
     http_req_connecting............: avg=105.99µs min=0s      med=0s    max=3.88ms p(90)=0s     p(95)=0s
     http_req_duration..............: avg=1.8s     min=2.96ms  med=1.6s  max=26.62s p(90)=2.09s  p(95)=2.53s
       { expected_response:true }...: avg=1.71s    min=84.17ms med=1.6s  max=20.08s p(90)=2.09s  p(95)=2.28s
     http_req_failed................: 1.51%  ✓ 51        ✗ 3325
     http_req_receiving.............: avg=121.73µs min=0s      med=110µs max=949µs  p(90)=168µs  p(95)=196µs
     http_req_sending...............: avg=71.14ms  min=7µs     med=12µs  max=2.35s  p(90)=32.5µs p(95)=66.25µs
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s    max=0s     p(90)=0s     p(95)=0s
     http_req_waiting...............: avg=1.73s    min=2.92ms  med=1.58s max=26.62s p(90)=2.07s  p(95)=2.21s
     http_reqs......................: 3376   54.662274/s
     iteration_duration.............: avg=1.8s     min=8.45ms  med=1.6s  max=26.62s p(90)=2.09s  p(95)=2.53s
     iterations.....................: 3376   54.662274/s
     vus............................: 45     min=45      max=100
     vus_max........................: 100    min=100     max=100
```
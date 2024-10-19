# Loro Benchmark on Real-World Data from Latch.bio

This repository contains benchmarks demonstrating the performance improvements
in importing docs for Loro version 1.0.

The following benchmarks were conducted using a real-world editing history from
latch.bio, comprising 1,659,541 operations. All benchmarks results below were
performed on a MacBook Pro M1 2020.

| name                             | task                            | time                    |
| -------------------------------- | ------------------------------- | ----------------------- |
| 0 Old Snapshot Format on 0.16.12 | Import                          | 17.339081ms +- 29.8µs   |
|                                  | Import+GetAllValues             | 17.395361ms +- 43.714µs |
|                                  | Import+GetAllValues+Edit        | 17.463965ms +- 26.334µs |
|                                  | Import+GetAllValues+Edit+Expo   | 32.388292ms +- 56.006µs |
| 1 Old Snapshot Format on 1.0.0   | Import                          | 16.882856ms +- 34.296µs |
|                                  | Import+GetAllValues             | 16.820565ms +- 30.189µs |
|                                  | Import+GetAllValues+Edit        | 16.934977ms +- 36.064µs |
|                                  | Import+GetAllValues+Edit+Export | 31.161731ms +- 80.663µs |
| 2 New Snapshot Format            | Import                          | 1.150184ms +- 10.06µs   |
|                                  | Import+GetAllValues             | 1.192459ms +- 12.164µs  |
|                                  | Import+GetAllValues+Edit        | 1.208262ms +- 12.034µs  |
|                                  | Import+GetAllValues+Edit+Export | 5.46355ms +- 77.198µs   |
| 3 Shallow Snapshot Format        | Import                          | 374.98µs +- 8.472µs     |
|                                  | Import+GetAllValues             | 375.227µs +- 1.596µs    |
|                                  | Import+GetAllValues+Edit        | 374.859µs +- 1.398µs    |
|                                  | Import+GetAllValues+Edit+Export | 843.65µs +- 5.116µs     |

```log

============================
    Old Snapshot Format on 0.16.12
============================
Snapshot Size: 1.11 MB
Parse Time: 39.223166ms
Memory Usage 10.45 MB
Parse + get_deep_value() time: 39.427541ms
Parse + get_deep_value() + PushNewCell time: 39.676416ms
Parse + get_deep_value() + PushNewCell + Export time: 62.471458ms
Memory Usage After All Operations 10.49 MB



============================
    Old Snapshot Format on 1.0.0-beta.1
============================
Snapshot Size: 1.11 MB
Parse Time: 20.279625ms
Memory Usage 6.65 MB
Ops Size 1659541
Parse + get_deep_value() time: 20.335791ms
Parse + get_deep_value() + PushNewCell time: 20.424416ms
Parse + get_deep_value() + PushNewCell + Export time: 35.387958ms
Memory Usage After All Operations 6.75 MB



============================
    New Snapshot Format 1.0.0-beta.1
============================
Snapshot Size: 813.09 KB
Parse Time: 1.211667ms
Memory Usage 1.11 MB
Ops Size 1659541
Parse + get_deep_value() time: 1.231417ms
Parse + get_deep_value() + PushNewCell time: 1.257708ms
Parse + get_deep_value() + PushNewCell + Export time: 3.589167ms
Memory Usage After All Operations 1.56 MB



============================
    Shallow Snapshot Format 1.0.0-beta.1
============================
Snapshot Size: 28.21 KB
Parse Time: 192.458µs
Memory Usage 300.99 KB
Ops Size 1659541
Parse + get_deep_value() time: 206.833µs
Parse + get_deep_value() + PushNewCell time: 218.917µs
Parse + get_deep_value() + PushNewCell + Export time: 813.167µs
Memory Usage After All Operations 461.16 KB
```

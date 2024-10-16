# Loro Benchmark on Real-World Data from Latch.bio

This repository contains benchmarks demonstrating the performance improvements
in importing docs for Loro version 1.0.

The following benchmarks were conducted using a real-world editing history from
latch.bio, comprising 1,659,541 operations. All benchmarks results below were
performed on a MacBook Pro M1 2020.

| name                             | task                            | time                     |
| -------------------------------- | ------------------------------- | ------------------------ |
| 0 Old Snapshot Format on 0.16.12 | Import                          | 15.688953ms +- 10.263µs  |
|                                  | Import+GetAllValues             | 15.781199ms +- 12.296µs  |
|                                  | Import+GetAllValues+Edit        | 15.916294ms +- 113.443µs |
|                                  | Import+GetAllValues+Edit+Expo   | 30.2621ms +- 32.936µs    |
| 1 Old Snapshot Format            | Import                          | 16.400821ms +- 109.773µs |
|                                  | Import+GetAllValues             | 16.338033ms +- 19.918µs  |
|                                  | Import+GetAllValues+Edit        | 16.38042ms +- 20.873µs   |
|                                  | Import+GetAllValues+Edit+Export | 30.096402ms +- 168.673µs |
| 2 New Snapshot Format            | Import                          | 906.097µs +- 23.672µs    |
|                                  | Import+GetAllValues             | 880.976µs +- 25.544µs    |
|                                  | Import+GetAllValues+Edit        | 773.657µs +- 23.585µs    |
|                                  | Import+GetAllValues+Edit+Export | 4.102005ms +- 92.102µs   |
| 3 Shallow Snapshot Format        | Import                          | 255.449µs +- 9.661µs     |
|                                  | Import+GetAllValues             | 185.39µs +- 10.48µs      |
|                                  | Import+GetAllValues+Edit        | 225.466µs +- 10.787µs    |
|                                  | Import+GetAllValues+Edit+Export | 581.245µs +- 17.223µs    |

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

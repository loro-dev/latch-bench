# Loro Benchmark on Real-World Data from Latch.bio

The benchmarks below were conducted using a real-world editing history from latch.bio, encompassing 1,659,541 operations.

These benchmarks were executed on a MacBook Pro M1 2020.

| name                             | task                            | time                     |
| -------------------------------- | ------------------------------- | ------------------------ |
| 0 Old Snapshot Format on 0.16.12 | Import                          | 15.996928ms +- 58.435µs  |
|                                  | Import+GetAllValues             | 16.118824ms +- 153.909µs |
|                                  | Import+GetAllValues+Edit        | 16.015109ms +- 23.045µs  |
|                                  | Import+GetAllValues+Edit+Expo   | 30.757287ms +- 153.625µs |
| 1 Old Snapshot Format on 1.0.0   | Import                          | 16.324012ms +- 21.872µs  |
|                                  | Import+GetAllValues             | 16.397019ms +- 26.234µs  |
|                                  | Import+GetAllValues+Edit        | 16.587012ms +- 107.193µs |
|                                  | Import+GetAllValues+Edit+Export | 30.865065ms +- 370.096µs |
| 2 New Snapshot Format            | Import                          | 921.635µs +- 18.894µs    |
|                                  | Import+GetAllValues             | 923.792µs +- 22.066µs    |
|                                  | Import+GetAllValues+Edit        | 801.133µs +- 30.773µs    |
|                                  | Import+GetAllValues+Edit+Export | 3.796512ms +- 116.328µs  |
| 3 Shallow Snapshot Format        | Import                          | 99.541µs +- 847ns        |
|                                  | Import+GetAllValues             | 121.836µs +- 1.663µs     |
|                                  | Import+GetAllValues+Edit        | 137.912µs +- 1.486µs     |
|                                  | Import+GetAllValues+Edit+Export | 463.643µs +- 1.798µs     |

```log
============================
    Old Snapshot Format on 0.16.12
============================
Snapshot Size: 1.11 MB
Parse Time: 39.952125ms
Memory Usage 10.45 MB
Parse + get_deep_value() time: 40.046291ms
Parse + get_deep_value() + PushNewCell time: 40.22025ms
Parse + get_deep_value() + PushNewCell + Export time: 62.9975ms
Memory Usage After All Operations 10.49 MB



============================
    Old Snapshot Format on 1.0.0-alpha.5
============================
Snapshot Size: 1.11 MB
Parse Time: 20.226625ms
Memory Usage 6.65 MB
Ops Size 1659541
Parse + get_deep_value() time: 20.321791ms
Parse + get_deep_value() + PushNewCell time: 20.386208ms
Parse + get_deep_value() + PushNewCell + Export time: 35.727958ms
Memory Usage After All Operations 6.75 MB



============================
    New Snapshot Format on 1.0.0-alpha.5
============================
Snapshot Size: 813.09 KB
Parse Time: 1.303709ms
Memory Usage 1.11 MB
Ops Size 1659541
Parse + get_deep_value() time: 1.339417ms
Parse + get_deep_value() + PushNewCell time: 1.369167ms
Parse + get_deep_value() + PushNewCell + Export time: 3.772959ms
Memory Usage After All Operations 1.56 MB



============================
    Shallow Snapshot Format on 1.0.0-alpha.5
============================
Snapshot Size: 28.21 KB
Parse Time: 213.292µs
Memory Usage 301.63 KB
Ops Size 1659541
Parse + get_deep_value() time: 228.209µs
Parse + get_deep_value() + PushNewCell time: 239.792µs
Parse + get_deep_value() + PushNewCell + Export time: 792µs
Memory Usage After All Operations 461.69 KB
```

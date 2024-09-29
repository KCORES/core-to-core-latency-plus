CPU core-to-core latency tools
------------------------------


### Desc
![License](https://img.shields.io/badge/license-MIT-green.svg)


该程序用来计算 CPU 核心间通信延迟.  

具体方式为, 将两个线程固定在两个不同的 CPU 核心上, 通过CPU缓存一致性协议, 让线程执行一系列比较交换操作来测量延迟.

该 fork 旨在提升程序的易用性并收集更多 CPU benckmark 数据来进行展示. 



### How to run

如何运行:

```
$ cargo install core-to-core-latency
$ core-to-core-latency
```

### Single socket results 

- 按照延迟中间值进行排序, 越小越好
- 如果一个 CPU 同时存在 P 核和 E 核, 只计算 P 核.

CPU                                                                            | Median Latency | Min Latency | Max Latency 
-------------------------------------------------------------------------------| ------------------------------------------
AMD Ryzen 9 7950X, 16 Cores, zen4, 2022-Q3                                     | 67.7ns         | 10.3ns      | 73.0ns
AMD EPYC 7773X, 64 Cores, Milan-X, 2022-Q1                                     | 115.3ns        | 21.3ns      | 139.6ns
Intel Xeon Gold 6242, 16 Cores, Cascade Lake, 2019-Q2                          | 47.6ns         | 44.6ns      | 58.6ns
Intel Xeon Phi 7210, 64 Cores, Knights Landing, 2016-Q2                        | 91.0ns         | 41.0ns      | 112.5ns
HiSilicon Kunpeng 920-6426, 64 cores, ARMv8.2-A, 2019-Q1                       | 71.8ns         | 30.0ns      | 87.7ns
Intel Core i9-12900K, 8P+8E Cores, Alder Lake, 12th gen, 2021-Q4               | 38.5ns         | 27.8ns      | 50.7ns
Intel Core i9-9900K, 3.60GHz, 8 Cores, Coffee Lake, 9th gen, 2018-Q4           | 21.0ns         | 17.7ns      | 24.2ns
Intel Core i7-1165G7, 2.80GHz, 4 Cores, Tiger Lake, 11th gen, 2020-Q3          | 27.4ns         | 26.0ns      | 29.2ns
Intel Core i7-6700K, 4.00GHz, 4 Cores, Skylake, 6th gen, 2015-Q3               | 20.0ns         | 18.9ns      | 20.9ns
Intel Core i5-10310U, 4 Cores, Comet Lake, 10th gen, 2020-Q2                   | 20.7ns         | 20.0ns      | 21.4ns
Intel Core i5-4590, 3.30GHz 4 Cores, Haswell, 4th gen, 2014-Q2                 | 21.2ns         | 20.5ns      | 21.6ns
Apple M1 Pro, 6P+2E Cores, 2021-Q4                                             | 40.0ns         | 38.3ns      | 40.9ns
Intel Xeon Platinum 8375C, 2.90GHz, 32 Cores, Ice Lake, 3rd gen, 2021-Q2       | 51.3ns         | 39.7ns      | 62.0ns
Intel Xeon Platinum 8275CL, 3.00GHz, 24 Cores, Cascade Lake, 2nd gen, 2019-Q2  | 46.8ns         | 41.5ns      | 51.3ns
Intel Xeon E5-2695 v4, 2.10GHz, 18 Cores, Broadwell, 5th gen, 2016-Q1          | 43.8ns         | 33.9ns      | 50.1ns
AMD EPYC 7R13, 48 Cores, Milan, 3rd gen, 2021-Q1                               | 106.6ns        | 20.1ns      | 115.9ns 
AMD Ryzen Threadripper 3960X, 3.80GHz, 24 Cores, Zen 2, 3rd Gen, 2019-Q4       | 94.6ns         | 20.6ns      | 102.8ns 
AMD Ryzen Threadripper 1950X, 3.40GHz, 16 Cores, Zen, 1st Gen, 2017-Q3         | 153.7ns        | 23.2ns      | 159.1ns
AMD Ryzen 9 5950X, 3.40GHz, 16 Cores, Zen3, 4th gen, 2020-Q4                   | 17ns, 85ns
AMD Ryzen 9 5900X, 3.40GHz, 12 Cores, Zen3, 4th gen, 2020-Q4                   | 16ns, 84ns
AMD Ryzen 7 5800U, 1.9GHz up to 4.4GHz, 8 Cores, Zen3, 4th gen, 2021-Q4        | 19ns
AMD Ryzen 7 5700X, 3.40GHz, 8 Cores, Zen3, 4th gen, 2022-Q2                    | 18ns
AMD Ryzen 7 2700X, 3.70GHz, 8 Cores, Zen+, 2nd gen, 2018-Q3                    | 24ns, 92ns
AMD Ryzen 9 5900HX, 3.3GHz, 8 Cores, Zen3, 4th gen, 2021-Q1                    | 17ns          | 8ns, , 18ns
AWS Graviton3, 64 Cores, Arm Neoverse, 3rd gen, 2021-Q4                        | 46ns          |
AWS Graviton2, 64 Cores, Arm Neoverse, 2rd gen, 2020-Q1                        | 47ns          |
Sun/Oracle SPARC T4, 2.85GHz, 8 cores, 2011-Q3                                 | 98ns          |
IBM Power7, 3.3GHz, 8 Cores, 2010-Q1                                           | 173ns         |
IBM PowerPC 970, 1.8GHz, 2 Cores, 2003-Q2                                      | 576ns         |

### Dual sockets results
---------------------

The following shows dual-socket configuration latency where one CPU on the first socket sends a message to
another CPU on the second socket.
The number in parenthesis next to the latency denotes the slowdown compared to single socket.

CPU                                                                            | Median Latency
-------------------------------------------------------------------------------| ------------------
Intel Xeon Gold 6242, 16 Cores, Cascade Lake, 2019-Q2                          | 136ns (2.8x)
Intel Xeon Platinum 8375C, 2.90GHz, 32 Cores, Ice Lake, 3rd gen, 2021-Q2       | 108ns (2.1x)
Intel Xeon Platinum 8275CL, 3.00GHz, 24 Cores, Cascade Lake, 2nd gen, 2019-Q2  | 134ns (2.8x)
Intel Xeon E5-2695 v4, 2.10GHz, 18 Cores, Broadwell, 5th gen, 2016-Q1          | 118ns (2.7x)
AMD EPYC 7R13, 48 Cores, Milan, 3rd gen, 2021-Q1                               | 197ns
Sun/Oracle SPARC T4, 2.85GHz, 8 cores, 2011-Q3                                 | 356ns (3.6x)
IBM Power7, 3.3GHz, 8 Cores, 2010-Q1                                           | 443ns (2.5x)

### Hyper-threads
-------------

We measure the latency between two hyper-threads of the same core

CPU                                                                            | Median Latency
-------------------------------------------------------------------------------| ------------------
AMD Ryzen 9 7950X, 16 Cores, zne4, 2022-Q3                                     | 5.3ns
AMD EPYC 7773X, 64 Cores, Milan-X, 2022-Q1                                     | 10ns
Intel Xeon Gold 6242, 16 Cores, Cascade Lake, 2019-Q2                          | 7.4ns
Intel Core i9-12900K, 8+8 Cores, Alder Lake, 12th gen, 2021-Q4                 | 4.3ns
Intel Core i9-9900K, 3.60GHz, 8 Cores, Coffee Lake, 9th gen, 2018-Q4           | 6.2ns
Intel Core i7-1165G7, 2.80GHz, 4 Cores, Tiger Lake, 11th gen, 2020-Q3          | 5.9ns
Intel Core i7-6700K, 4.00GHz, 4 Cores, Skylake, 6th gen, 2015-Q3               | 6.9ns
Intel Core i5-10310U, 4 Cores, Comet Lake, 10th gen, 2020-Q2                   | 7.3ns
Intel Xeon Platinum 8375C, 2.90GHz, 32 Cores, Ice Lake, 3rd gen, 2021-Q2       | 8.1ns
Intel Xeon Platinum 8275CL, 3.00GHz, 24 Cores, Cascade Lake, 2nd gen, 2019-Q2  | 7.6ns
Intel Xeon E5-2695 v4, 2.10GHz, 18 Cores, Broadwell, 5th gen, 2016-Q1          | 7.6ns
AMD EPYC 7R13, 48 Cores, Milan, 3rd gen, 2021-Q1                               | 9.8ns
AMD Ryzen Threadripper 3960X, 3.80GHz, 24 Cores, Zen 2, 3rd Gen, 2019-Q4       | 6.5ns
AMD Ryzen Threadripper 1950X, 3.40GHz, 16 Cores, Zen, 1st Gen, 2017-Q3         | 10ns
AMD Ryzen 9 5950X, 3.40GHz, 16 Cores, Zen3, 4th gen, 2020-Q4                   | 7.8ns
AMD Ryzen 9 5900X, 3.40GHz, 12 Cores, Zen3, 4th gen, 2020-Q4                   | 7.6ns
AMD Ryzen 7 5700X, 3.40GHz, 8 Cores, Zen3, 4th gen, 2022-Q2                    | 7.8ns
AMD Ryzen 7 2700X, 3.70GHz, 8 Cores, Zen+, 2nd gen, 2018-Q3                    | 9.7ns
Sun/Oracle SPARC T4, 2.85GHz, 8 cores, 2011-Q3                                 | 24ns
IBM Power7, 3.3GHz, 8 Cores, 2010-Q1                                           | 70ns

---

**The notebook [results/results.ipynb](results/results.ipynb) contains the code to generate these graphs**

How to use
----------

First [install Rust](https://www.rust-lang.org/tools/install) and `gcc` on linux, then:

```
$ cargo install core-to-core-latency
$ core-to-core-latency
Num cores: 10
Using RDTSC to measure time: false
Num round trips per samples: 1000
Num samples: 300
Showing latency=round-trip-time/2 in nanoseconds:

       0       1       2       3       4       5       6       7       8       9
  0
  1   52±6
  2   38±6    39±4
  3   39±5    39±6    38±6
  4   34±6    38±4    37±6    36±5
  5   38±5    38±6    38±6    38±6    37±6
  6   38±5    37±6    39±6    36±4    49±6    38±6
  7   36±6    39±5    39±6    37±6    35±6    36±6    38±6
  8   37±5    38±6    35±5    39±5    38±6    38±5    37±6    37±6
  9   48±6    39±6    36±6    39±6    38±6    36±6    41±6    38±6    39±6

Min  latency: 34.5ns ±6.1 cores: (4,0)
Max  latency: 52.1ns ±9.4 cores: (1,0)
Mean latency: 38.4ns
```

Contribute
-----------

Use `core-to-core-latency 5000 --csv > output.csv` to instruct the program to use
5000 iterations per sample to reduce the noise, and save the results.

It can be used in the jupter notebook [results/results.ipynb](results/results.ipynb) for rendering graphs.

Create a GitHub issue with the generated `output.csv` file and I'll add your results.

### License

MIT

## Benchmarks

### Prime generation benchmark

Prime generation times with the generate_prime() function from bitsizes 128, through to 4096 bits:

```
generate_prime/128 bits time:   [2.1422 ms 2.1770 ms 2.2121 ms]

generate_prime/256 bits time:   [5.2033 ms 5.3443 ms 5.4904 ms]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  
generate_prime/512 bits time:   [20.344 ms 21.680 ms 23.078 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking generate_prime/1024 bits: Warming up for 3.0000 s
generate_prime/1024 bits
                        time:   [129.32 ms 147.76 ms 167.73 ms]
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe

Benchmarking generate_prime/2048 bits: Warming up for 3.0000 s
generate_prime/2048 bits
                        time:   [1.2667 s 1.4846 s 1.7265 s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking generate_prime/4096 bits: Warming up for 3.0000 s
generate_prime/4096 bits
                        time:   [17.369 s 20.613 s 24.142 s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

```
## References
- https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm
- https://www.di-mgt.com.au/rsa_alg.html
- https://www.di-mgt.com.au/rsa_theory.html
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
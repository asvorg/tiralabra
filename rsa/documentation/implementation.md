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

### GCD benchmark

This function is of constant time complexity

```
gcd/128 bits            time:   [2.1188 µs 2.1240 µs 2.1292 µs]
                        change: [+2.2515% +2.9614% +3.6870%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

gcd/256 bits            time:   [2.1568 µs 2.1609 µs 2.1653 µs]
                        change: [+4.0514% +4.3679% +4.6694%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild

gcd/512 bits            time:   [2.1541 µs 2.1587 µs 2.1632 µs]
                        change: [+3.7085% +4.0567% +4.4842%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

gcd/1024 bits           time:   [2.1758 µs 2.1904 µs 2.2144 µs]
                        change: [-7.5594% -2.6358% +1.0790%] (p = 0.31 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

gcd/2048 bits           time:   [2.1364 µs 2.1405 µs 2.1449 µs]
                        change: [-1.3862% +0.6269% +2.2080%] (p = 0.56 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

gcd/4096 bits           time:   [2.1271 µs 2.1314 µs 2.1357 µs]
                        change: [-5.2932% -1.3633% +1.1858%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

gcd/8192 bits           time:   [2.1152 µs 2.1192 µs 2.1234 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

```

## References
- https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm
- https://www.di-mgt.com.au/rsa_alg.html
- https://www.di-mgt.com.au/rsa_theory.html
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://www.geeksforgeeks.org/rsa-algorithm-cryptography/
- https://www.youtube.com/watch?v=aJ9HAdiAnIU&t=170s d calculation
- https://brilliant.org/wiki/extended-euclidean-algorithm/
- https://crypto.stackexchange.com/questions/5889/calculating-rsa-private-exponent-when-given-public-exponent-and-the-modulus-fact
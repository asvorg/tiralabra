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

### Extended Euclidian algorithm benchmark

This function is also of constant time complexity

```
extended_euclidean_algorithm/128 bits
                        time:   [6.2085 µs 6.3033 µs 6.4137 µs]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

extended_euclidean_algorithm/256 bits
                        time:   [6.9914 µs 7.0603 µs 7.1565 µs]
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe

extended_euclidean_algorithm/512 bits
                        time:   [6.0830 µs 6.1552 µs 6.2865 µs]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

extended_euclidean_algorithm/1024 bits
                        time:   [6.3831 µs 6.4072 µs 6.4342 µs]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

extended_euclidean_algorithm/2048 bits
                        time:   [5.9280 µs 5.9366 µs 5.9459 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
  
extended_euclidean_algorithm/4096 bits
                        time:   [5.9985 µs 6.0705 µs 6.1543 µs]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
```

### Encryption and decryption benchmark
```
encrypt_and_decrypt/128 bits
                        time:   [2.1113 ms 2.1350 ms 2.1591 ms]

encrypt_and_decrypt/256 bits
                        time:   [5.9742 ms 6.0967 ms 6.2209 ms]
                        
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encrypt_and_decrypt/512 bits
                        time:   [30.652 ms 32.320 ms 34.030 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking encrypt_and_decrypt/1024 bits: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 24.3s, or reduce sample count to 20.
encrypt_and_decrypt/1024 bits
                        time:   [246.70 ms 272.24 ms 300.10 ms]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking encrypt_and_decrypt/2048 bits: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 420.2s, or reduce sample count to 10.
encrypt_and_decrypt/2048 bits
                        time:   [2.8697 s 3.2161 s 3.5833 s]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Benchmarking encrypt_and_decrypt/4096 bits: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 4653.3s, or reduce sample count to 10.
encrypt_and_decrypt/4096 bits
                        time:   [37.329 s 42.513 s 48.030 s]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```

## References
- https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm
- https://www.di-mgt.com.au/rsa_alg.html
- https://www.di-mgt.com.au/rsa_theory.html
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://www.geeksforgeeks.org/rsa-algorithm-cryptography/
- https://brilliant.org/wiki/extended-euclidean-algorithm/
- https://crypto.stackexchange.com/questions/5889/calculating-rsa-private-exponent-when-given-public-exponent-and-the-modulus-fact
- https://en.wikipedia.org/wiki/Miller–Rabin_primality_test
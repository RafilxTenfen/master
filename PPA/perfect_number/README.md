# Perfect Numbers Generator

- Perfect number, is a positive integer that is equal to the sum of its proper divisors. The smallest perfect number is 6, which is the sum of 1, 2, and 3. Other perfect numbers are 28, 496, and 8.128.

### How to build
- Just run `make`
- This will generate a binary called 'generate'

### How to run

#### Brute Force
- The brute force strategy tries to divide each predecessor number and then sum all possible divisors to check if it's equal to the current number
- Inform the binary the max number that you want to check for a perfect number and the method brute force `bf`
```shell
$~ ./generate 4 bf
```

__Expected Result__
```shell
Perfect numbers to be generated: 9000
Perfect Numbers: 6 28 496 8128
It took 0.117603 seconds to find 4 perfect numbers
```

#### Euclid
- The Euclid strategy uses prime and Mersenne prime numbers to verify if a number is perfect
  - PerfectNumber = 2p-1(2p -1) where p is a prime for which 2p -1 is a also a prime number.

- Inform the binary the numbers that you want to generate and the default method is the Euclid
```shell
$~ ./generate 8
```
__Expected Result__
```shell
Perfect numbers to be generated: 2305843008139952129
Perfect Numbers: 6 28 496 8128 33550336 8589869056 137438691328 2305843008139952128
It took 22.748743 seconds to find 8 perfect numbers using Euclid
```

#### Issues
- In my machine the brute force strategy generate the max of 4 numbers

- The Euclid strategy took 40 seconds to generate 9 perfect numbers, but the `unsigned long` C type isn't big enough to print the value, ex.:
```shell
Perfect numbers to be generated: 999999999999999999999999999
Perfect Numbers: 6 28 496 8128 33550336 8589869056 137438691328 2305843008139952128 -1152921504606846976
It took 40.524437 seconds to find 9 perfect numbers using Euclid
```
> The received value for the 9'th perfect number should be 2658455991569831744654692615953842176 instead of -1152921504606846976

- Computer Specs

![image](https://user-images.githubusercontent.com/17556614/93689603-8f111d00-faa6-11ea-89a4-10e42fef614f.png)

# Hardware information
- `hardware_info.txt` ens5 LABP2D machine

# 2D Line Graphs
- `images` directory

# OpenMP implementation using brute force
- `main.c` commented to be able to generate `csv` data

# CSV generator
- `generateexcel.sh`

# Python code to generate line graphs
- `plot_graph.py.ipynb`
- type `jupyter notebook` and run
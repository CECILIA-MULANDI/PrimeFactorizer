#PrimeFinder    
1.find_prime_factors

    This function takes a positive integer `n` and returns a vector containing its prime factors. It iteratively tests each integer starting from 2 to determine if it divides `n` evenly. If a divisor is found, it is added to the result vector, and `n` is divided by this divisor. This process continues until the square of the divisor exceeds `n`. The remaining value of `n`, which is a prime number, is added to the result vector.

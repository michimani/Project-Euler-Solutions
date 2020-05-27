//
// Solution for Project Euler problem 10
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=10
//
package main

import "fmt"

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var primes []int = []int{}
	var isPrime bool = true

	for num := 2; num < 2000000; num++ {
		isPrime = true

		for _, prime := range primes {
			if num%prime == 0 {
				isPrime = false
				break
			}
		}

		if isPrime == true {
			primes = append(primes, num)
			answer = answer + num
		}
	}

	return answer
}

func main() {
	fmt.Println(Compute())
}

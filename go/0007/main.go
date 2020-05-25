//
// Solution for Project Euler problem 7
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=7
//
package main

import "fmt"

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var primes []int = []int{}
	var isPrime bool = true
	var num int = 2

	for len(primes) < 10002 {
		isPrime = true

		for _, prime := range primes {
			if num%prime == 0 {
				isPrime = false
				break
			}

		}

		if isPrime == true {
			primes = append(primes, num)
		}

		num++
	}

	answer = primes[10000]
	return answer
}

func main() {
	fmt.Println(Compute())
}

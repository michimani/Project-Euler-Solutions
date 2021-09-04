//
// Solution for Project Euler problem 7
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=7
//
package solution

import "fmt"

// Solve0007 is function to solve the problem.
func Solve0007() {
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

		if isPrime {
			primes = append(primes, num)
		}

		num++
	}

	answer = primes[10000]
	fmt.Println(answer)
}

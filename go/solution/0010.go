// Solution for Project Euler problem 10
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=10
package solution

import (
	"fmt"
	"math"
)

// Solve0010 is function to solve the problem.
func Solve0010() {
	var answer int

	for num := 2; num < 2000000; num++ {
		isPrime := true
		for d := 2; d < int(math.Sqrt(float64(num))+1); d++ {
			if num%d == 0 {
				isPrime = false
				break
			}
		}

		if isPrime {
			answer = answer + num
		}
	}

	fmt.Println(answer)
}

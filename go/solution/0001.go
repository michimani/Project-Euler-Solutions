//
// Solution for Project Euler problem 1
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=1
//
package solution

import (
	"fmt"
)

// Solve0001 is function to solve the problem.
func Solve0001() {
	var answer int = 0
	for num := 1; num < 1000; num++ {
		if num%3 == 0 || num%5 == 0 {
			answer = answer + num
		}
	}

	fmt.Println(answer)
}

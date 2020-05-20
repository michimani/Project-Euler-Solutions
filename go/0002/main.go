//
// Solution for Project Euler problem 2
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=2
//
package main

import (
	"fmt"
)

// Compute is function to solve the problem.
func Compute() int {
	var answer int = 2
	var base1 int = 1
	var base2 int = 2
	var addition int
	var limit int = 4000000

	for base2 < limit {
		addition = base1 + base2
		if addition%2 == 0 {
			answer = answer + addition
		}

		base1 = base2
		base2 = addition
	}

	return answer
}

func main() {
	fmt.Println(Compute())
}

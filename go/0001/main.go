//
// Solution for Project Euler problem 1
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=1
//
package main

import (
	"fmt"
)

// Compute is function to solve the problem.
func Compute() int {
	var answer int = 0
	for num := 1; num < 1000; num++ {
		if num%3 == 0 || num%5 == 0 {
			answer = answer + num
		}
	}

	return answer
}

func main() {
	fmt.Println(Compute())
}

//
// Solution for Project Euler problem 6
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=6
//
package main

import "fmt"

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var max = 100

	for n := 1; n <= max; n++ {
		for m := 1; m <= max; m++ {
			if n != m {
				answer = answer + (n * m)
			}
		}
	}

	return answer
}

func main() {
	fmt.Println(Compute())
}

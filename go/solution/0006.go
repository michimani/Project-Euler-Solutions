//
// Solution for Project Euler problem 6
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=6
//
package solution

import "fmt"

// Solve0006 is function to solve the problem.
func Solve0006() {
	var answer int
	var max = 100

	for n := 1; n <= max; n++ {
		for m := 1; m <= max; m++ {
			if n != m {
				answer = answer + (n * m)
			}
		}
	}

	fmt.Println(answer)
}

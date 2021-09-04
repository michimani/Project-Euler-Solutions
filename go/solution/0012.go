//
// Solution for Project Euler problem 12
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=12
//
package solution

import "fmt"

// Solve0012 is function to solve the problem.
func Solve0012() {
	var answer int
	var tri int = 1

	for n := 2; true; n++ {
		tri = tri + n
		divisors := getDivisors(tri)
		if len(divisors) > 500 {
			answer = tri
			break
		}
	}

	fmt.Println(answer)
}

// getDivisors is function to get list of divisors of the number.
func getDivisors(num int) []int {
	var divisors []int = []int{}
	var quot int

	for n := 1; n < num; n++ {
		if num%n == 0 {
			quot = num / n
			if quot <= n {
				break
			}

			divisors = append(divisors, n)
			divisors = append(divisors, quot)
		}
	}

	return divisors
}

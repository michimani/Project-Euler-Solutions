//
// Solution for Project Euler problem 12
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=12
//
package main

import "fmt"

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var tri int = 1

	for n := 2; true; n++ {
		tri = tri + n
		divisors := GetDivisors(tri)
		if len(divisors) > 500 {
			answer = tri
			break
		}
	}

	return answer
}

// GetDivisors is function to get list of divisors of the number.
func GetDivisors(num int) []int {
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

func main() {
	fmt.Println(Compute())
}

//
// Solution for Project Euler problem 12
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=12
//
package solution

import (
	"fmt"

	"github.com/michimani/project-euler-go/util"
)

// Solve0012 is function to solve the problem.
func Solve0012() {
	var answer int
	var tri int = 1

	for n := 2; true; n++ {
		tri = tri + n
		divisors := util.Divisors(tri)
		if len(divisors) > 500 {
			answer = tri
			break
		}
	}

	fmt.Println(answer)
}

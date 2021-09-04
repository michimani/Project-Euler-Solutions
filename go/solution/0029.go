//
// Solution for Project Euler problem 29
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=29
//
package solution

import (
	"fmt"
	"math"
	"math/big"
)

// Solve0029 is function to solve the problem.
func Solve0029() {
	var answer int
	var res map[string]struct{} = map[string]struct{}{}
	from := 2
	to := 100

	for a := from; a <= to; a++ {
		for b := from; b <= to; b++ {
			r := big.NewFloat(math.Pow(float64(a), float64(b))).String()
			res[r] = struct{}{}
		}
	}

	answer = len(res)
	fmt.Println(answer)
}

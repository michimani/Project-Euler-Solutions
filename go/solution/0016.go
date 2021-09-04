//
// Solution for Project Euler problem 16
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=16
//
package solution

import (
	"fmt"
	"math/big"
	"strconv"
)

// Solve0016 is function to solve the problem.
func Solve0016() {
	var answer int
	num := big.NewInt(1)
	for n := 0; n < 1000; n++ {
		num = new(big.Int).Mul(num, big.NewInt(2))
	}
	numStr := num.String()

	for s := 0; s < len(numStr); s++ {
		add, _ := strconv.Atoi(numStr[s : s+1])
		answer = answer + add
	}
	fmt.Println(answer)
}

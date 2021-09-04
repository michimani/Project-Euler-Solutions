//
// Solution for Project Euler problem 20
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=20
//
package solution

import (
	"fmt"
	"math/big"
	"strconv"
)

// Solve0020 is function to solve the problem.
func Solve0020() {
	var answer int
	num := big.NewInt(1)

	for n := 1; n <= 100; n++ {
		num = new(big.Int).Mul(num, big.NewInt(int64(n)))
	}

	var numStr string = num.String()
	for m := 0; m < len(numStr); m++ {
		mInt, _ := strconv.Atoi(numStr[m : m+1])
		answer = answer + mInt
	}

	fmt.Println(answer)
}

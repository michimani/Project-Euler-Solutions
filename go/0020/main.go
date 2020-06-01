//
// Solution for Project Euler problem 20
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=20
//
package main

import (
	"fmt"
	"math/big"
	"strconv"
)

// Compute is function to solve the problem.
func Compute() int {
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

	return answer
}

func main() {
	fmt.Println(Compute())
}

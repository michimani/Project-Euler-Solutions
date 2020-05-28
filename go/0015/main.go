//
// Solution for Project Euler problem 15
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=15
//
package main

import (
	"fmt"
	"math/big"
)

// Compute is function to solve the problem.
func Compute() *big.Int {
	var answer *big.Int
	var up int = 20
	var down int = 20
	deno := big.NewInt(1)
	nume := big.NewInt(1)

	for n := up + 1; n <= (up + down); n++ {
		nume = new(big.Int).Mul(nume, big.NewInt(int64(n)))
	}

	for m := 1; m <= down; m++ {
		deno = new(big.Int).Mul(deno, big.NewInt(int64(m)))
	}

	answer = new(big.Int).Div(nume, deno)

	return answer
}

func main() {
	fmt.Println(Compute())
}

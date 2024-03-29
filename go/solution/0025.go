//
// Solution for Project Euler problem 25
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=25
//
package solution

import (
	"fmt"
	"math/big"
)

// Solve0025 is function to solve the problem.
func Solve0025() {
	var answer string
	seqNo := big.NewInt(2)
	prepre := big.NewInt(1)
	pre := big.NewInt(1)
	tmp := big.NewInt(0)

	for len(tmp.String()) < 1000 {
		seqNo = new(big.Int).Add(seqNo, big.NewInt(1))
		tmp = new(big.Int).Add(prepre, pre)
		prepre = pre
		pre = tmp
	}

	answer = seqNo.String()

	fmt.Println(answer)
}

//
// Solution for Project Euler problem 23
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=23
//
package solution

import (
	"fmt"

	"github.com/michimani/project-euler-go/util"
)

const LIMIT int = 30000

// Solve0023 is function to solve the problem.
func Solve0023() {
	var answer int
	abundantNumbers := map[int]struct{}{}

	fmt.Println()
	for n := 1; n <= LIMIT; n++ {
		if isAbundantNumber(n) {
			abundantNumbers[n] = struct{}{}
		}
	}

	sumOfTwoAbundantNumbers := map[int]struct{}{}
	for an1 := range abundantNumbers {
		for an2 := range abundantNumbers {
			s := an1 + an2
			sumOfTwoAbundantNumbers[s] = struct{}{}
		}
	}

	for m := 1; m <= LIMIT; m++ {
		if _, ok := sumOfTwoAbundantNumbers[m]; !ok {
			answer = answer + m
		}
	}

	fmt.Println(answer)
}

func isAbundantNumber(num int) bool {
	propDivs := util.ProperDivisors(num)
	sum := util.Sum(propDivs)

	return sum > num
}

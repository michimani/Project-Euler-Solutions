//
// Solution for Project Euler problem 4
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=4
//
package solution

import (
	"fmt"

	"github.com/michimani/project-euler-go/util"
)

// Solve0004 is function to solve the problem.
func Solve0004() {
	var answer int
	var palindomes []int = []int{}

	for a := 100; a < 1000; a++ {
		for b := 100; b < 1000; b++ {
			times := a * b
			isP, err := util.IsPalindrome(times)
			if err != nil {
				fmt.Println(err.Error())
				return
			}

			if isP {
				palindomes = append(palindomes, times)
			}
		}
	}

	var palindomeTmp int = 0
	for _, p := range palindomes {
		if p > palindomeTmp {
			palindomeTmp = p
		}
	}

	answer = palindomeTmp

	fmt.Println(answer)
}

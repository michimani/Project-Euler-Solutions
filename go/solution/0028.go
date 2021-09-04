//
// Solution for Project Euler problem 28
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=28
//
package solution

import (
	"fmt"
	"math"
)

// Solve0028 is function to solve the problem.
func Solve0028() {
	answer := 1
	size := 1001
	around := int(math.Floor(float64(size) / float64(2)))

	for a := 1; a <= around; a++ {
		answer = answer + sumOfEdgeNums(a)
	}

	fmt.Println(answer)
}

func sumOfEdgeNums(around int) int {
	s := 0
	size := around*2 + 1
	maxEdgeNum := size * size
	s = maxEdgeNum
	for n := 1; n < 4; n++ {
		s = s + (maxEdgeNum - ((size - 1) * n))
	}

	return s
}

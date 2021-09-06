//
// Solution for Project Euler problem 9
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=9
//
package solution

import (
	"fmt"
	"math"
)

// Solve0009 is function to solve the problem.
func Solve0009() {
	var answer int
	a := 1
	b := 2
	maxC := 1000 - b - a
	for tmpA := 1; tmpA < int(math.Floor(float64(maxC)/2)); tmpA++ {
		for tmpB := tmpA + 1; tmpB < int(math.Floor(float64(maxC)/2)); tmpB++ {
			tmpC := 1000 - tmpB - tmpA
			if math.Pow(float64(tmpC), 2) == math.Pow(float64(tmpA), 2)+math.Pow(float64(tmpB), 2) {
				answer = tmpA * tmpB * tmpC
				break
			}
		}
	}

	fmt.Println(answer)
}

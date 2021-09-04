//
// Solution for Project Euler problem 30
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=30
//
package solution

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

// Solve0030 is function to solve the problem.
func Solve0030() {
	var answer int
	pow := 5
	maxOfADigit := int(math.Pow(9, float64(pow)))
	limit := 1
	d := 1
	for {
		limit = int(math.Pow10(d))
		if maxOfADigit*d < limit {
			break
		}
		d = d + 1
	}

	for i := 2; i < limit; i++ {
		sum, err := sumOfEachDigit(i, pow)
		if err != nil {
			fmt.Println(err.Error())
			return
		}

		if sum == i {
			answer = answer + i
		}
	}

	fmt.Println(answer)
}

func sumOfEachDigit(num, pow int) (int, error) {
	s := 0
	digits := strings.Split(strconv.Itoa(num), "")

	for _, d := range digits {
		n, err := strconv.Atoi(d)
		if err != nil {
			return 0, err
		}
		s = s + int(math.Pow(float64(n), float64(pow)))
	}

	return s, nil
}

package util

import "math"

func IsSquareOfNatureNumber(square int) bool {
	if square <= 0 {
		return false
	}

	return square == int(math.Pow(math.Floor(math.Pow(float64(square), 0.5)), 2))
}

// Divisors returns divisors of the number.
func Divisors(num int) []int {
	if num <= 0 {
		return []int{}
	}
	if num == 1 {
		return []int{1}
	}

	var divisors []int = []int{}
	var quot int
	for n := 1; n < num; n++ {
		if num%n == 0 {
			quot = num / n
			if quot <= n {
				break
			}

			divisors = append(divisors, n)
			divisors = append(divisors, quot)
		}
	}

	return divisors
}
